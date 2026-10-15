use super::support::*;
use serde_json::{Value, json};

#[test]
fn n01_n02_canonical() {
    check("N01", r(), success(vec![c0(), c1(), c2()]));
    let mut input = q();
    input["options"]["confirmed_timezone"] = json!("UTC");
    check("N02", input, success(vec![c0()]));
}

#[test]
fn t01_through_t13_all_time_alternatives() {
    for (id, text, zone) in [
        ("T01", "1970-01-01T03:45:00Z", None),
        ("T02", "1970-01-01T09:15:00+05:30", None),
        ("T03", "1970-01-01T09:15:00", Some("Asia/Kolkata")),
        ("T04", "1970-01-01T03:45:00", Some("UTC")),
    ] {
        check(
            id,
            t(text, zone),
            success(vec![candle("ts_utc_ms", json!(13500000))]),
        );
    }
    check(
        "T05",
        t("1970-01-01T09:15:00", None),
        f("TIMEZONE_REQUIRED", Some(2), Some("timestamp")),
    );
    check(
        "T06",
        t("1970-01-02T00:15:00+05:30", Some("Asia/Kolkata")),
        success(vec![candle("ts_utc_ms", json!(67500000))]),
    );
    check(
        "T07",
        t("1970-01-01T03:45:00Z", Some("Asia/Kolkata")),
        f("TIMEZONE_CONFLICT", Some(2), Some("timestamp")),
    );
    check(
        "T08",
        t("1970-01-01T09:15:00+05:30", Some("UTC")),
        f("TIMEZONE_CONFLICT", Some(2), Some("timestamp")),
    );
    let mut input = q();
    input["options"]["confirmed_timezone"] = json!("Asia/Kolkata");
    check(
        "T09",
        input,
        f(
            "TIMEZONE_CONFLICT",
            None,
            Some("options.confirmed_timezone"),
        ),
    );
    check(
        "T10",
        t("1970-01-01T00:00:00.1Z", None),
        success(vec![candle("ts_utc_ms", json!(100))]),
    );
    for (i, text) in [
        "1970-02-30T00:00:00Z",
        "1970-01-01T24:00:00Z",
        "1970-01-01T00:00:60Z",
        "1970-01-01T00:00:00.0000Z",
        "1970-01-01",
    ]
    .iter()
    .enumerate()
    {
        check(
            &format!("T11-{i}"),
            t(text, None),
            f("INVALID_TIMESTAMP", Some(2), Some("timestamp")),
        );
    }
    for (i, text, zone) in [
        (0, "1970-01-01T00:00:00+00:00", Some("UTC")),
        (1, "1970-01-01T00:00:00-00:00", None),
    ] {
        check(&format!("T12-{i}"), t(text, zone), success(vec![c0()]));
    }
    let mut input = q();
    input["rows"][0][3] = json!("1970-01-01T00:00:00Z");
    check(
        "T13",
        input,
        f("INVALID_TIMESTAMP", Some(2), Some("ts_utc_ms")),
    );
}

#[test]
fn p01_through_p06_v01_through_v03_all_numeric_alternatives() {
    for (id, col, text, outfield, out) in [
        ("P01", 4, "100.0100", "open", 10001),
        ("P02", 4, "000100.0000", "open", 10000),
        ("P03", 6, "0.0100", "low", 1),
    ] {
        let mut input = q();
        input["rows"][0][col] = json!(text);
        check(id, input, success(vec![candle(outfield, json!(out))]));
    }
    for (id, values, code) in [
        ("P04", vec!["100.001", "100.00100"], "PRICE_PRECISION"),
        ("P05", vec!["0", "-1", "-0.00"], "PRICE_OUT_OF_RANGE"),
        (
            "P06",
            vec![
                "NaN",
                "Infinity",
                "-Infinity",
                "1e2",
                "+100",
                "100.",
                ".50",
                " 100",
            ],
            "INVALID_PRICE",
        ),
    ] {
        for (i, text) in values.iter().enumerate() {
            let mut input = q();
            input["rows"][0][4] = json!(text);
            check(&format!("{id}-{i}"), input, f(code, Some(2), Some("open")));
        }
    }
    for (i, text) in ["0", "000", "-0"].iter().enumerate() {
        let mut input = q();
        input["rows"][0][8] = json!(text);
        check(
            &format!("V01-{i}"),
            input,
            success(vec![candle("volume", json!(0))]),
        );
    }
    let mut input = q();
    input["rows"][0][8] = json!("-1");
    check(
        "V02",
        input,
        f("VOLUME_OUT_OF_RANGE", Some(2), Some("volume")),
    );
    for (i, text) in ["1.0", "1e1", "NaN", "Infinity", " 10"].iter().enumerate() {
        let mut input = q();
        input["rows"][0][8] = json!(text);
        check(
            &format!("V03-{i}"),
            input,
            f("INVALID_VOLUME", Some(2), Some("volume")),
        );
    }
}

#[test]
fn b01_through_b06_exact_bounds() {
    let mut input = q();
    input["rows"][0][3] = json!("4102444799999");
    check(
        "B01",
        input,
        success(vec![candle("ts_utc_ms", json!(4102444799999_u64))]),
    );
    for (i, text) in ["-1", "4102444800000"].iter().enumerate() {
        let mut input = q();
        input["rows"][0][3] = json!(text);
        check(
            &format!("B02-{i}"),
            input,
            f("TIMESTAMP_OUT_OF_RANGE", Some(2), Some("ts_utc_ms")),
        );
    }
    let mut input = q();
    input["rows"][0][3] = json!("0.0");
    check(
        "B03",
        input,
        f("INVALID_TIMESTAMP", Some(2), Some("ts_utc_ms")),
    );
    for (i, text) in ["1969-12-31T23:59:59.999Z", "2100-01-01T00:00:00Z"]
        .iter()
        .enumerate()
    {
        check(
            &format!("B04-{i}"),
            t(text, None),
            f("TIMESTAMP_OUT_OF_RANGE", Some(2), Some("timestamp")),
        );
    }
    let mut input = q();
    let mut expected = c0();
    for (index, field) in [(4, "open"), (5, "high"), (6, "low"), (7, "close")] {
        input["rows"][0][index] = json!("90071992547409.91");
        expected[field] = json!(9007199254740991_u64);
    }
    input["rows"][0][8] = json!("9007199254740991");
    expected["volume"] = json!(9007199254740991_u64);
    check("B05", input, success(vec![expected]));
    let mut input = q();
    input["rows"][0][3] = json!("4102444800000");
    input["rows"][0][4] = json!("90071992547409.92");
    input["rows"][0][8] = json!("9007199254740992");
    check(
        "B06",
        input,
        failure(vec![
            issue("TIMESTAMP_OUT_OF_RANGE", Some(2), Some("ts_utc_ms")),
            issue("PRICE_OUT_OF_RANGE", Some(2), Some("open")),
            issue("VOLUME_OUT_OF_RANGE", Some(2), Some("volume")),
        ]),
    );
}

#[test]
fn d01_through_d08_a01_through_a03_validation_and_atomicity() {
    let mut input = q();
    input["rows"][0][5] = json!("98");
    check("D01", input, f("INVALID_OHLC", Some(2), None));
    let mut input = r();
    input["rows"][1][3] = json!("0");
    check(
        "D02",
        input,
        f("DUPLICATE_TIMESTAMP", Some(3), Some("ts_utc_ms")),
    );
    let mut input = r();
    input["rows"][1][3] = json!("600000");
    input["rows"][2][3] = json!("300000");
    check(
        "D03",
        input,
        f("DESCENDING_TIMESTAMP", Some(4), Some("ts_utc_ms")),
    );
    let mut input = r();
    input["rows"][2][3] = json!("0");
    check(
        "D04",
        input,
        failure(vec![
            issue("DUPLICATE_TIMESTAMP", Some(4), Some("ts_utc_ms")),
            issue("DESCENDING_TIMESTAMP", Some(4), Some("ts_utc_ms")),
        ]),
    );
    let mut input = r();
    input["rows"][1][0] = json!("OTHER");
    input["rows"][2][1] = json!("OTHER");
    check(
        "D05",
        input,
        failure(vec![
            issue("MIXED_DATASET", Some(3), Some("symbol")),
            issue("MIXED_DATASET", Some(4), Some("exchange")),
        ]),
    );
    let mut input = q();
    input["rows"][0][0] = json!(" ");
    input["rows"][0][1] = json!("");
    check(
        "D06",
        input,
        failure(vec![
            issue("INVALID_IDENTITY", Some(2), Some("symbol")),
            issue("INVALID_IDENTITY", Some(2), Some("exchange")),
        ]),
    );
    let mut input = q();
    input["rows"][0][2] = json!("1m");
    check(
        "D07",
        input,
        f("UNSUPPORTED_INTERVAL", Some(2), Some("interval")),
    );
    let mut input = q();
    input["rows"][0][9] = json!("other");
    check("D08", input, f("SOURCE_MISMATCH", Some(2), Some("source")));
    let mut input = r();
    input["rows"][0][4] = json!("NaN");
    input["rows"][0][8] = json!("-1");
    input["rows"][1][3] = json!("0");
    input["rows"][1][5] = json!("Infinity");
    check(
        "A01",
        input,
        failure(vec![
            issue("INVALID_PRICE", Some(2), Some("open")),
            issue("VOLUME_OUT_OF_RANGE", Some(2), Some("volume")),
            issue("DUPLICATE_TIMESTAMP", Some(3), Some("ts_utc_ms")),
            issue("INVALID_PRICE", Some(3), Some("high")),
        ]),
    );
    let mut input = q();
    input["rows"][0][5] = json!("98");
    input["rows"][0][8] = json!("-1");
    check(
        "A02",
        input,
        failure(vec![
            issue("VOLUME_OUT_OF_RANGE", Some(2), Some("volume")),
            issue("INVALID_OHLC", Some(2), None),
        ]),
    );
    let mut input = q();
    input["metadata"]["adjustment_status"] = json!("bad");
    input["options"]["expected_timestamps_utc_ms"] = json!([0, 0]);
    input["rows"][0][4] = json!("100.001");
    let mut expected = failure(vec![
        issue("INVALID_METADATA", None, Some("metadata.adjustment_status")),
        issue(
            "INVALID_EXPECTED_GRID",
            None,
            Some("options.expected_timestamps_utc_ms"),
        ),
        issue("PRICE_PRECISION", Some(2), Some("open")),
    ]);
    expected["metadata"] = Value::Null;
    check("A03", input, expected);
}

#[test]
fn g01_through_g06_explicit_grids() {
    let mut observed = r();
    observed["rows"].as_array_mut().unwrap().remove(1);
    check("G01", observed.clone(), success(vec![c0(), c2()]));
    observed["options"]["expected_timestamps_utc_ms"] = json!([0, 300000, 600000]);
    let mut expected = success(vec![c0(), c2()]);
    expected["gap_assessment"] = json!({"status":"performed","missing_ts_utc_ms":[300000]});
    expected["issues"] = json!([warning()]);
    check("G02", observed, expected);
    let mut input = q();
    input["options"]["expected_timestamps_utc_ms"] = json!([0, 300000, 600000]);
    let mut expected = success(vec![c0()]);
    expected["gap_assessment"] = json!({"status":"performed","missing_ts_utc_ms":[300000,600000]});
    expected["issues"] = json!([warning(), warning()]);
    check("G03", input, expected);
    let mut input = q();
    input["options"]["expected_timestamps_utc_ms"] = json!([]);
    let mut expected = success(vec![c0()]);
    expected["gap_assessment"] = json!({"status":"performed","missing_ts_utc_ms":[]});
    check("G04", input, expected);
    for (i, grid) in [
        json!([0, 0]),
        json!([300000, 0]),
        json!([-1]),
        json!([4102444800000_u64]),
        json!([0.5]),
        json!("0"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut input = q();
        input["options"]["expected_timestamps_utc_ms"] = grid;
        check(
            &format!("G05-{i}"),
            input,
            f(
                "INVALID_EXPECTED_GRID",
                None,
                Some("options.expected_timestamps_utc_ms"),
            ),
        );
    }
    let mut input = q();
    input["options"]["expected_timestamps_utc_ms"] = json!([0, 300000]);
    input["rows"][0][4] = json!("NaN");
    check("G06", input, f("INVALID_PRICE", Some(2), Some("open")));
}

#[test]
fn s01_through_s12_metadata_and_shapes() {
    let mut input = q();
    input["schema_version"] = json!("2");
    check(
        "S01",
        input,
        f("UNSUPPORTED_SCHEMA_VERSION", None, Some("schema_version")),
    );
    let mut input = q();
    input["metadata"]["source"] = json!("");
    let mut expected = f("INVALID_METADATA", None, Some("metadata.source"));
    expected["metadata"] = Value::Null;
    check("S02", input, expected);
    let declared = json!({"source":"synthetic","synthetic":false,"adjustment_status":"split_adjusted","redistribution_status":"private_only"});
    let mut input = q();
    input["metadata"] = declared.clone();
    let mut expected = success(vec![c0()]);
    expected["metadata"] = declared;
    check("S03", input, expected);
    let mut input = q();
    input["headers"].as_array_mut().unwrap().pop();
    input["rows"][0].as_array_mut().unwrap().pop();
    check("S04", input, f("INVALID_HEADER", Some(1), None));
    let mut input = q();
    input["headers"]
        .as_array_mut()
        .unwrap()
        .push(json!("extra"));
    input["rows"][0]
        .as_array_mut()
        .unwrap()
        .push(json!("invented"));
    check("S05", input, f("INVALID_HEADER", Some(1), None));
    let mut input = r();
    input["rows"][2].as_array_mut().unwrap().pop();
    check("S06", input, f("ROW_WIDTH", Some(4), None));
    let mut input = r();
    input["rows"] = json!([]);
    check("S07", input, f("EMPTY_DATASET", None, Some("rows")));
    let mut input = q();
    input["options"]["timestamp_column"] = json!("open");
    check(
        "S08",
        input,
        f("INVALID_OPTIONS", None, Some("options.timestamp_column")),
    );
    let mut input = q();
    input["options"]["timestamp_format"] = json!("guess");
    check(
        "S09",
        input,
        f("INVALID_OPTIONS", None, Some("options.timestamp_format")),
    );
    let mut input = q();
    input["extra"] = json!(true);
    check("S10", input, f("INVALID_REQUEST", None, Some("extra")));
    let mut input = q();
    input["options"]["confirmed_timezone"] = json!("host");
    check(
        "S11",
        input,
        f("INVALID_OPTIONS", None, Some("options.confirmed_timezone")),
    );
    let mut input = q();
    input["headers"][9] = json!("open");
    check("S12", input, f("INVALID_HEADER", Some(1), None));
}
