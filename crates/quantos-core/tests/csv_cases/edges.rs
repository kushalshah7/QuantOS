use super::support::*;
use quantos_core::data::{CsvParseCodeV1, CsvParseResultV1, parse_csv_v1};

#[test]
fn csv_edge_initial_bom_and_later_bom_preservation() {
    check(
        "CSV_EDGE_BOM_DATA",
        b"\xef\xbb\xbfa,b\n1,2",
        success(&["a", "b"], &[&["1", "2"]]),
    );
    check(
        "CSV_EDGE_TWO_BOMS",
        "\u{feff}\u{feff}a,b".as_bytes(),
        success(&["\u{feff}a", "b"], &[]),
    );
    check(
        "CSV_EDGE_LATER_BOM",
        "a,\u{feff}b\n\u{feff}v,w".as_bytes(),
        success(&["a", "\u{feff}b"], &[&["\u{feff}v", "w"]]),
    );
    check(
        "CSV_EDGE_NONINITIAL_BOM",
        " \u{feff}a,b".as_bytes(),
        success(&[" \u{feff}a", "b"], &[]),
    );
    check(
        "CSV_EDGE_BOM_OFFSET",
        b"\xef\xbb\xbf\"a\"x",
        failure("INVALID_AFTER_QUOTE", Some(1), 6),
    );
}

#[test]
fn csv_edge_original_multibyte_offsets() {
    check(
        "CSV_EDGE_UNICODE_QUOTE",
        "é\"".as_bytes(),
        failure("UNEXPECTED_QUOTE", Some(1), 2),
    );
    check(
        "CSV_EDGE_UNICODE_AFTER",
        "\"é\"x".as_bytes(),
        failure("INVALID_AFTER_QUOTE", Some(1), 4),
    );
    check(
        "CSV_EDGE_UNICODE_ERROR_BYTE",
        "\"é\"🙂".as_bytes(),
        failure("INVALID_AFTER_QUOTE", Some(1), 4),
    );
    check(
        "CSV_EDGE_BOM_UNICODE_OFFSET",
        "\u{feff}é\"".as_bytes(),
        failure("UNEXPECTED_QUOTE", Some(1), 5),
    );
}

#[test]
fn csv_edge_utf8_precedes_earlier_syntax_and_incomplete_encoding() {
    check(
        "CSV_EDGE_UTF8_PRECEDENCE",
        &[b'a', b'"', b'b', b',', 0xff],
        failure("INVALID_UTF8", None, 4),
    );
    check(
        "CSV_EDGE_UTF8_BEFORE_EOF_QUOTE",
        &[b'"', 0xff],
        failure("INVALID_UTF8", None, 1),
    );
    check(
        "CSV_EDGE_UTF8_INCOMPLETE",
        b"a,\xe2\x82",
        failure("INVALID_UTF8", None, 2),
    );
    check(
        "CSV_EDGE_UTF8_BAD_CONTINUATION",
        &[0xc3, 0x28],
        failure("INVALID_UTF8", None, 0),
    );
    check(
        "CSV_EDGE_UTF8_AFTER_BOM",
        &[0xef, 0xbb, 0xbf, b'a', b'\n', 0xff],
        failure("INVALID_UTF8", None, 5),
    );
}

#[test]
fn csv_edge_mixed_separators_and_quoted_cr_lf_are_exact() {
    check(
        "CSV_EDGE_MIXED",
        b"a,b\r\n1,2\n3,4\r\n",
        success(&["a", "b"], &[&["1", "2"], &["3", "4"]]),
    );
    check(
        "CSV_EDGE_EMBEDDED_CRLF_CR",
        b"\"a\r\nb\",h\n\"v\rw\",z",
        success(&["a\r\nb", "h"], &[&["v\rw", "z"]]),
    );
    check(
        "CSV_EDGE_BARE_CR_ROW",
        b"a\r\nb\rc",
        failure("INVALID_RECORD_SEPARATOR", Some(2), 4),
    );
    check(
        "CSV_EDGE_CR_AFTER_QUOTE",
        b"\"a\"\r",
        failure("INVALID_RECORD_SEPARATOR", Some(1), 3),
    );
}

#[test]
fn csv_edge_empties_header_only_blank_records_and_final_separators() {
    check("CSV_EDGE_QUOTED_EMPTY", b"\"\"", success(&[""], &[]));
    check("CSV_EDGE_HEADER_ONLY", b"a,b", success(&["a", "b"], &[]));
    check(
        "CSV_EDGE_TRAILING_EMPTY_DATA",
        b"a,b,\n1,2,",
        success(&["a", "b", ""], &[&["1", "2", ""]]),
    );
    check(
        "CSV_EDGE_ALL_EMPTY_FIELDS",
        b",,",
        success(&["", "", ""], &[]),
    );
    check(
        "CSV_EDGE_THREE_BLANKS",
        b"\n\n\n",
        success(&[""], &[&[""], &[""]]),
    );
    check(
        "CSV_EDGE_CRLF_BLANKS",
        b"\r\n\r\n",
        success(&[""], &[&[""]]),
    );
    check(
        "CSV_EDGE_FINAL_BLANK",
        b"a,b\n1,2\n\n",
        success(&["a", "b"], &[&["1", "2"], &[""]]),
    );
}

#[test]
fn csv_edge_escaped_quotes_and_spaces_have_no_special_privilege() {
    check(
        "CSV_EDGE_ESCAPED_QUOTE_ONLY",
        b"\"\"\"\"",
        success(&["\""], &[]),
    );
    check(
        "CSV_EDGE_TRIPLE_QUOTE_UNCLOSED",
        b"\"\"\"",
        failure("UNTERMINATED_QUOTED_FIELD", Some(1), 3),
    );
    check(
        "CSV_EDGE_SPACE_BEFORE_QUOTE",
        b" \"a\"",
        failure("UNEXPECTED_QUOTE", Some(1), 1),
    );
    check(
        "CSV_EDGE_SPACE_AFTER_QUOTE",
        b"\"a\" ",
        failure("INVALID_AFTER_QUOTE", Some(1), 3),
    );
    check(
        "CSV_EDGE_SPACE_AFTER_SECOND_FIELD",
        b"a,\"b\" \n",
        failure("INVALID_AFTER_QUOTE", Some(1), 5),
    );
    check(
        "CSV_EDGE_SPACES_PRESERVED",
        b" a , b \n 1 , 2 ",
        success(&[" a ", " b "], &[&[" 1 ", " 2 "]]),
    );
}

#[test]
fn csv_edge_multiline_fields_do_not_advance_record_numbers() {
    check(
        "CSV_EDGE_AFTER_MULTILINE",
        b"h\n\"a\nb\"x",
        failure("INVALID_AFTER_QUOTE", Some(2), 7),
    );
    check(
        "CSV_EDGE_MULTILINE_EOF",
        b"h\n\"a\nb",
        failure("UNTERMINATED_QUOTED_FIELD", Some(2), 6),
    );
    check(
        "CSV_EDGE_LATER_RECORD_NO_PARTIAL",
        b"a,b\n1,2\nbad\"x",
        failure("UNEXPECTED_QUOTE", Some(3), 11),
    );
    check(
        "CSV_EDGE_FIRST_SYNTAX",
        b"a\"b\rc",
        failure("UNEXPECTED_QUOTE", Some(1), 1),
    );
}

#[test]
fn csv_edge_no_trimming_unicode_normalization_or_nul_rejection() {
    check(
        "CSV_EDGE_NUL",
        b"a\0,b\n\0,x",
        success(&["a\0", "b"], &[&["\0", "x"]]),
    );
    check(
        "CSV_EDGE_QUOTED_NUL",
        b"\"a\0\",b",
        success(&["a\0", "b"], &[]),
    );
    check(
        "CSV_EDGE_UNICODE_FORMS",
        "é,e\u{301}\n🙂,汉字".as_bytes(),
        success(&["é", "e\u{301}"], &[&["🙂", "汉字"]]),
    );
    check(
        "CSV_EDGE_NO_SEPARATOR_DIRECTIVE",
        b"sep=;\na;b",
        success(&["sep=;"], &[&["a;b"]]),
    );
    check(
        "CSV_EDGE_NO_COMMENTS",
        b"#a,b\n1",
        success(&["#a", "b"], &[&["1"]]),
    );
}

#[test]
fn csv_edge_repeated_full_results_and_private_messages() {
    for input in [
        b"a,b\n1,2".as_slice(),
        b"\"SECRET".as_slice(),
        b"SECRET\"".as_slice(),
        b"\"SECRET\"x".as_slice(),
        b"a\rb".as_slice(),
        &[0xff],
    ] {
        let expected = parse_csv_v1(input);
        for _ in 0..10 {
            assert_eq!(parse_csv_v1(input), expected);
        }
        if let CsvParseResultV1::Failure { issue, .. } = expected {
            assert!(!issue.message.contains("SECRET"));
        }
    }
    let result = parse_csv_v1(b"\"SECRET\"x");
    assert!(matches!(
        result,
        CsvParseResultV1::Failure {
            issue: quantos_core::data::CsvParseIssueV1 {
                code: CsvParseCodeV1::InvalidAfterQuote,
                ..
            },
            ..
        }
    ));
}
