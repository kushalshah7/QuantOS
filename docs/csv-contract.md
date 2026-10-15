# CSV byte parser — schema 1

Owner-approved QOS-0004 specification. This CSV-only interface produces exact
strings for the existing [normalization contract](data-contract.md). Syntactically
valid CSV is not a statement that its contents are valid market data. Parser and
normalizer issue codes remain separate. No combined import API is introduced.

## API and exact result shapes

Public path: `quantos_core::data::parse_csv_v1`.

```rust
pub fn parse_csv_v1(input: &[u8]) -> CsvParseResultV1;
```

Re-export `CsvParseResultV1`, `CsvParseIssueV1`, `CsvParseCodeV1` from data.
The result is a tagged Rust enum; use existing Serde dependencies and Serialize
only. Success serialization has exactly these required fields, with no issue:

```json
{"schema_version":"1","status":"success","headers":["..."],"rows":[["..."]]}
```

Failure has exactly these fields, with no headers, rows or partial table:

```json
{"schema_version":"1","status":"failure","issue":{"code":"INVALID_UTF8","row":null,"byte_offset":0,"message":"Input must be valid UTF-8."}}
```

`row` is null or an integer record number (header 1). `byte_offset` is a zero-based
integer position in the ORIGINAL byte slice, including any initial BOM. Rust
positions/counts use `usize`. Messages are nonempty, deterministic and do not echo
raw CSV contents; example wording is not an assertion key.

Closed `CsvParseCodeV1` strings:

```text
INVALID_UTF8
EMPTY_INPUT
UNEXPECTED_QUOTE
INVALID_AFTER_QUOTE
UNTERMINATED_QUOTED_FIELD
INVALID_RECORD_SEPARATOR
```

These six parser codes do not add to or alter the 25 normalizer codes.

## Locked parsing policies

1. Validate the entire byte slice as UTF-8 first. Any invalid encoding takes
   precedence over CSV syntax, even if later in the file. Return INVALID_UTF8,
   row null, at Rust `Utf8Error::valid_up_to()` in the original slice.
2. Remove exactly one UTF-8 BOM (EF BB BF), only at byte zero. Preserve all other
   BOM characters. Empty bytes or BOM-only input produce EMPTY_INPUT, row null,
   at offset equal to original input length.
3. Comma is the only delimiter. Outside quoted fields LF and CRLF are record
   separators; mixed use is allowed. A bare CR outside quotes produces
   INVALID_RECORD_SEPARATOR at that CR. No delimiter guessing, comments or
   spreadsheet separator directives.
4. A quoted field begins only when its first byte is a double quote. Inside it,
   doubled quotes decode to one quote. Every other character, including comma,
   CR, LF, NUL and Unicode, is preserved exactly. Never normalize embedded line
   endings.
5. A quote in an unquoted field produces UNEXPECTED_QUOTE at that quote. After a
   closing quote, only comma, a valid record separator or EOF is allowed. Any
   other byte produces INVALID_AFTER_QUOTE there, except bare CR produces
   INVALID_RECORD_SEPARATOR. Spaces before/after quotes have no special privilege.
6. EOF inside an unclosed quoted field produces UNTERMINATED_QUOTED_FIELD at
   offset `input.len()`. After UTF-8 validation report only the first CSV syntax
   error encountered in byte order; unclosed quotes are detected at EOF. No
   recovery or skipped records.
7. Preserve empty fields, including one after a trailing comma. A final LF/CRLF
   terminates the current record without inventing another record. Blank records
   remain one empty field. Consecutive separators represent consecutive blank
   records. A quoted empty field is valid.
8. First record is headers; remaining records are rows. Header-only input is
   parser success with empty rows. Header names, widths, identity, numeric/time
   rules and data-row presence belong to normalize_v1, not this parser.
9. Record numbering starts at 1 for headers and advances only on separators
   outside quotes. Embedded newlines never advance it. Syntax errors report the
   current record; encoding/empty errors report row null.
10. Preserve decoded fields exactly except quote unescaping and the one optional
    initial BOM. No trimming, case folding, numeric conversion, missing-value
    substitution or Unicode normalization.

Use a small linear state machine. No IO, networking, unsafe code, hidden
persistence or generic parsing framework. Failure discards every partial record.

## Golden parser examples

Inputs below use Rust byte-string escapes or explicit hexadecimal bytes.
Messages must describe the rule without echoing data; compare error code/row/
offset and exact serialized shape, not fixed message wording.

| ID | Input | Exact result |
| --- | --- | --- |
| CSV01 | `b""` | EMPTY_INPUT, row null, offset 0. |
| CSV02 | `[0xEF,0xBB,0xBF]` | EMPTY_INPUT, row null, offset 3. |
| CSV03 | `b"a,b\n1,2\n"` | headers `["a","b"]`, rows `[["1","2"]]`. |
| CSV04 | `b"a,b\r\n1,2\r\n"` | Same table as CSV03. |
| CSV05 | `b"\"a,b\",\"x\"\"y\"\n\"v\nw\",z"` | headers `["a,b","x\"y"]`, rows `[["v\nw","z"]]`. |
| CSV06 | `b"a,b\n\n"` | headers `["a","b"]`, rows `[[""]]`; retain blank record. |
| CSV07 | `b"a,"` | headers `["a",""]`, rows `[]`. |
| CSV08 | `b"a\"b"` | UNEXPECTED_QUOTE, row 1, offset 1. |
| CSV09 | `b"\"a\"x"` | INVALID_AFTER_QUOTE, row 1, offset 3. |
| CSV10 | `b"\"a"` | UNTERMINATED_QUOTED_FIELD, row 1, offset 2. |
| CSV11 | `b"a\rb"` | INVALID_RECORD_SEPARATOR, row 1, offset 1. |
| CSV12 | `[0x61,0x2C,0xFF]` | INVALID_UTF8, row null, offset 2. |

Additional acceptance covers initial/later BOM, multibyte-byte offsets, UTF-8
precedence, mixed separators, exact quoted CRLF/bare CR, escaped quotes, trailing
empties, quoted-empty/header-only and blank records, spaces around quotes, errors
after multiline fields, full serialization and identical repeated results/messages.
Expectations are written independently, not generated by production helpers.

## Test-only normalization pipeline

New integration tests may match a parse success and construct the existing
normalize_v1 request with explicit schema_version, metadata and options. This
composition remains test-only; it is not a new public combined-import entry point.
Reuse existing read-only synthetic fixtures with `include_bytes!`:

- `valid-candles.csv`: exact three normalized candles.
- `invalid-ohlc.csv`: parser success, atomic INVALID_OHLC.
- `duplicate-timestamp.csv`: parser success, atomic DUPLICATE_TIMESTAMP.
- `missing-timezone.csv`: explicit `timestamp` mapping and ISO format; null
  confirmation produces TIMEZONE_REQUIRED; confirmed Asia/Kolkata gives 13500000.
- Quoted canonical fields normalize identically; multiline source stays exact.
  A blank data record is retained and becomes ROW_WIDTH at record 2.
- Malformed CSV has no partial table; do not send a parser failure to normalization.

## Verification limits

Local tests/builds/compilation are NOT RUN in LOW-RESOURCE mode. Existing pinned
rustfmt and lightweight text/Git diff inspection are permitted; no installation.
Final-head existing CI must execute new CSV/pipeline tests and unchanged
normalization/P0 suites, and actual output must be inspected. WASM target evidence
is compilation only; macOS and browser/native-WASM runtime parity NOT RUN.
No CLI/browser integration, dependency/workflow change or complete-P1 acceptance.
