use super::command::{Outcome, Output};
use std::io::{self, Write};

const DIAGNOSTIC: &[u8] = b"quantos-cli: output failure\n";

fn write_complete(writer: &mut impl Write, mut bytes: &[u8]) -> io::Result<()> {
    while !bytes.is_empty() {
        match writer.write(bytes) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::WriteZero)),
            Ok(count) => bytes = &bytes[count..],
            Err(error) => return Err(error), // including Interrupted: do not retry errors
        }
    }
    writer.flush()
}

pub(super) fn emit(out: &mut impl Write, err: &mut impl Write, outcome: Outcome) -> u8 {
    let bytes = match outcome.output {
        Output::Text(bytes) => Ok(bytes),
        Output::Report(report) => serde_json::to_vec(&report).map(|mut bytes| {
            bytes.push(b'\n');
            bytes
        }),
    };
    if let Ok(bytes) = bytes
        && write_complete(out, &bytes).is_ok()
    {
        return outcome.code;
    }
    let _ = write_complete(err, DIAGNOSTIC);
    3
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Broken {
        bytes: Vec<u8>,
        calls: usize,
        flushes: usize,
        mode: u8,
    }
    impl Write for Broken {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            match self.mode {
                0 => Err(io::Error::from(io::ErrorKind::BrokenPipe)),
                1 if self.calls > 1 => Err(io::Error::from(io::ErrorKind::BrokenPipe)),
                2 => Ok(0),
                3 => Err(io::Error::from(io::ErrorKind::Interrupted)),
                _ => {
                    let count = if self.mode == 1 { 1 } else { bytes.len() };
                    self.bytes.extend_from_slice(&bytes[..count]);
                    Ok(count)
                }
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.mode == 4 {
                Err(io::Error::from(io::ErrorKind::BrokenPipe))
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn injected_stdout_failures_are_sanitized_and_never_retried() {
        for mode in 0..=4 {
            let mut out = Broken {
                bytes: Vec::new(),
                calls: 0,
                flushes: 0,
                mode,
            };
            let mut err = Vec::new();
            let outcome = Outcome {
                code: 0,
                output: Output::Text(b"SECRET OUTPUT\n".to_vec()),
            };
            assert_eq!(emit(&mut out, &mut err, outcome), 3);
            assert_eq!(err, b"quantos-cli: output failure\n");
            assert_eq!(out.calls, if mode == 1 { 2 } else { 1 });
            assert_eq!(out.flushes, usize::from(mode == 4));
            if mode == 1 {
                assert_eq!(out.bytes, b"S");
            }
        }
    }
    #[test]
    fn successful_output_is_complete_and_flushed_once() {
        let mut out = Broken {
            bytes: Vec::new(),
            calls: 0,
            flushes: 0,
            mode: 5,
        };
        let mut err = Vec::new();
        assert_eq!(
            emit(
                &mut out,
                &mut err,
                super::super::command::run(vec!["unknown".into()])
            ),
            2
        );
        assert_eq!(out.flushes, 1);
        assert!(err.is_empty());
        assert_eq!(out.bytes.iter().filter(|b| **b == b'\n').count(), 1);
        let value: serde_json::Value = serde_json::from_slice(&out.bytes).unwrap();
        assert_eq!(value["issue"]["code"], "USAGE");
    }
}
