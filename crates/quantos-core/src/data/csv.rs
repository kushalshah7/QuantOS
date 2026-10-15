//! Strict, in-memory UTF-8 CSV parsing; domain validation remains separate.

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CsvParseCodeV1 {
    InvalidUtf8,
    EmptyInput,
    UnexpectedQuote,
    InvalidAfterQuote,
    UnterminatedQuotedField,
    InvalidRecordSeparator,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CsvParseIssueV1 {
    pub code: CsvParseCodeV1,
    pub row: Option<usize>,
    pub byte_offset: usize,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CsvParseResultV1 {
    Success {
        schema_version: &'static str,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Failure {
        schema_version: &'static str,
        issue: CsvParseIssueV1,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    Unquoted,
    Quoted,
    AfterQuote,
}

fn failure(code: CsvParseCodeV1, row: Option<usize>, byte_offset: usize) -> CsvParseResultV1 {
    let message = match code {
        CsvParseCodeV1::InvalidUtf8 => "Input must be valid UTF-8.",
        CsvParseCodeV1::EmptyInput => "CSV input must contain a record.",
        CsvParseCodeV1::UnexpectedQuote => "A quote may open a field only at its first byte.",
        CsvParseCodeV1::InvalidAfterQuote => {
            "Only a delimiter, record separator or EOF may follow a closing quote."
        }
        CsvParseCodeV1::UnterminatedQuotedField => "A quoted field must close before EOF.",
        CsvParseCodeV1::InvalidRecordSeparator => "Outside quotes a CR must be followed by LF.",
    };
    CsvParseResultV1::Failure {
        schema_version: "1",
        issue: CsvParseIssueV1 {
            code,
            row,
            byte_offset,
            message: message.to_owned(),
        },
    }
}

/// Parse exact field strings without IO or domain validation. Encoding errors
/// precede syntax errors, whose offsets always refer to the original byte slice.
pub fn parse_csv_v1(input: &[u8]) -> CsvParseResultV1 {
    let text = match std::str::from_utf8(input) {
        Ok(text) => text,
        Err(error) => return failure(CsvParseCodeV1::InvalidUtf8, None, error.valid_up_to()),
    };
    let (text, base) = match text.strip_prefix('\u{feff}') {
        Some(body) => (body, 3),
        None => (text, 0),
    };
    if text.is_empty() {
        return failure(CsvParseCodeV1::EmptyInput, None, input.len());
    }
    let mut chars = text.char_indices().peekable();
    let mut state = State::Start;
    let mut field = String::new();
    let mut record = Vec::new();
    let mut table = Vec::new();
    let mut active = false;
    let mut row = 1;
    while let Some((position, ch)) = chars.next() {
        let offset = base + position;
        if state == State::Quoted {
            if ch == '"' {
                state = State::AfterQuote;
            } else {
                field.push(ch);
            }
            continue;
        }
        if state == State::AfterQuote && ch == '"' {
            // The preceding quote was the first half of an escape, not a close.
            field.push('"');
            state = State::Quoted;
            continue;
        }
        match ch {
            ',' => {
                record.push(std::mem::take(&mut field));
                state = State::Start;
                active = true;
            }
            '\r' | '\n' => {
                if ch == '\r' {
                    if chars.peek().map(|(_, next)| *next) != Some('\n') {
                        return failure(CsvParseCodeV1::InvalidRecordSeparator, Some(row), offset);
                    }
                    chars.next();
                }
                record.push(std::mem::take(&mut field));
                table.push(std::mem::take(&mut record));
                state = State::Start;
                active = false;
                row += 1;
            }
            '"' if state == State::Start => {
                state = State::Quoted;
                active = true;
            }
            '"' => return failure(CsvParseCodeV1::UnexpectedQuote, Some(row), offset),
            _ if state == State::AfterQuote => {
                return failure(CsvParseCodeV1::InvalidAfterQuote, Some(row), offset);
            }
            _ => {
                field.push(ch);
                state = State::Unquoted;
                active = true;
            }
        }
    }
    if state == State::Quoted {
        return failure(
            CsvParseCodeV1::UnterminatedQuotedField,
            Some(row),
            input.len(),
        );
    }
    if active {
        record.push(field);
        table.push(record);
    }
    // Nonempty valid CSV has at least one record, including a blank/quoted one.
    let mut records = table.into_iter();
    CsvParseResultV1::Success {
        schema_version: "1",
        headers: records.next().unwrap_or_default(),
        rows: records.collect(),
    }
}
