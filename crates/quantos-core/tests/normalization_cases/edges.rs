use super::support::*;
use quantos_core::data::{GapAssessmentV1, normalize_v1};
use serde_json::{Value, json};

#[test]
fn reordered_headers_and_explicit_mapping_preserve_values() {
    let mut input = t("1970-01-01T09:15:00", Some("Asia/Kolkata"));
    input["headers"].as_array_mut().unwrap().reverse();
    input["rows"][0].as_array_mut().unwrap().reverse();
    check(
        "reordered mapped headers",
        input,
        success(vec![candle("ts_utc_ms", json!(13500000))]),
    );
}

#[test]
fn root_missing_wrong_type_unknown_and_non_object_are_reports() {
    for (i, value) in [
        Value::Null,
        json!(false),
        json!(1),
        json!("request"),
        json!([]),
    ]
    .into_iter()
    .enumerate()
    {
        let mut expected = f("INVALID_REQUEST", None, None);
        expected["metadata"] = Value::Null;
        check(&format!("non-object-{i}"), value, expected);
    }
    for field in ["schema_version", "headers", "rows", "metadata", "options"] {
        for missing in [true, false] {
            let mut input = q();
            if missing {
                input.as_object_mut().unwrap().remove(field);
            } else {
                input[field] = json!(false);
            }
            let mut expected = f("INVALID_REQUEST", None, Some(field));
            if field == "metadata" {
                expected["metadata"] = Value::Null;
            }
            check(&format!("root-{field}-{missing}"), input, expected);
        }
    }
    let mut expected = failure(
        ["headers", "metadata", "options", "rows", "schema_version"]
            .map(|field| issue("INVALID_REQUEST", None, Some(field)))
            .to_vec(),
    );
    expected["metadata"] = Value::Null;
    check(
        "all independent root missing properties",
        json!({}),
        expected,
    );
    let mut input = q();
    input["headers"][0] = json!(1);
    check(
        "wrong typed header cell",
        input,
        f("INVALID_REQUEST", None, Some("headers")),
    );
}

#[test]
fn options_properties_have_exact_locations_and_no_defaults() {
    for field in [
        "timestamp_column",
        "timestamp_format",
        "confirmed_timezone",
        "expected_timestamps_utc_ms",
    ] {
        for missing in [true, false] {
            let mut input = q();
            if missing {
                input["options"].as_object_mut().unwrap().remove(field);
            } else {
                input["options"][field] = json!(false);
            }
            let code = if field == "expected_timestamps_utc_ms" && !missing {
                "INVALID_EXPECTED_GRID"
            } else {
                "INVALID_OPTIONS"
            };
            check(
                &format!("option-{field}-{missing}"),
                input,
                f(code, None, Some(&format!("options.{field}"))),
            );
        }
    }
    let mut input = q();
    input["options"]["unexpected"] = json!(null);
    check(
        "unknown option",
        input,
        f("INVALID_OPTIONS", None, Some("options.unexpected")),
    );
    for name in ["", "source", "high", "symbol"] {
        let mut input = q();
        input["options"]["timestamp_column"] = json!(name);
        check(
            "invalid timestamp column",
            input,
            f("INVALID_OPTIONS", None, Some("options.timestamp_column")),
        );
    }
    let mut input = t("1970-01-01T09:15:00", Some("host"));
    input["rows"][0][4] = json!("NaN");
    check(
        "invalid confirmation is not null",
        input,
        failure(vec![
            issue("INVALID_OPTIONS", None, Some("options.confirmed_timezone")),
            issue("INVALID_PRICE", Some(2), Some("open")),
        ]),
    );
    check(
        "explicit offset remains usable with invalid confirmation",
        t("1970-01-01T09:15:00+05:30", Some("host")),
        f("INVALID_OPTIONS", None, Some("options.confirmed_timezone")),
    );
}

#[test]
fn metadata_properties_preserve_valid_source_dependencies() {
    for field in [
        "source",
        "synthetic",
        "adjustment_status",
        "redistribution_status",
    ] {
        for missing in [true, false] {
            let mut input = q();
            if missing {
                input["metadata"].as_object_mut().unwrap().remove(field);
            } else {
                input["metadata"][field] = json!(123);
            }
            let mut expected = f("INVALID_METADATA", None, Some(&format!("metadata.{field}")));
            expected["metadata"] = Value::Null;
            check(&format!("metadata-{field}-{missing}"), input, expected);
        }
    }
    let mut input = q();
    input["metadata"]["extra"] = json!(true);
    input["rows"][0][9] = json!("private-feed");
    let mut expected = failure(vec![
        issue("INVALID_METADATA", None, Some("metadata.extra")),
        issue("SOURCE_MISMATCH", Some(2), Some("source")),
    ]);
    expected["metadata"] = Value::Null;
    check(
        "unknown metadata preserves valid source check",
        input,
        expected,
    );
    let mut input = q();
    input["metadata"]["source"] = json!("\t \r\n");
    input["rows"][0][9] = json!("private-feed");
    let mut expected = f("INVALID_METADATA", None, Some("metadata.source"));
    expected["metadata"] = Value::Null;
    check("invalid source suppresses only matching", input, expected);
    for adjustment in [
        "unknown",
        "unadjusted",
        "split_adjusted",
        "total_return_adjusted",
    ] {
        for redistribution in ["unknown", "private_only", "permitted"] {
            let mut input = q();
            input["metadata"]["adjustment_status"] = json!(adjustment);
            input["metadata"]["redistribution_status"] = json!(redistribution);
            let mut expected = success(vec![c0()]);
            expected["metadata"] = input["metadata"].clone();
            check("all closed provenance states", input, expected);
        }
    }
    let mut input = q();
    input["metadata"]["source"] = json!(" custom ");
    input["rows"][0][9] = json!(" custom ");
    input["rows"][0][0] = json!(" SYNTH ");
    let mut out = c0();
    out["source"] = json!(" custom ");
    out["symbol"] = json!(" SYNTH ");
    let mut expected = success(vec![out]);
    expected["metadata"] = input["metadata"].clone();
    check(
        "nonempty declaration and identity whitespace preserved",
        input,
        expected,
    );
}

#[test]
fn malformed_rows_and_cells_keep_independent_siblings() {
    for value in [Value::Null, json!("not a row"), json!({}), json!(true)] {
        let mut input = q();
        input["rows"][0] = value;
        check(
            "non-array record",
            input,
            f("INVALID_REQUEST", Some(2), None),
        );
    }
    let mut input = q();
    input["rows"][0].as_array_mut().unwrap().pop();
    check("short record", input, f("ROW_WIDTH", Some(2), None));
    let mut input = q();
    input["rows"][0][0] = json!(123);
    input["rows"][0][4] = json!("PRIVATE_PRICE_VALUE");
    input["rows"][0][8] = json!("-1");
    input["rows"][0][9] = json!("PRIVATE_FEED_VALUE");
    let result = normalize_v1(&input);
    assert!(
        result
            .issues
            .iter()
            .all(|i| !i.message.contains("PRIVATE_PRICE_VALUE")
                && !i.message.contains("PRIVATE_FEED_VALUE"))
    );
    check(
        "non-string identity and independently invalid siblings",
        input,
        failure(vec![
            issue("INVALID_REQUEST", Some(2), Some("symbol")),
            issue("INVALID_PRICE", Some(2), Some("open")),
            issue("VOLUME_OUT_OF_RANGE", Some(2), Some("volume")),
            issue("SOURCE_MISMATCH", Some(2), Some("source")),
        ]),
    );
    let mut input = r();
    input["rows"][0][4] = json!("NaN");
    input["rows"][1][0] = json!("OTHER");
    check(
        "identity despite unrelated price error",
        input,
        failure(vec![
            issue("INVALID_PRICE", Some(2), Some("open")),
            issue("MIXED_DATASET", Some(3), Some("symbol")),
        ]),
    );
    let mut input = r();
    input["rows"][0][0] = json!(false);
    input["rows"][1][0] = json!("OTHER");
    check(
        "first usable identity after wrong typed cell",
        input,
        failure(vec![
            issue("INVALID_REQUEST", Some(2), Some("symbol")),
            issue("MIXED_DATASET", Some(4), Some("symbol")),
        ]),
    );
}

#[test]
fn malformed_time_and_wrong_width_do_not_hide_sequence_errors() {
    for (first, last, code) in [
        ("300000", "0", "DESCENDING_TIMESTAMP"),
        ("0", "0", "DUPLICATE_TIMESTAMP"),
    ] {
        let mut input = r();
        input["rows"][0][3] = json!(first);
        input["rows"][1][3] = json!("malformed");
        input["rows"][2][3] = json!(last);
        check(
            "valid timestamps around malformed cell",
            input,
            failure(vec![
                issue("INVALID_TIMESTAMP", Some(3), Some("ts_utc_ms")),
                issue(code, Some(4), Some("ts_utc_ms")),
            ]),
        );
    }
    let mut input = r();
    input["rows"][0][3] = json!("300000");
    input["rows"][1].as_array_mut().unwrap().pop();
    input["rows"][2][3] = json!("0");
    check(
        "valid timestamps around wrong-width row",
        input,
        failure(vec![
            issue("ROW_WIDTH", Some(3), None),
            issue("DESCENDING_TIMESTAMP", Some(4), Some("ts_utc_ms")),
        ]),
    );
}

#[test]
fn gregorian_leaps_offset_bounds_and_fractional_milliseconds() {
    check(
        "Gregorian leap 2000",
        t("2000-02-29T00:00:00Z", None),
        success(vec![candle("ts_utc_ms", json!(951782400000_u64))]),
    );
    for text in [
        "1900-02-29T00:00:00Z",
        "2001-02-29T00:00:00Z",
        "2100-02-29T00:00:00Z",
        "1970-04-31T00:00:00Z",
        "1970-00-01T00:00:00Z",
        "1970-01-00T00:00:00Z",
        "1970-01-01T00:00:00+24:00",
        "1970-01-01T00:00:00+00:60",
    ] {
        check(
            "invalid Gregorian date/offset",
            t(text, None),
            f("INVALID_TIMESTAMP", Some(2), Some("timestamp")),
        );
    }
    check(
        "syntax precedes missing timezone",
        t("1970-02-30T09:15:00", None),
        f("INVALID_TIMESTAMP", Some(2), Some("timestamp")),
    );
    for text in ["1970-01-01T05:30:00+05:30", "1969-12-31T23:00:00-01:00"] {
        check(
            "lower UTC boundary after offset",
            t(text, None),
            success(vec![c0()]),
        );
    }
    check(
        "upper UTC boundary after offset",
        t("2100-01-01T05:29:59.999+05:30", Some("Asia/Kolkata")),
        success(vec![candle("ts_utc_ms", json!(4102444799999_u64))]),
    );
    for text in [
        "1970-01-01T05:29:59.999+05:30",
        "2100-01-01T05:30:00+05:30",
        "0000-02-29T00:00:00Z",
    ] {
        check(
            "UTC bound after valid Gregorian conversion",
            t(text, None),
            f("TIMESTAMP_OUT_OF_RANGE", Some(2), Some("timestamp")),
        );
    }
    for (text, value) in [
        ("1970-01-01T00:00:00.001Z", 1),
        ("1970-01-01T00:00:00.01Z", 10),
        ("1970-01-01T00:00:00.000Z", 0),
    ] {
        check(
            "millisecond precision",
            t(text, None),
            success(vec![candle("ts_utc_ms", json!(value))]),
        );
    }
    check(
        "explicit subset maximum offset",
        t("1970-01-02T00:00:00+23:59", None),
        success(vec![candle("ts_utc_ms", json!(60000))]),
    );
}

#[test]
fn arbitrarily_long_numeric_strings_keep_error_precedence() {
    let huge = "9".repeat(10000);
    let mut input = q();
    input["rows"][0][3] = json!(huge);
    input["rows"][0][4] = json!(huge);
    input["rows"][0][8] = json!(huge);
    check(
        "long valid integers overflow as range errors",
        input,
        failure(vec![
            issue("TIMESTAMP_OUT_OF_RANGE", Some(2), Some("ts_utc_ms")),
            issue("PRICE_OUT_OF_RANGE", Some(2), Some("open")),
            issue("VOLUME_OUT_OF_RANGE", Some(2), Some("volume")),
        ]),
    );
    let mut input = q();
    input["rows"][0][4] = json!(format!("{huge}.001"));
    check(
        "precision precedes huge range",
        input,
        f("PRICE_PRECISION", Some(2), Some("open")),
    );
    let mut input = q();
    input["rows"][0][3] = json!(format!("{huge}x"));
    input["rows"][0][4] = json!(format!("{huge}.001x"));
    input["rows"][0][8] = json!(format!("{huge}x"));
    check(
        "lexical suffix precedes overflow/precision",
        input,
        failure(vec![
            issue("INVALID_TIMESTAMP", Some(2), Some("ts_utc_ms")),
            issue("INVALID_PRICE", Some(2), Some("open")),
            issue("INVALID_VOLUME", Some(2), Some("volume")),
        ]),
    );
    let zeros = "0".repeat(10000);
    let mut input = q();
    input["rows"][0][3] = json!(format!("{zeros}0"));
    input["rows"][0][4] = json!(format!("{zeros}100.0100"));
    input["rows"][0][8] = json!(format!("{zeros}0"));
    let mut out = c0();
    out["open"] = json!(10001);
    out["volume"] = json!(0);
    check(
        "long leading zeros preserve exact value",
        input,
        success(vec![out]),
    );
    let mut input = q();
    input["rows"][0][3] = json!("-000");
    input["rows"][0][8] = json!("-000");
    check(
        "integer negative zero",
        input,
        success(vec![candle("volume", json!(0))]),
    );
    let mut input = q();
    input["rows"][0][4] = json!("-000.000");
    check(
        "price negative zero is not positive",
        input,
        f("PRICE_OUT_OF_RANGE", Some(2), Some("open")),
    );
}

#[test]
fn unicode_codepoint_issue_order_is_not_utf16_or_insertion_order() {
    let mut input = q();
    input["\u{10000}"] = json!(true);
    input["\u{e000}"] = json!(true);
    input["z"] = json!(true);
    input["options"]["\u{10000}"] = json!(true);
    input["options"]["\u{e000}"] = json!(true);
    check(
        "global Unicode code-point order",
        input,
        failure(vec![
            issue("INVALID_REQUEST", None, Some("z")),
            issue("INVALID_REQUEST", None, Some("\u{e000}")),
            issue("INVALID_REQUEST", None, Some("\u{10000}")),
            issue("INVALID_OPTIONS", None, Some("options.\u{e000}")),
            issue("INVALID_OPTIONS", None, Some("options.\u{10000}")),
        ]),
    );
    let mut input = q();
    input["headers"][0] = json!("\u{10000}");
    input["headers"][1] = json!("\u{e000}");
    input["rows"][0][0] = json!(false);
    input["rows"][0][1] = json!(false);
    check(
        "unmapped cell fields use code-point order",
        input,
        failure(vec![
            issue("INVALID_HEADER", Some(1), None),
            issue("INVALID_REQUEST", Some(2), Some("\u{e000}")),
            issue("INVALID_REQUEST", Some(2), Some("\u{10000}")),
        ]),
    );
}

#[test]
fn explicit_grid_retains_extra_observations_and_numeric_integer_values() {
    let mut input = q();
    input["rows"][0][3] = json!("13500000");
    input["options"]["expected_timestamps_utc_ms"] = json!([0, 300000]);
    let mut expected = success(vec![candle("ts_utc_ms", json!(13500000))]);
    expected["gap_assessment"] = json!({"status":"performed","missing_ts_utc_ms":[0,300000]});
    expected["issues"] = json!([warning(), warning()]);
    let result = normalize_v1(&input);
    assert!(matches!(
        result.gap_assessment,
        GapAssessmentV1::Performed { .. }
    ));
    check("extra observations retained", input, expected);
    let mut input = q();
    input["options"]["expected_timestamps_utc_ms"] = json!([0.0]);
    let mut expected = success(vec![c0()]);
    expected["gap_assessment"] = json!({"status":"performed","missing_ts_utc_ms":[]});
    check("JSON mathematical integer grid number", input, expected);
}

#[test]
fn repeated_full_results_and_messages_are_deterministic() {
    let mut invalid = r();
    invalid["rows"][0][4] = json!("NaN");
    invalid["rows"][1][3] = json!("0");
    let mut gaps = q();
    gaps["options"]["expected_timestamps_utc_ms"] = json!([0, 300000, 600000]);
    for input in [
        r(),
        t("1970-01-01T09:15:00", Some("Asia/Kolkata")),
        t("1970-01-01T09:15:00", None),
        invalid,
        gaps,
    ] {
        let expected = normalize_v1(&input);
        for _ in 0..10 {
            assert_eq!(normalize_v1(&input), expected);
        }
    }
}
