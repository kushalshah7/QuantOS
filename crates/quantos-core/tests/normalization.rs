#[path = "normalization_cases/edges.rs"]
mod edges;
#[path = "normalization_cases/golden.rs"]
mod golden;
#[path = "normalization_cases/support.rs"]
mod support;

use quantos_core::data::normalize_v1;
use serde_json::json;
use std::process::Command;

#[test]
fn timezone_child() {
    if std::env::var_os("QUANTOS_NORMALIZATION_TZ_CHILD").is_none() {
        return;
    }
    let outputs = [
        normalize_v1(&support::r()),
        normalize_v1(&support::t("1970-01-01T09:15:00", Some("Asia/Kolkata"))),
        normalize_v1(&support::t("1970-01-01T09:15:00", None)),
    ];
    println!(
        "QUANTOS_TZ_RESULT={}",
        serde_json::to_string(&outputs).unwrap()
    );
}

#[test]
fn host_timezone_independence_in_isolated_subprocesses() {
    let mut results = Vec::new();
    for timezone in ["UTC", "Asia/Kolkata"] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "timezone_child", "--nocapture"])
            .env("QUANTOS_NORMALIZATION_TZ_CHILD", "1")
            .env("TZ", timezone)
            .output()
            .expect("isolated normalization test must start");
        assert!(
            output.status.success(),
            "{timezone}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        let json = stdout
            .lines()
            .find_map(|line| {
                line.split_once("QUANTOS_TZ_RESULT=")
                    .map(|(_, value)| value)
            })
            .expect("child result marker");
        let result: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(result[0]["status"], "success");
        assert_eq!(result[1]["candles"][0]["ts_utc_ms"], json!(13500000));
        assert_eq!(result[2]["issues"][0]["code"], "TIMEZONE_REQUIRED");
        results.push(result);
    }
    assert_eq!(
        results[0], results[1],
        "full values, including messages, must be TZ-independent"
    );
}
