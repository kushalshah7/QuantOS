use quantos_core::data::{NormalizationStatusV1, normalize_v1};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(super) fn metadata() -> Value {
    json!({"source":"synthetic","synthetic":true,"adjustment_status":"unknown","redistribution_status":"unknown"})
}

pub(super) fn r() -> Value {
    json!({
        "schema_version":"1",
        "headers":["symbol","exchange","interval","ts_utc_ms","open","high","low","close","volume","source"],
        "rows":[
            ["SYNTH","TEST","5m","0","100","103","99","102","10","synthetic"],
            ["SYNTH","TEST","5m","300000","102","104","101","103","12","synthetic"],
            ["SYNTH","TEST","5m","600000","103","105","100","101","8","synthetic"]
        ],
        "metadata":metadata(),
        "options":{"timestamp_column":"ts_utc_ms","timestamp_format":"utc_ms","confirmed_timezone":null,"expected_timestamps_utc_ms":null}
    })
}

pub(super) fn q() -> Value {
    let mut input = r();
    input["rows"].as_array_mut().unwrap().truncate(1);
    input
}

pub(super) fn t(text: &str, zone: Option<&str>) -> Value {
    let mut input = q();
    input["headers"][3] = json!("timestamp");
    input["rows"][0][3] = json!(text);
    input["options"]["timestamp_column"] = json!("timestamp");
    input["options"]["timestamp_format"] = json!("iso8601");
    input["options"]["confirmed_timezone"] = json!(zone);
    input
}

pub(super) fn c0() -> Value {
    json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":0,"open":10000,"high":10300,"low":9900,"close":10200,"volume":10,"source":"synthetic"})
}
pub(super) fn c1() -> Value {
    json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":300000,"open":10200,"high":10400,"low":10100,"close":10300,"volume":12,"source":"synthetic"})
}
pub(super) fn c2() -> Value {
    json!({"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":600000,"open":10300,"high":10500,"low":10000,"close":10100,"volume":8,"source":"synthetic"})
}
pub(super) fn candle(field: &str, value: Value) -> Value {
    let mut out = c0();
    out[field] = value;
    out
}
pub(super) fn issue(code: &str, row: Option<u64>, field: Option<&str>) -> Value {
    json!({"code":code,"severity":"error","row":row,"field":field})
}
pub(super) fn warning() -> Value {
    json!({"code":"MISSING_EXPECTED_BAR","severity":"warning","row":null,"field":null})
}
pub(super) fn success(candles: Vec<Value>) -> Value {
    json!({"schema_version":"1","status":"success","metadata":metadata(),"candles":candles,"issues":[],"gap_assessment":{"status":"not_performed","missing_ts_utc_ms":[]}})
}
pub(super) fn failure(issues: Vec<Value>) -> Value {
    json!({"schema_version":"1","status":"failure","metadata":metadata(),"candles":[],"issues":issues,"gap_assessment":{"status":"not_performed","missing_ts_utc_ms":[]}})
}
pub(super) fn f(code: &str, row: Option<u64>, field: Option<&str>) -> Value {
    failure(vec![issue(code, row, field)])
}
pub(super) fn check(id: &str, input: Value, expected: Value) {
    let result = normalize_v1(&input);
    let status = if expected["status"] == "success" {
        NormalizationStatusV1::Success
    } else {
        NormalizationStatusV1::Failure
    };
    assert_eq!(result.status, status, "{id}: typed Rust status");
    if result.status == NormalizationStatusV1::Failure {
        assert!(result.candles.is_empty(), "{id}: atomic failure");
    }
    let mut actual = serde_json::to_value(&result).unwrap();
    for issue in actual["issues"].as_array_mut().unwrap() {
        let object = issue.as_object_mut().unwrap();
        assert_eq!(
            object.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from(["code", "severity", "row", "field", "message"]),
            "{id}: exact issue shape"
        );
        assert!(
            object["message"]
                .as_str()
                .is_some_and(|message| !message.trim().is_empty()),
            "{id}: nonempty message"
        );
        object.remove("message");
    }
    assert_eq!(
        actual, expected,
        "{id}: complete serialized result and ordered issue keys"
    );
}
