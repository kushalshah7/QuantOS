use super::support;
use quantos_core::data::{CsvParseResultV1, NormalizationResultV1, normalize_v1, parse_csv_v1};
use serde_json::{Value, json};
use std::collections::BTreeSet;

const VALID: &[u8] = include_bytes!("../../../../fixtures/synthetic/valid-candles.csv");
const INVALID_OHLC: &[u8] = include_bytes!("../../../../fixtures/synthetic/invalid-ohlc.csv");
const DUPLICATE: &[u8] = include_bytes!("../../../../fixtures/synthetic/duplicate-timestamp.csv");
const MISSING_ZONE: &[u8] = include_bytes!("../../../../fixtures/synthetic/missing-timezone.csv");
const HEADER: &str = "symbol,exchange,interval,ts_utc_ms,open,high,low,close,volume,source";

fn metadata(source: &str) -> Value {
    json!({"source":source,"synthetic":true,"adjustment_status":"unknown","redistribution_status":"unknown"})
}
fn options(column: &str, format: &str, confirmation: Option<&str>) -> Value {
    json!({"timestamp_column":column,"timestamp_format":format,"confirmed_timezone":confirmation,"expected_timestamps_utc_ms":null})
}

// Test-only composition, with caller-supplied metadata/options. A parse failure
// cannot yield a table for normalization; no public combined API is introduced.
fn pipeline(
    bytes: &[u8],
    metadata: Value,
    options: Value,
) -> (CsvParseResultV1, Option<NormalizationResultV1>) {
    let parsed = parse_csv_v1(bytes);
    let normalized = match &parsed {
        CsvParseResultV1::Success { headers, rows, .. } => Some(normalize_v1(
            &json!({"schema_version":"1","headers":headers,"rows":rows,"metadata":metadata,"options":options}),
        )),
        CsvParseResultV1::Failure { .. } => None,
    };
    (parsed, normalized)
}

fn candles() -> Vec<Value> {
    vec![
        json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":0,"open":10000,"high":10300,"low":9900,"close":10200,"volume":10,"source":"synthetic"}),
        json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":300000,"open":10200,"high":10400,"low":10100,"close":10300,"volume":12,"source":"synthetic"}),
        json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":600000,"open":10300,"high":10500,"low":10000,"close":10100,"volume":8,"source":"synthetic"}),
    ]
}
fn success(candles: Vec<Value>, source: &str) -> Value {
    json!({"schema_version":"1","status":"success","metadata":metadata(source),"candles":candles,"issues":[],"gap_assessment":{"status":"not_performed","missing_ts_utc_ms":[]}})
}
fn failure(code: &str, row: Option<usize>, field: Option<&str>) -> Value {
    json!({"schema_version":"1","status":"failure","metadata":metadata("synthetic"),"candles":[],"issues":[{"code":code,"severity":"error","row":row,"field":field}],"gap_assessment":{"status":"not_performed","missing_ts_utc_ms":[]}})
}
fn check_normalized(id: &str, actual: Option<NormalizationResultV1>, expected: Value) {
    let result = actual.expect("parser success must feed the test-only pipeline");
    let mut actual = serde_json::to_value(result).unwrap();
    for issue in actual["issues"].as_array_mut().unwrap() {
        let object = issue.as_object_mut().unwrap();
        assert_eq!(
            object.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from(["code", "severity", "row", "field", "message"]),
            "{id}: normalizer issue shape unchanged"
        );
        assert!(
            object["message"]
                .as_str()
                .is_some_and(|text| !text.trim().is_empty())
        );
        object.remove("message");
    }
    assert_eq!(
        actual, expected,
        "{id}: exact normalized values and ordered issue keys"
    );
}

#[test]
fn csv_pipeline_valid_fixture_exact_three_candles() {
    let (parsed, normalized) = pipeline(
        VALID,
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert!(matches!(parsed, CsvParseResultV1::Success { .. }));
    check_normalized(
        "CSV_PIPELINE_VALID",
        normalized,
        success(candles(), "synthetic"),
    );
}

#[test]
fn csv_pipeline_invalid_ohlc_is_parser_success_and_atomic_domain_failure() {
    let (parsed, normalized) = pipeline(
        INVALID_OHLC,
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert!(matches!(parsed, CsvParseResultV1::Success { .. }));
    check_normalized(
        "CSV_PIPELINE_OHLC",
        normalized,
        failure("INVALID_OHLC", Some(2), None),
    );
}

#[test]
fn csv_pipeline_duplicate_is_parser_success_and_atomic_domain_failure() {
    let (parsed, normalized) = pipeline(
        DUPLICATE,
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert!(matches!(parsed, CsvParseResultV1::Success { .. }));
    check_normalized(
        "CSV_PIPELINE_DUPLICATE",
        normalized,
        failure("DUPLICATE_TIMESTAMP", Some(3), Some("ts_utc_ms")),
    );
}

#[test]
fn csv_pipeline_missing_timezone_needs_explicit_mapping_and_confirmation() {
    let (parsed, normalized) = pipeline(
        MISSING_ZONE,
        metadata("synthetic"),
        options("timestamp", "iso8601", None),
    );
    assert!(matches!(parsed, CsvParseResultV1::Success { .. }));
    check_normalized(
        "CSV_PIPELINE_MISSING_ZONE",
        normalized,
        failure("TIMEZONE_REQUIRED", Some(2), Some("timestamp")),
    );
    let (_, normalized) = pipeline(
        MISSING_ZONE,
        metadata("synthetic"),
        options("timestamp", "iso8601", Some("Asia/Kolkata")),
    );
    let mut out = candles().remove(0);
    out["ts_utc_ms"] = json!(13500000);
    check_normalized(
        "CSV_PIPELINE_CONFIRMED_KOLKATA",
        normalized,
        success(vec![out], "synthetic"),
    );
}

#[test]
fn csv_pipeline_quoted_canonical_fields_and_initial_bom_normalize_identically() {
    let quoted=b"\"symbol\",\"exchange\",\"interval\",\"ts_utc_ms\",\"open\",\"high\",\"low\",\"close\",\"volume\",\"source\"\n\"SYNTH\",\"TEST\",\"5m\",\"0\",\"100\",\"103\",\"99\",\"102\",\"10\",\"synthetic\"\n\"SYNTH\",\"TEST\",\"5m\",\"300000\",\"102\",\"104\",\"101\",\"103\",\"12\",\"synthetic\"\n\"SYNTH\",\"TEST\",\"5m\",\"600000\",\"103\",\"105\",\"100\",\"101\",\"8\",\"synthetic\"";
    let (_, normalized) = pipeline(
        quoted,
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    check_normalized(
        "CSV_PIPELINE_QUOTED",
        normalized,
        success(candles(), "synthetic"),
    );
    let bom = [b"\xef\xbb\xbf".as_slice(), VALID].concat();
    let (_, normalized) = pipeline(
        &bom,
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    check_normalized(
        "CSV_PIPELINE_BOM",
        normalized,
        success(candles(), "synthetic"),
    );
}

#[test]
fn csv_pipeline_multiline_source_survives_without_line_ending_normalization() {
    let bytes = format!("{HEADER}\nSYNTH,TEST,5m,0,100,103,99,102,10,\"synthetic\r\nsource\"");
    let source = "synthetic\r\nsource";
    let (parsed, normalized) = pipeline(
        bytes.as_bytes(),
        metadata(source),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert_eq!(
        serde_json::to_value(parsed).unwrap()["rows"][0][9],
        json!(source)
    );
    let mut out = candles().remove(0);
    out["source"] = json!(source);
    check_normalized(
        "CSV_PIPELINE_MULTILINE_SOURCE",
        normalized,
        success(vec![out], source),
    );
}

#[test]
fn csv_pipeline_blank_data_record_is_retained_and_becomes_row_width_at_two() {
    let bytes = format!("{HEADER}\n\n");
    let (parsed, normalized) = pipeline(
        bytes.as_bytes(),
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert_eq!(serde_json::to_value(parsed).unwrap()["rows"], json!([[""]]));
    check_normalized(
        "CSV_PIPELINE_BLANK_ROW",
        normalized,
        failure("ROW_WIDTH", Some(2), None),
    );
}

#[test]
fn csv_pipeline_header_only_is_parser_success_not_valid_domain_data() {
    let (parsed, normalized) = pipeline(
        HEADER.as_bytes(),
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert!(matches!(parsed, CsvParseResultV1::Success { .. }));
    check_normalized(
        "CSV_PIPELINE_HEADER_ONLY",
        normalized,
        failure("EMPTY_DATASET", None, Some("rows")),
    );
}

#[test]
fn csv_pipeline_malformed_bytes_never_return_partial_table_or_normalize() {
    let bytes = b"a,b\n1,2\nbad\"x";
    let (parsed, normalized) = pipeline(
        bytes,
        metadata("synthetic"),
        options("ts_utc_ms", "utc_ms", None),
    );
    assert!(matches!(parsed, CsvParseResultV1::Failure { .. }));
    assert!(normalized.is_none());
    support::check(
        "CSV_PIPELINE_NO_PARTIAL",
        bytes,
        support::failure("UNEXPECTED_QUOTE", Some(3), 11),
    );
}
