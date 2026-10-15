use quantos_core::data::{CsvParseResultV1, parse_csv_v1};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(super) fn success(headers: &[&str], rows: &[&[&str]]) -> Value {
    json!({"schema_version":"1","status":"success","headers":headers,"rows":rows})
}

pub(super) fn failure(code: &str, row: Option<usize>, offset: usize) -> Value {
    json!({"schema_version":"1","status":"failure","issue":{"code":code,"row":row,"byte_offset":offset}})
}

pub(super) fn check(id: &str, input: &[u8], expected: Value) {
    let result = parse_csv_v1(input);
    assert_eq!(
        matches!(result, CsvParseResultV1::Success { .. }),
        expected["status"] == "success",
        "{id}: tagged Rust variant"
    );
    let mut actual = serde_json::to_value(&result).unwrap();
    if let Some(issue) = actual.get_mut("issue") {
        let object = issue.as_object_mut().unwrap();
        assert_eq!(
            object.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from(["code", "row", "byte_offset", "message"]),
            "{id}: exact issue shape"
        );
        assert!(
            object["message"]
                .as_str()
                .is_some_and(|text| !text.trim().is_empty()),
            "{id}: nonempty descriptive message"
        );
        object.remove("message");
    }
    assert_eq!(
        actual, expected,
        "{id}: complete serialized union and exact strings/offsets/record number"
    );
}
