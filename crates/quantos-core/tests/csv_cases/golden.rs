use super::support::*;

#[test]
fn csv01_empty_input() {
    check("CSV01", b"", failure("EMPTY_INPUT", None, 0));
}
#[test]
fn csv02_bom_only() {
    check(
        "CSV02",
        &[0xef, 0xbb, 0xbf],
        failure("EMPTY_INPUT", None, 3),
    );
}
#[test]
fn csv03_lf_records() {
    check("CSV03", b"a,b\n1,2\n", success(&["a", "b"], &[&["1", "2"]]));
}
#[test]
fn csv04_crlf_records() {
    check(
        "CSV04",
        b"a,b\r\n1,2\r\n",
        success(&["a", "b"], &[&["1", "2"]]),
    );
}
#[test]
fn csv05_quoted_delimiters_escapes_and_multiline() {
    check(
        "CSV05",
        b"\"a,b\",\"x\"\"y\"\n\"v\nw\",z",
        success(&["a,b", "x\"y"], &[&["v\nw", "z"]]),
    );
}
#[test]
fn csv06_blank_data_record() {
    check("CSV06", b"a,b\n\n", success(&["a", "b"], &[&[""]]));
}
#[test]
fn csv07_trailing_header_field() {
    check("CSV07", b"a,", success(&["a", ""], &[]));
}
#[test]
fn csv08_unexpected_quote() {
    check("CSV08", b"a\"b", failure("UNEXPECTED_QUOTE", Some(1), 1));
}
#[test]
fn csv09_invalid_after_quote() {
    check(
        "CSV09",
        b"\"a\"x",
        failure("INVALID_AFTER_QUOTE", Some(1), 3),
    );
}
#[test]
fn csv10_unterminated_quote() {
    check(
        "CSV10",
        b"\"a",
        failure("UNTERMINATED_QUOTED_FIELD", Some(1), 2),
    );
}
#[test]
fn csv11_bare_cr() {
    check(
        "CSV11",
        b"a\rb",
        failure("INVALID_RECORD_SEPARATOR", Some(1), 1),
    );
}
#[test]
fn csv12_invalid_utf8() {
    check(
        "CSV12",
        &[0x61, 0x2c, 0xff],
        failure("INVALID_UTF8", None, 2),
    );
}
