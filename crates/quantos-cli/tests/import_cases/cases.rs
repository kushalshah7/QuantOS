use super::support::*;
use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn valid_fixture_both_flag_orders_spaced_and_relative_paths() {
    let d = Sandbox::new();
    let csv = d.write("prices sample.csv", VALID);
    let cfg = d.write("config sample.json", CONFIG);
    check(
        "valid",
        d.import(&csv, &cfg),
        0,
        normalized(candles(), vec![], true),
    );
    check(
        "flags reversed",
        d.spawn(&[
            "import".into(),
            "--config".into(),
            cfg.into_os_string(),
            "--csv".into(),
            csv.into_os_string(),
        ]),
        0,
        normalized(candles(), vec![], true),
    );
    check(
        "relative paths",
        d.spawn(&[
            "import".into(),
            "--csv".into(),
            "prices sample.csv".into(),
            "--config".into(),
            "config sample.json".into(),
        ]),
        0,
        normalized(candles(), vec![], true),
    );
}
#[test]
fn atomic_domain_failures_and_explicit_kolkata_confirmation() {
    let d = Sandbox::new();
    let cfg = d.write("cfg.json", CONFIG);
    for (name, bytes, code, row, field) in [
        ("ohlc", BAD_OHLC, "INVALID_OHLC", 2, None),
        (
            "dup",
            DUPLICATE,
            "DUPLICATE_TIMESTAMP",
            3,
            Some("ts_utc_ms"),
        ),
    ] {
        let csv = d.write(name, bytes);
        check(
            name,
            d.import(&csv, &cfg),
            1,
            normalized(vec![], vec![issue(code, Some(row), field)], false),
        );
    }
    let csv = d.write("naive.csv", NAIVE);
    let mut c = config();
    c["options"]["timestamp_column"] = json!("timestamp");
    c["options"]["timestamp_format"] = json!("iso8601");
    let cfg = d.config(&c);
    check(
        "timezone required",
        d.import(&csv, &cfg),
        1,
        normalized(
            vec![],
            vec![issue("TIMEZONE_REQUIRED", Some(2), Some("timestamp"))],
            false,
        ),
    );
    c["options"]["confirmed_timezone"] = json!("Asia/Kolkata");
    let cfg = d.config(&c);
    let mut out = candles().remove(0);
    out["ts_utc_ms"] = json!(13500000);
    check(
        "confirmed",
        d.import(&csv, &cfg),
        0,
        normalized(vec![out], vec![], true),
    );
}
#[test]
fn parser_failure_and_quoted_multiline_source_are_forwarded() {
    let d = Sandbox::new();
    let cfg = d.write("cfg.json", CONFIG);
    let csv = d.write("bad.csv", b"a,b\n1,2\nbad\"x");
    check(
        "parse failure",
        d.import(&csv, &cfg),
        1,
        csv_error("UNEXPECTED_QUOTE", Some(3), 11),
    );
    let csv = d.write("bad utf8", &[b'a', b',', 0xff]);
    check(
        "encoding",
        d.import(&csv, &cfg),
        1,
        csv_error("INVALID_UTF8", None, 2),
    );
    let csv=d.write("quoted.csv",b"\"symbol\",exchange,interval,ts_utc_ms,open,high,low,close,volume,source\nSYNTH,TEST,5m,0,100,103,99,102,10,\"DATA_SENTINEL\r\nsource\"");
    let mut c = config();
    c["metadata"]["source"] = json!("DATA_SENTINEL\r\nsource");
    let cfg = d.config(&c);
    let mut out = candles().remove(0);
    out["source"] = c["metadata"]["source"].clone();
    let mut expected = normalized(vec![out], vec![], true);
    expected["result"]["metadata"] = c["metadata"].clone();
    let actual = d.import(&csv, &cfg);
    assert!(String::from_utf8_lossy(&actual.stdout).contains("DATA_SENTINEL"));
    check("multiline data intentionally present", actual, 0, expected);
}
#[test]
fn gap_warnings_are_success_and_preserve_exact_missing_list() {
    let d = Sandbox::new();
    let csv=d.write("one.csv",b"symbol,exchange,interval,ts_utc_ms,open,high,low,close,volume,source\nSYNTH,TEST,5m,0,100,103,99,102,10,synthetic");
    let mut c = config();
    c["options"]["expected_timestamps_utc_ms"] = json!([0, 300000, 600000]);
    let cfg = d.config(&c);
    let warning =
        json!({"code":"MISSING_EXPECTED_BAR","severity":"warning","row":null,"field":null});
    let mut expected = normalized(
        vec![candles().remove(0)],
        vec![warning.clone(), warning],
        true,
    );
    expected["result"]["gap_assessment"] =
        json!({"status":"performed","missing_ts_utc_ms":[300000,600000]});
    check("gaps", d.import(&csv, &cfg), 0, expected);
}
#[test]
fn child_values_are_not_coerced_defaulted_or_validated_in_cli() {
    let d = Sandbox::new();
    let csv = d.write("valid.csv", VALID);
    let mut c = config();
    c["schema_version"] = json!("2");
    c["metadata"]["source"] = json!(false);
    c["options"]["confirmed_timezone"] = json!("host");
    let cfg = d.config(&c);
    let mut expected = normalized(
        vec![],
        vec![
            issue("UNSUPPORTED_SCHEMA_VERSION", None, Some("schema_version")),
            issue("INVALID_OPTIONS", None, Some("options.confirmed_timezone")),
            issue("INVALID_METADATA", None, Some("metadata.source")),
        ],
        false,
    );
    expected["result"]["metadata"] = Value::Null;
    check("independent core issues", d.import(&csv, &cfg), 1, expected);
    for (field, code, location) in [
        ("metadata", "INVALID_REQUEST", "metadata"),
        ("options", "INVALID_REQUEST", "options"),
        ("schema_version", "INVALID_REQUEST", "schema_version"),
    ] {
        let mut c = config();
        c[field] = Value::Null;
        let cfg = d.config(&c);
        let mut expected = normalized(vec![], vec![issue(code, None, Some(location))], false);
        if field == "metadata" {
            expected["result"]["metadata"] = Value::Null;
        }
        check(field, d.import(&csv, &cfg), 1, expected);
    }
    let mut c = config();
    c["options"]
        .as_object_mut()
        .unwrap()
        .remove("expected_timestamps_utc_ms");
    let cfg = d.config(&c);
    check(
        "no missing grid default",
        d.import(&csv, &cfg),
        1,
        normalized(
            vec![],
            vec![issue(
                "INVALID_OPTIONS",
                None,
                Some("options.expected_timestamps_utc_ms"),
            )],
            false,
        ),
    );
}
#[test]
fn usage_errors_win_before_file_access_and_help_is_plain_static() {
    let d = Sandbox::new();
    let cases: Vec<Vec<&str>> = vec![
        vec!["unknown"],
        vec!["--help", "extra"],
        vec!["import"],
        vec!["import", "--csv"],
        vec!["import", "--csv", "x"],
        vec!["import", "--csv", "x", "--config"],
        vec!["import", "--csv", "x", "--csv", "y"],
        vec!["import", "--config", "x", "--config", "y"],
        vec!["import", "--csv", "-", "--config", "y"],
        vec!["import", "--csv", "x", "--config", "-"],
        vec!["import", "--unknown", "x", "--config", "y"],
        vec!["import", "--csv=x", "--config=y"],
        vec!["import", "--csv", "x", "--config", "y", "extra"],
        vec!["import", "--csv", "--config", "--config", "x"],
        vec!["import", "--csv", "x", "--config", "--unknown"],
        vec!["import", "--csv", "", "--config", "y"],
    ];
    for args in cases {
        check(
            "usage",
            d.spawn(&args.into_iter().map(OsString::from).collect::<Vec<_>>()),
            2,
            cli("USAGE", "args"),
        );
    }
    let help = d.spawn(&["--help".into()]);
    assert_eq!(help.status.code(), Some(0));
    assert!(help.stderr.is_empty());
    assert_eq!(help.stdout,b"QuantOS CSV import\nUsage:\n  quantos-cli\n  quantos-cli --help\n  quantos-cli import --csv <path> --config <path>\n");
}
#[test]
fn json_envelope_duplicates_encoding_trailing_and_nesting_errors() {
    let d = Sandbox::new();
    let missing = d.path.join("missing.csv");
    for raw in [
        b"".as_slice(),
        b"{".as_slice(),
        b"{} trailing".as_slice(),
        b"{} {}".as_slice(),
        b"\xef\xbb\xbf{}".as_slice(),
        &[0xff],
        br#"{"schema_version":"1","schema_version":"1","metadata":{},"options":{}}"#,
        br#"{"schema_version":"1","metadata":{"source":"x","source":"y"},"options":{}}"#,
        br#"{"schema_version":"1","metadata":{"source":"x","\u0073ource":"y"},"options":{}}"#,
        br#"{"schema_version":"1","metadata":{},"options":{"x":[{"a":1,"a":2}]}}"#,
        br#"{"schema_version":"1","metadata":{},"options":{},"extra":{"a":1,"a":2}}"#,
    ] {
        let cfg = d.write("bad.json", raw);
        check(
            "CONFIG_JSON before CSV IO",
            d.import(&missing, &cfg),
            2,
            cli("CONFIG_JSON", "config"),
        );
    }
    for value in [
        json!(null),
        json!(1),
        json!([]),
        json!({}),
        json!({"schema_version":"1","metadata":{}}),
        json!({"schema_version":"1","metadata":{},"options":{},"extra":true}),
    ] {
        let cfg = d.config(&value);
        check(
            "envelope",
            d.import(&missing, &cfg),
            2,
            cli("CONFIG_ENVELOPE", "config"),
        );
    }
    let deep = format!(
        "{{\"schema_version\":\"1\",\"metadata\":{}0{},\"options\":{{}}}}",
        "[".repeat(512),
        "]".repeat(512)
    );
    let cfg = d.write("deep.json", deep.as_bytes());
    check(
        "default nesting bound",
        d.import(&missing, &cfg),
        2,
        cli("CONFIG_JSON", "config"),
    );
}
#[test]
fn io_and_config_csv_precedence_are_sanitized() {
    let d = Sandbox::new();
    let absent = d.path.join("PATH_SENTINEL_SECRET");
    let csv = d.write("bad.csv", b"a\"b");
    for cfg in [&absent, &d.path] {
        let output = d.import(&csv, cfg);
        assert!(!String::from_utf8_lossy(&output.stdout).contains("PATH_SENTINEL_SECRET"));
        check("config read", output, 3, cli("CONFIG_READ", "config"));
    }
    let cfg = d.write("config.json", CONFIG);
    for csv in [&absent, &d.path] {
        check("CSV read", d.import(csv, &cfg), 3, cli("CSV_READ", "csv"));
    }
    let cfg = d.write("config.json", b"{SECRET_CONTENT");
    let output = d.import(&absent, &cfg);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("SECRET_CONTENT"));
    check("JSON before CSV", output, 2, cli("CONFIG_JSON", "config"));
    let mut c = config();
    c["metadata"] = Value::Null;
    let cfg = d.config(&c);
    check(
        "CSV read before child validation",
        d.import(&absent, &cfg),
        3,
        cli("CSV_READ", "csv"),
    );
    check(
        "CSV syntax before child validation",
        d.import(&csv, &cfg),
        1,
        csv_error("UNEXPECTED_QUOTE", Some(1), 1),
    );
}
#[test]
fn exact_limits_are_allowed_and_oversize_precedes_parsing() {
    let d = Sandbox::new();
    let csv = d.write("valid.csv", VALID);
    let mut padded = CONFIG.to_vec();
    padded.resize(1_048_576, b' ');
    let cfg = d.write("exact config.json", &padded);
    check(
        "config inclusive limit",
        d.import(&csv, &cfg),
        0,
        normalized(candles(), vec![], true),
    );
    let cfg = d.path.join("oversize config.json");
    let mut f = File::create(&cfg).unwrap();
    f.write_all(b"invalid").unwrap();
    f.set_len(1_048_577).unwrap();
    drop(f);
    check(
        "config too large before JSON and CSV",
        d.import(&d.path.join("missing.csv"), &cfg),
        2,
        cli("CONFIG_TOO_LARGE", "config"),
    );
    let cfg = d.write("small.json", CONFIG);
    let huge = d.path.join("sparse.csv");
    let mut f = File::create(&huge).unwrap();
    f.write_all(b"a\"").unwrap();
    f.set_len(67_108_864).unwrap();
    drop(f);
    check(
        "CSV inclusive limit passes read stage",
        d.import(&huge, &cfg),
        1,
        csv_error("UNEXPECTED_QUOTE", Some(1), 1),
    );
    File::options()
        .write(true)
        .open(&huge)
        .unwrap()
        .set_len(67_108_865)
        .unwrap();
    check(
        "CSV too large before syntax",
        d.import(&huge, &cfg),
        3,
        cli("CSV_TOO_LARGE", "csv"),
    );
}
#[test]
fn repeated_calls_and_success_failure_leave_inputs_unchanged() {
    let d = Sandbox::new();
    let csv = d.write("input.csv", VALID);
    let cfg = d.write("config.json", CONFIG);
    let a = d.import(&csv, &cfg);
    let b = d.import(&csv, &cfg);
    assert_eq!(a.stdout, b.stdout);
    check("repeat success", a, 0, normalized(candles(), vec![], true));
    assert_eq!(fs::read(&csv).unwrap(), VALID);
    assert_eq!(fs::read(&cfg).unwrap(), CONFIG);
    fs::write(&csv, BAD_OHLC).unwrap();
    let a = d.import(&csv, &cfg);
    let b = d.import(&csv, &cfg);
    assert_eq!(a.stdout, b.stdout);
    check(
        "repeat domain failure",
        a,
        1,
        normalized(vec![], vec![issue("INVALID_OHLC", Some(2), None)], false),
    );
    assert_eq!(fs::read(&csv).unwrap(), BAD_OHLC);
    assert_eq!(fs::read(&cfg).unwrap(), CONFIG);
    fs::write(&cfg, b"{SECRET").unwrap();
    let a = d.import(&csv, &cfg);
    let b = d.import(&csv, &cfg);
    assert_eq!(a.stdout, b.stdout);
    check(
        "repeat operational failure",
        a,
        2,
        cli("CONFIG_JSON", "config"),
    );
    assert_eq!(fs::read(&cfg).unwrap(), b"{SECRET");
    assert_eq!(fs::read(&csv).unwrap(), BAD_OHLC);
}
#[test]
fn native_unicode_input_paths_are_supported() {
    let d = Sandbox::new();
    let csv = d.write("prices-é-株.csv", VALID);
    let cfg = d.write("config-é-株.json", CONFIG);
    check(
        "native Unicode paths",
        d.import(&csv, &cfg),
        0,
        normalized(candles(), vec![], true),
    );
}
#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn native_non_utf8_input_paths_are_supported() {
    use std::os::unix::ffi::OsStringExt;
    let d = Sandbox::new();
    let csv = d.path.join(OsString::from_vec(b"prices\xff.csv".to_vec()));
    let cfg = d.path.join(OsString::from_vec(b"config\xfe.json".to_vec()));
    fs::write(&csv, VALID).unwrap();
    fs::write(&cfg, CONFIG).unwrap();
    check(
        "native non-UTF8 paths",
        d.import(&csv, &cfg),
        0,
        normalized(candles(), vec![], true),
    );
}
#[cfg(target_os = "macos")]
#[test]
fn native_non_utf8_unreadable_paths_are_sanitized() {
    use std::os::unix::ffi::OsStringExt;
    let d = Sandbox::new();
    // Pass raw-byte native paths without trying to create filenames macOS rejects.
    let csv = d.path.join(OsString::from_vec(b"prices\xff.csv".to_vec()));
    let cfg = d.write("import config.json", CONFIG);
    check(
        "unreadable native CSV path",
        d.import(&csv, &cfg),
        3,
        cli("CSV_READ", "csv"),
    );
    let csv = d.write("prices.csv", VALID);
    let cfg = d.path.join(OsString::from_vec(b"config\xfe.json".to_vec()));
    check(
        "unreadable native config path",
        d.import(&csv, &cfg),
        3,
        cli("CONFIG_READ", "config"),
    );
}
