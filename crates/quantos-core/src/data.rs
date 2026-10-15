//! Schema-1 normalization of an already parsed, in-memory synthetic/private table.

mod numbers;
mod time;
mod types;

pub use types::{
    AdjustmentStatusV1, GapAssessmentV1, IntervalV1, IssueCodeV1, IssueSeverityV1, IssueV1,
    NormalizationResultV1, NormalizationStatusV1, NormalizedCandleV1, ProvenanceV1,
    RedistributionStatusV1,
};

use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

const FIELDS: [&str; 10] = [
    "symbol",
    "exchange",
    "interval",
    "ts_utc_ms",
    "open",
    "high",
    "low",
    "close",
    "volume",
    "source",
];

fn description(code: IssueCodeV1) -> &'static str {
    use IssueCodeV1::*;
    match code {
        InvalidRequest => "The request property or record has the wrong shape or type.",
        UnsupportedSchemaVersion => "Only schema version 1 is supported.",
        InvalidOptions => "An option is missing, unknown, invalid or has the wrong type.",
        InvalidMetadata => "A provenance property is missing, invalid or has the wrong type.",
        InvalidExpectedGrid => {
            "The expected grid must contain increasing unique in-range integers."
        }
        EmptyDataset => "At least one data record is required.",
        InvalidHeader => "The headers must match the explicitly configured column set.",
        RowWidth => "The data record width must equal the header width.",
        InvalidIdentity => "Symbol and exchange must contain a non-whitespace character.",
        MixedDataset => "The identity must match the first nonempty value in its column.",
        UnsupportedInterval => "Only five-minute bar-open records are supported.",
        SourceMismatch => "The record source must match the declared dataset source.",
        InvalidTimestamp => "The timestamp must match the declared format and valid date rules.",
        TimezoneRequired => "A naive ISO timestamp requires an explicit timezone confirmation.",
        TimezoneConflict => "The timezone confirmation conflicts with the declared UTC meaning.",
        TimestampOutOfRange => {
            "The normalized UTC millisecond value is outside the supported range."
        }
        DuplicateTimestamp => "This validated opening repeats an earlier validated timestamp.",
        DescendingTimestamp => "This opening precedes the nearest earlier validated timestamp.",
        InvalidPrice => "A price must be finite plain decimal text.",
        PricePrecision => "A price may have at most two meaningful fractional digits.",
        PriceOutOfRange => {
            "A price must convert exactly to positive JavaScript-safe integer paise."
        }
        InvalidVolume => "Volume must be plain integer text.",
        VolumeOutOfRange => "Volume must be a nonnegative JavaScript-safe integer.",
        InvalidOhlc => "Open and close must lie between low and high.",
        MissingExpectedBar => "An explicitly expected opening is absent.",
    }
}

fn error(issues: &mut Vec<IssueV1>, code: IssueCodeV1, row: Option<u64>, field: Option<&str>) {
    issues.push(IssueV1 {
        code,
        severity: IssueSeverityV1::Error,
        row,
        field: field.map(str::to_owned),
        message: description(code).to_owned(),
    });
}

fn nonempty(text: &str) -> bool {
    !text
        .bytes()
        .all(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
}

#[derive(Default)]
struct Options {
    column: Option<String>,
    format: Option<time::Format>,
    zone: Option<time::Zone>,
    zone_valid: bool,
    grid: Option<Vec<u64>>,
}

fn grid_number(value: &Value) -> Option<u64> {
    if let Some(value) = value.as_u64() {
        return (value < numbers::UTC_END).then_some(value);
    }
    // JSON numbers can be stored as 0.0 or 1e0 as well as integer literals.
    // This conversion is only for grid numbers, never decimal CSV prices.
    let value = value.as_f64()?;
    ((0.0..numbers::UTC_END as f64).contains(&value) && value.fract() == 0.0)
        .then_some(value as u64)
}

fn options(root: &Map<String, Value>, issues: &mut Vec<IssueV1>) -> Options {
    let Some(map) = root.get("options").and_then(Value::as_object) else {
        error(issues, IssueCodeV1::InvalidRequest, None, Some("options"));
        return Options::default();
    };
    let names = [
        "timestamp_column",
        "timestamp_format",
        "confirmed_timezone",
        "expected_timestamps_utc_ms",
    ];
    for key in map.keys().filter(|key| !names.contains(&key.as_str())) {
        error(
            issues,
            IssueCodeV1::InvalidOptions,
            None,
            Some(&format!("options.{key}")),
        );
    }
    let mut out = Options::default();
    match map.get("timestamp_column").and_then(Value::as_str) {
        Some(name)
            if !name.is_empty()
                && !FIELDS
                    .iter()
                    .enumerate()
                    .any(|(i, field)| i != 3 && *field == name) =>
        {
            out.column = Some(name.to_owned());
        }
        _ => error(
            issues,
            IssueCodeV1::InvalidOptions,
            None,
            Some("options.timestamp_column"),
        ),
    }
    out.format = match map.get("timestamp_format").and_then(Value::as_str) {
        Some("utc_ms") => Some(time::Format::UtcMs),
        Some("iso8601") => Some(time::Format::Iso8601),
        _ => {
            error(
                issues,
                IssueCodeV1::InvalidOptions,
                None,
                Some("options.timestamp_format"),
            );
            None
        }
    };
    match map.get("confirmed_timezone") {
        Some(Value::Null) => out.zone_valid = true,
        Some(Value::String(zone)) if zone == "UTC" || zone == "Asia/Kolkata" => {
            out.zone_valid = true;
            out.zone = Some(if zone == "UTC" {
                time::Zone::Utc
            } else {
                time::Zone::Kolkata
            });
        }
        _ => error(
            issues,
            IssueCodeV1::InvalidOptions,
            None,
            Some("options.confirmed_timezone"),
        ),
    }
    match map.get("expected_timestamps_utc_ms") {
        None => error(
            issues,
            IssueCodeV1::InvalidOptions,
            None,
            Some("options.expected_timestamps_utc_ms"),
        ),
        Some(Value::Null) => {}
        Some(value) => {
            let values = value
                .as_array()
                .and_then(|array| array.iter().map(grid_number).collect::<Option<Vec<_>>>());
            match values {
                Some(values) if values.windows(2).all(|pair| pair[0] < pair[1]) => {
                    out.grid = Some(values)
                }
                _ => error(
                    issues,
                    IssueCodeV1::InvalidExpectedGrid,
                    None,
                    Some("options.expected_timestamps_utc_ms"),
                ),
            }
        }
    }
    if out.format == Some(time::Format::UtcMs) && out.zone == Some(time::Zone::Kolkata) {
        error(
            issues,
            IssueCodeV1::TimezoneConflict,
            None,
            Some("options.confirmed_timezone"),
        );
    }
    out
}

fn metadata(
    root: &Map<String, Value>,
    issues: &mut Vec<IssueV1>,
) -> (Option<ProvenanceV1>, Option<String>) {
    let Some(map) = root.get("metadata").and_then(Value::as_object) else {
        error(issues, IssueCodeV1::InvalidRequest, None, Some("metadata"));
        return (None, None);
    };
    let start = issues.len();
    let names = [
        "source",
        "synthetic",
        "adjustment_status",
        "redistribution_status",
    ];
    for key in map.keys().filter(|key| !names.contains(&key.as_str())) {
        error(
            issues,
            IssueCodeV1::InvalidMetadata,
            None,
            Some(&format!("metadata.{key}")),
        );
    }
    let source = map
        .get("source")
        .and_then(Value::as_str)
        .filter(|s| nonempty(s))
        .map(str::to_owned);
    if source.is_none() {
        error(
            issues,
            IssueCodeV1::InvalidMetadata,
            None,
            Some("metadata.source"),
        );
    }
    let synthetic = map.get("synthetic").and_then(Value::as_bool);
    if synthetic.is_none() {
        error(
            issues,
            IssueCodeV1::InvalidMetadata,
            None,
            Some("metadata.synthetic"),
        );
    }
    let adjustment = match map.get("adjustment_status").and_then(Value::as_str) {
        Some("unknown") => Some(AdjustmentStatusV1::Unknown),
        Some("unadjusted") => Some(AdjustmentStatusV1::Unadjusted),
        Some("split_adjusted") => Some(AdjustmentStatusV1::SplitAdjusted),
        Some("total_return_adjusted") => Some(AdjustmentStatusV1::TotalReturnAdjusted),
        _ => {
            error(
                issues,
                IssueCodeV1::InvalidMetadata,
                None,
                Some("metadata.adjustment_status"),
            );
            None
        }
    };
    let redistribution = match map.get("redistribution_status").and_then(Value::as_str) {
        Some("unknown") => Some(RedistributionStatusV1::Unknown),
        Some("private_only") => Some(RedistributionStatusV1::PrivateOnly),
        Some("permitted") => Some(RedistributionStatusV1::Permitted),
        _ => {
            error(
                issues,
                IssueCodeV1::InvalidMetadata,
                None,
                Some("metadata.redistribution_status"),
            );
            None
        }
    };
    let result = match (source.clone(), synthetic, adjustment, redistribution) {
        (Some(source), Some(synthetic), Some(adjustment_status), Some(redistribution_status))
            if issues.len() == start =>
        {
            Some(ProvenanceV1 {
                source,
                synthetic,
                adjustment_status,
                redistribution_status,
            })
        }
        _ => None,
    };
    (result, source)
}

struct Layout {
    indexes: [usize; 10],
}

fn layout(headers: &[Value], column: &str, issues: &mut Vec<IssueV1>) -> Option<Layout> {
    let expected: Vec<&str> = FIELDS
        .iter()
        .enumerate()
        .map(|(i, name)| if i == 3 { column } else { *name })
        .collect();
    let mut counts = BTreeMap::<&str, usize>::new();
    for name in headers.iter().filter_map(Value::as_str) {
        *counts.entry(name).or_default() += 1;
    }
    let all: BTreeSet<&str> = counts
        .keys()
        .copied()
        .chain(expected.iter().copied())
        .collect();
    let mut defects = Vec::new();
    for name in all {
        let count = counts.get(name).copied().unwrap_or(0);
        if count == 0 {
            defects.push(format!("missing header {name:?}"));
        }
        if count > 1 {
            defects.push(format!("duplicate header {name:?}"));
        }
        if !expected.contains(&name) {
            defects.push(format!("unexpected header {name:?}"));
        }
    }
    if !defects.is_empty() {
        error(issues, IssueCodeV1::InvalidHeader, Some(1), None);
        if let Some(issue) = issues.last_mut() {
            issue.message = format!("Invalid headers: {}.", defects.join("; "));
        }
        return None;
    }
    // All expected names occur exactly once; no unknown or missing names remain.
    let mut indexes = [0; 10];
    for (i, name) in expected.iter().enumerate() {
        indexes[i] = headers
            .iter()
            .position(|value| value.as_str() == Some(*name))?;
    }
    Some(Layout { indexes })
}

#[derive(Default)]
struct RowState {
    symbol: Option<String>,
    exchange: Option<String>,
    previous: Option<u64>,
    seen: BTreeSet<u64>,
}

fn identity(
    value: Option<&str>,
    first: &mut Option<String>,
    row: u64,
    field: &str,
    issues: &mut Vec<IssueV1>,
) {
    if let Some(value) = value {
        if !nonempty(value) {
            error(issues, IssueCodeV1::InvalidIdentity, Some(row), Some(field));
        } else if let Some(first) = first {
            if first.as_str() != value {
                error(issues, IssueCodeV1::MixedDataset, Some(row), Some(field));
            }
        } else {
            *first = Some(value.to_owned());
        }
    }
}

fn row_values(
    row: &[Value],
    number: u64,
    layout: &Layout,
    options: &Options,
    source: Option<&str>,
    state: &mut RowState,
    issues: &mut Vec<IssueV1>,
) -> Option<NormalizedCandleV1> {
    let values: [Option<&str>; 10] = std::array::from_fn(|i| row[layout.indexes[i]].as_str());
    identity(values[0], &mut state.symbol, number, "symbol", issues);
    identity(values[1], &mut state.exchange, number, "exchange", issues);
    if values[2].is_some_and(|value| value != "5m") {
        error(
            issues,
            IssueCodeV1::UnsupportedInterval,
            Some(number),
            Some("interval"),
        );
    }
    if source
        .zip(values[9])
        .is_some_and(|(declared, observed)| declared != observed)
    {
        error(
            issues,
            IssueCodeV1::SourceMismatch,
            Some(number),
            Some("source"),
        );
    }
    let timestamp = match values[3].zip(options.format) {
        Some((text, format)) => {
            match time::normalize(text, format, options.zone, options.zone_valid) {
                Ok(value) => value,
                Err(code) => {
                    error(issues, code, Some(number), options.column.as_deref());
                    None
                }
            }
        }
        None => None,
    };
    if let Some(timestamp) = timestamp {
        if !state.seen.insert(timestamp) {
            error(
                issues,
                IssueCodeV1::DuplicateTimestamp,
                Some(number),
                options.column.as_deref(),
            );
        }
        if state.previous.is_some_and(|previous| timestamp < previous) {
            error(
                issues,
                IssueCodeV1::DescendingTimestamp,
                Some(number),
                options.column.as_deref(),
            );
        }
        state.previous = Some(timestamp);
    }
    let prices: [Option<u64>; 4] = std::array::from_fn(|i| {
        values[i + 4].and_then(|value| match numbers::price(value) {
            Ok(value) => Some(value),
            Err(code) => {
                error(issues, code, Some(number), Some(FIELDS[i + 4]));
                None
            }
        })
    });
    if let [Some(open), Some(high), Some(low), Some(close)] = prices
        && !(low <= open && open <= high && low <= close && close <= high)
    {
        error(issues, IssueCodeV1::InvalidOhlc, Some(number), None);
    }
    let volume = values[8].and_then(|value| match numbers::volume(value) {
        Ok(value) => Some(value),
        Err(code) => {
            error(issues, code, Some(number), Some("volume"));
            None
        }
    });
    let [Some(open), Some(high), Some(low), Some(close)] = prices else {
        return None;
    };
    Some(NormalizedCandleV1 {
        symbol: values[0]?.to_owned(),
        exchange: values[1]?.to_owned(),
        interval: IntervalV1::FiveMinutes,
        ts_utc_ms: timestamp?,
        open,
        high,
        low,
        close,
        volume: volume?,
        source: values[9]?.to_owned(),
    })
}

fn field_order<'a>(field: Option<&'a str>, column: Option<&str>) -> (usize, &'a str) {
    let Some(field) = field else {
        return (11, "");
    };
    if Some(field) == column {
        return (3, "");
    }
    match FIELDS.iter().position(|name| *name == field) {
        Some(i) if i != 3 => (i, ""),
        _ => (10, field),
    }
}

fn finish(
    metadata: Option<ProvenanceV1>,
    mut candles: Vec<NormalizedCandleV1>,
    mut issues: Vec<IssueV1>,
    options: &Options,
) -> NormalizationResultV1 {
    issues.sort_by(|a, b| match (a.row, b.row) {
        (None, None) => a.code.cmp(&b.code).then(a.field.cmp(&b.field)),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a_row), Some(b_row)) => a_row
            .cmp(&b_row)
            .then_with(|| {
                field_order(a.field.as_deref(), options.column.as_deref())
                    .cmp(&field_order(b.field.as_deref(), options.column.as_deref()))
            })
            .then(a.code.cmp(&b.code)),
    });
    issues.dedup_by(|a, b| a.code == b.code && a.row == b.row && a.field == b.field);
    let failed = !issues.is_empty();
    let gap_assessment = if failed {
        candles.clear();
        GapAssessmentV1::NotPerformed {
            missing_ts_utc_ms: Vec::new(),
        }
    } else if let Some(grid) = &options.grid {
        let observed: BTreeSet<u64> = candles.iter().map(|candle| candle.ts_utc_ms).collect();
        let missing: Vec<u64> = grid
            .iter()
            .copied()
            .filter(|timestamp| !observed.contains(timestamp))
            .collect();
        for timestamp in &missing {
            issues.push(IssueV1 {
                code: IssueCodeV1::MissingExpectedBar,
                severity: IssueSeverityV1::Warning,
                row: None,
                field: None,
                message: format!("Missing expected opening at UTC millisecond {timestamp}."),
            });
        }
        GapAssessmentV1::Performed {
            missing_ts_utc_ms: missing,
        }
    } else {
        GapAssessmentV1::NotPerformed {
            missing_ts_utc_ms: Vec::new(),
        }
    };
    NormalizationResultV1 {
        schema_version: "1",
        status: if failed {
            NormalizationStatusV1::Failure
        } else {
            NormalizationStatusV1::Success
        },
        metadata,
        issues,
        gap_assessment,
        candles,
    }
}

/// Normalize an already parsed table without IO, floating-point price arithmetic,
/// host timezone access or partial acceptance. Invalid input returns a report.
pub fn normalize_v1(input: &Value) -> NormalizationResultV1 {
    let mut issues = Vec::new();
    let Some(root) = input.as_object() else {
        error(&mut issues, IssueCodeV1::InvalidRequest, None, None);
        return finish(None, Vec::new(), issues, &Options::default());
    };
    let names = ["schema_version", "headers", "rows", "metadata", "options"];
    for key in root.keys().filter(|key| !names.contains(&key.as_str())) {
        error(&mut issues, IssueCodeV1::InvalidRequest, None, Some(key));
    }
    match root.get("schema_version").and_then(Value::as_str) {
        Some("1") => {}
        Some(_) => error(
            &mut issues,
            IssueCodeV1::UnsupportedSchemaVersion,
            None,
            Some("schema_version"),
        ),
        None => error(
            &mut issues,
            IssueCodeV1::InvalidRequest,
            None,
            Some("schema_version"),
        ),
    }
    let options = options(root, &mut issues);
    let (metadata, source) = metadata(root, &mut issues);
    let header_array = root.get("headers").and_then(Value::as_array);
    let headers = header_array.filter(|values| values.iter().all(Value::is_string));
    if headers.is_none() {
        error(
            &mut issues,
            IssueCodeV1::InvalidRequest,
            None,
            Some("headers"),
        );
    }
    let layout = headers
        .zip(options.column.as_deref())
        .and_then(|(headers, column)| layout(headers, column, &mut issues));
    let mut candles = Vec::new();
    let mut state = RowState::default();
    match root.get("rows").and_then(Value::as_array) {
        None => error(&mut issues, IssueCodeV1::InvalidRequest, None, Some("rows")),
        Some(rows) => {
            if rows.is_empty() {
                error(&mut issues, IssueCodeV1::EmptyDataset, None, Some("rows"));
            }
            for (index, value) in rows.iter().enumerate() {
                let number = index as u64 + 2;
                let Some(row) = value.as_array() else {
                    error(&mut issues, IssueCodeV1::InvalidRequest, Some(number), None);
                    continue;
                };
                for (i, _) in row
                    .iter()
                    .enumerate()
                    .filter(|(_, value)| !value.is_string())
                {
                    let field = header_array
                        .and_then(|headers| headers.get(i))
                        .and_then(Value::as_str);
                    error(
                        &mut issues,
                        IssueCodeV1::InvalidRequest,
                        Some(number),
                        field,
                    );
                }
                if header_array.is_some_and(|headers| row.len() != headers.len()) {
                    error(&mut issues, IssueCodeV1::RowWidth, Some(number), None);
                    continue;
                }
                if let Some(layout) = &layout
                    && let Some(candle) = row_values(
                        row,
                        number,
                        layout,
                        &options,
                        source.as_deref(),
                        &mut state,
                        &mut issues,
                    )
                {
                    candles.push(candle);
                }
            }
        }
    }
    finish(metadata, candles, issues, &options)
}
