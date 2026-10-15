use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

pub(super) const VALID: &[u8] = include_bytes!("../../../../fixtures/synthetic/valid-candles.csv");
pub(super) const BAD_OHLC: &[u8] =
    include_bytes!("../../../../fixtures/synthetic/invalid-ohlc.csv");
pub(super) const DUPLICATE: &[u8] =
    include_bytes!("../../../../fixtures/synthetic/duplicate-timestamp.csv");
pub(super) const NAIVE: &[u8] =
    include_bytes!("../../../../fixtures/synthetic/missing-timezone.csv");
pub(super) const CONFIG: &[u8] =
    include_bytes!("../../../../fixtures/synthetic/import-config.json");
static NEXT: AtomicU64 = AtomicU64::new(0);

pub(super) struct Sandbox {
    pub(super) path: PathBuf,
    base: PathBuf,
}
impl Sandbox {
    pub(super) fn new() -> Self {
        let base = fs::canonicalize(std::env::temp_dir()).unwrap();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        for _ in 0..100 {
            let path = base.join(format!(
                "quantos-import-owned-{}-{stamp}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    assert_eq!(
                        fs::canonicalize(&path).unwrap().parent(),
                        Some(base.as_path())
                    );
                    return Self { path, base };
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => panic!("test directory: {e}"),
            }
        }
        panic!("cannot allocate test-owned directory")
    }
    pub(super) fn write(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.path.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }
    pub(super) fn config(&self, value: &Value) -> PathBuf {
        self.write("import config.json", &serde_json::to_vec(value).unwrap())
    }
    pub(super) fn spawn(&self, args: &[OsString]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_quantos-cli"))
            .current_dir(&self.path)
            .args(args)
            .output()
            .expect("actual CLI must start")
    }
    pub(super) fn import(&self, csv: &Path, config: &Path) -> Output {
        self.spawn(&args(csv, config))
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        // Remove only this instance's created, resolved direct child of temp.
        if let Ok(resolved) = fs::canonicalize(&self.path)
            && resolved == self.path
            && resolved.parent() == Some(self.base.as_path())
            && resolved
                .file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("quantos-import-owned-"))
        {
            let _ = fs::remove_dir_all(&resolved);
        }
    }
}
pub(super) fn args(csv: &Path, config: &Path) -> Vec<OsString> {
    vec![
        "import".into(),
        "--csv".into(),
        csv.as_os_str().to_owned(),
        "--config".into(),
        config.as_os_str().to_owned(),
    ]
}
pub(super) fn config() -> Value {
    serde_json::from_slice(CONFIG).unwrap()
}
pub(super) fn metadata() -> Value {
    json!({"source":"synthetic","synthetic":true,"adjustment_status":"unknown","redistribution_status":"unknown"})
}
pub(super) fn candles() -> Vec<Value> {
    vec![
        json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":0,"open":10000,"high":10300,"low":9900,"close":10200,"volume":10,"source":"synthetic"}),
        json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":300000,"open":10200,"high":10400,"low":10100,"close":10300,"volume":12,"source":"synthetic"}),
        json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":600000,"open":10300,"high":10500,"low":10000,"close":10100,"volume":8,"source":"synthetic"}),
    ]
}
pub(super) fn issue(code: &str, row: Option<u64>, field: Option<&str>) -> Value {
    json!({"code":code,"severity":"error","row":row,"field":field})
}
pub(super) fn normalized(candles: Vec<Value>, issues: Vec<Value>, valid: bool) -> Value {
    let status = if valid { "success" } else { "failure" };
    json!({"schema_version":"1","status":status,"stage":"normalization","result":{"schema_version":"1","status":status,"metadata":metadata(),"candles":candles,"issues":issues,"gap_assessment":{"status":"not_performed","missing_ts_utc_ms":[]}}})
}
pub(super) fn cli(code: &str, field: &str) -> Value {
    json!({"schema_version":"1","status":"failure","stage":"cli","issue":{"code":code,"field":field}})
}
pub(super) fn csv_error(code: &str, row: Option<u64>, offset: usize) -> Value {
    json!({"schema_version":"1","status":"failure","stage":"csv","result":{"schema_version":"1","status":"failure","issue":{"code":code,"row":row,"byte_offset":offset}}})
}
fn strip_message(issue: &mut Value, keys: &[&str]) {
    let map = issue.as_object_mut().unwrap();
    assert_eq!(
        map.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        keys.iter().copied().collect()
    );
    assert!(
        map["message"]
            .as_str()
            .is_some_and(|s| !s.trim().is_empty())
    );
    map.remove("message");
}
pub(super) fn check(id: &str, output: Output, code: i32, expected: Value) {
    assert_eq!(
        output.status.code(),
        Some(code),
        "{id}: exit; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "{id}: stderr");
    assert_eq!(output.stdout.last(), Some(&b'\n'), "{id}: final newline");
    assert_eq!(
        output.stdout.iter().filter(|b| **b == b'\n').count(),
        1,
        "{id}: one JSON line"
    );
    let text = std::str::from_utf8(&output.stdout).unwrap();
    let (mut quoted, mut escape) = (false, false);
    for b in output.stdout[..output.stdout.len() - 1].iter().copied() {
        if quoted {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                quoted = false;
            }
        } else if b == b'"' {
            quoted = true;
        } else {
            assert!(!b.is_ascii_whitespace(), "{id}: compact JSON");
        }
    }
    let mut actual: Value = serde_json::from_str(text).unwrap();
    assert!(actual.is_object());
    match actual["stage"].as_str().unwrap() {
        "cli" => strip_message(&mut actual["issue"], &["code", "field", "message"]),
        "csv" => strip_message(
            &mut actual["result"]["issue"],
            &["code", "row", "byte_offset", "message"],
        ),
        "normalization" => {
            for issue in actual["result"]["issues"].as_array_mut().unwrap() {
                strip_message(issue, &["code", "severity", "row", "field", "message"]);
            }
        }
        _ => panic!("unexpected stage"),
    }
    assert_eq!(
        actual, expected,
        "{id}: entire report and ordered issue keys"
    );
}
