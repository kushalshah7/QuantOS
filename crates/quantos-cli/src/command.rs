use quantos_core::data::{
    CsvParseResultV1, NormalizationResultV1, NormalizationStatusV1, normalize_v1, parse_csv_v1,
};
use serde::Serialize;
use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

const CONFIG_LIMIT: u64 = 1_048_576;
const CSV_LIMIT: u64 = 67_108_864;
const HELP: &str = "QuantOS CSV import\nUsage:\n  quantos-cli\n  quantos-cli --help\n  quantos-cli import --csv <path> --config <path>\n";

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum Code {
    Usage,
    ConfigJson,
    ConfigEnvelope,
    ConfigTooLarge,
    ConfigRead,
    CsvRead,
    CsvTooLarge,
}
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Field {
    Args,
    Config,
    Csv,
}
#[derive(Serialize)]
pub(super) struct Issue {
    code: Code,
    field: Field,
    message: &'static str,
}
#[derive(Serialize)]
#[serde(tag = "stage", rename_all = "lowercase")]
pub(super) enum Report {
    Normalization {
        schema_version: &'static str,
        status: NormalizationStatusV1,
        result: NormalizationResultV1,
    },
    Csv {
        schema_version: &'static str,
        status: NormalizationStatusV1,
        result: CsvParseResultV1,
    },
    Cli {
        schema_version: &'static str,
        status: NormalizationStatusV1,
        issue: Issue,
    },
}
pub(super) enum Output {
    Text(Vec<u8>),
    Report(Report),
}
pub(super) struct Outcome {
    pub(super) output: Output,
    pub(super) code: u8,
}

fn operational(code: Code) -> Outcome {
    let (field, exit, message) = match code {
        Code::Usage => (Field::Args, 2, "Use import --csv <path> --config <path>."),
        Code::ConfigJson => (
            Field::Config,
            2,
            "Configuration must be UTF-8 JSON without BOM or duplicate keys.",
        ),
        Code::ConfigEnvelope => (
            Field::Config,
            2,
            "Configuration must have exactly schema_version, metadata and options.",
        ),
        Code::ConfigTooLarge => (Field::Config, 2, "Configuration exceeds the byte limit."),
        Code::ConfigRead => (Field::Config, 3, "Cannot read configuration file."),
        Code::CsvRead => (Field::Csv, 3, "Cannot read CSV file."),
        Code::CsvTooLarge => (Field::Csv, 3, "CSV exceeds the byte limit."),
    };
    Outcome {
        code: exit,
        output: Output::Report(Report::Cli {
            schema_version: "1",
            status: NormalizationStatusV1::Failure,
            issue: Issue {
                code,
                field,
                message,
            },
        }),
    }
}

fn arguments(args: &[OsString]) -> Option<(PathBuf, PathBuf)> {
    if args.len() != 5 || args[0] != "import" {
        return None;
    }
    let (mut csv, mut config) = (None, None);
    for pair in args[1..].chunks_exact(2) {
        let value = &pair[1];
        if value.is_empty() || value == "-" || value.as_encoded_bytes().starts_with(b"--") {
            return None;
        }
        if pair[0] == "--csv" && csv.is_none() {
            csv = Some(PathBuf::from(value));
        } else if pair[0] == "--config" && config.is_none() {
            config = Some(PathBuf::from(value));
        } else {
            return None;
        }
    }
    csv.zip(config)
}

enum ReadFailure {
    Read,
    TooLarge,
}
fn bounded(path: &Path, limit: u64) -> Result<Vec<u8>, ReadFailure> {
    let file = File::open(path).map_err(|_| ReadFailure::Read)?;
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ReadFailure::Read)?;
    if bytes.len() as u64 > limit {
        Err(ReadFailure::TooLarge)
    } else {
        Ok(bytes)
    }
}

pub(super) fn run(args: Vec<OsString>) -> Outcome {
    if args.is_empty() {
        return Outcome {
            code: 0,
            output: Output::Text(format!("{}\n", quantos_core::foundation_status()).into_bytes()),
        };
    }
    if args.len() == 1 && args[0] == "--help" {
        return Outcome {
            code: 0,
            output: Output::Text(HELP.as_bytes().to_vec()),
        };
    }
    let Some((csv_path, config_path)) = arguments(&args) else {
        return operational(Code::Usage);
    };
    let bytes = match bounded(&config_path, CONFIG_LIMIT) {
        Ok(bytes) => bytes,
        Err(ReadFailure::Read) => return operational(Code::ConfigRead),
        Err(ReadFailure::TooLarge) => return operational(Code::ConfigTooLarge),
    };
    let config = match super::config::parse(&bytes) {
        Ok(value) => value,
        Err(()) => return operational(Code::ConfigJson),
    };
    let Some(envelope) = config.as_object().filter(|m| {
        m.len() == 3
            && ["schema_version", "metadata", "options"]
                .iter()
                .all(|key| m.contains_key(*key))
    }) else {
        return operational(Code::ConfigEnvelope);
    };
    let bytes = match bounded(&csv_path, CSV_LIMIT) {
        Ok(bytes) => bytes,
        Err(ReadFailure::Read) => return operational(Code::CsvRead),
        Err(ReadFailure::TooLarge) => return operational(Code::CsvTooLarge),
    };
    match parse_csv_v1(&bytes) {
        CsvParseResultV1::Success { headers, rows, .. } => {
            let request: Value = json!({"schema_version":envelope["schema_version"],"metadata":envelope["metadata"],"options":envelope["options"],"headers":headers,"rows":rows});
            let result = normalize_v1(&request);
            let code = if result.status == NormalizationStatusV1::Success {
                0
            } else {
                1
            };
            Outcome {
                code,
                output: Output::Report(Report::Normalization {
                    schema_version: "1",
                    status: result.status.clone(),
                    result,
                }),
            }
        }
        result @ CsvParseResultV1::Failure { .. } => Outcome {
            code: 1,
            output: Output::Report(Report::Csv {
                schema_version: "1",
                status: NormalizationStatusV1::Failure,
                result,
            }),
        },
    }
}
