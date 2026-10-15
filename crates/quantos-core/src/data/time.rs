use super::{IssueCodeV1, numbers};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Format {
    UtcMs,
    Iso8601,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Zone {
    Utc,
    Kolkata,
}

impl Zone {
    fn minutes(self) -> i32 {
        match self {
            Self::Utc => 0,
            Self::Kolkata => 330,
        }
    }
}

fn number(bytes: &[u8]) -> Option<u32> {
    bytes.iter().try_fold(0, |n, b| {
        b.is_ascii_digit().then(|| n * 10 + u32::from(b - b'0'))
    })
}

fn leap(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn month_days(year: u32, month: u32) -> u32 {
    match month {
        2 if leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn days_before_year(year: i64) -> i64 {
    let previous = year - 1;
    year * 365 + previous.div_euclid(4) - previous.div_euclid(100) + previous.div_euclid(400) + 1
}

struct IsoTime {
    local_ms: i64,
    offset: Option<i32>,
}

fn iso(text: &str) -> Option<IsoTime> {
    let b = text.as_bytes();
    if !text.is_ascii()
        || b.len() < 19
        || (b[4], b[7], b[10], b[13], b[16]) != (b'-', b'-', b'T', b':', b':')
    {
        return None;
    }
    let year = number(&b[..4])?;
    let month = number(&b[5..7])?;
    let day = number(&b[8..10])?;
    let hour = number(&b[11..13])?;
    let minute = number(&b[14..16])?;
    let second = number(&b[17..19])?;
    if !(1..=12).contains(&month)
        || day == 0
        || day > month_days(year, month)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }
    let mut at = 19;
    let mut millis = 0;
    if b.get(at) == Some(&b'.') {
        at += 1;
        let start = at;
        while b.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        let count = at - start;
        if !(1..=3).contains(&count) {
            return None;
        }
        millis = number(&b[start..at])? * 10_u32.pow(3 - count as u32);
    }
    let offset = match b.get(at) {
        None => None,
        Some(b'Z') if at + 1 == b.len() => Some(0),
        Some(sign @ (b'+' | b'-')) if at + 6 == b.len() && b[at + 3] == b':' => {
            let hours = number(&b[at + 1..at + 3])?;
            let minutes = number(&b[at + 4..at + 6])?;
            if hours > 23 || minutes > 59 {
                return None;
            }
            let magnitude = (hours * 60 + minutes) as i32;
            Some(if *sign == b'-' { -magnitude } else { magnitude })
        }
        _ => return None,
    };
    let days = days_before_year(i64::from(year)) - days_before_year(1970)
        + (1..month)
            .map(|m| i64::from(month_days(year, m)))
            .sum::<i64>()
        + i64::from(day - 1);
    let local_ms = days * 86_400_000
        + i64::from(hour * 3600 + minute * 60 + second) * 1000
        + i64::from(millis);
    Some(IsoTime { local_ms, offset })
}

pub(super) fn normalize(
    text: &str,
    format: Format,
    confirmation: Option<Zone>,
    confirmation_valid: bool,
) -> Result<Option<u64>, IssueCodeV1> {
    if format == Format::UtcMs {
        return numbers::utc_ms(text).map(Some);
    }
    let value = iso(text).ok_or(IssueCodeV1::InvalidTimestamp)?;
    let offset = match value.offset {
        Some(offset) => {
            if confirmation_valid && confirmation.is_some_and(|zone| zone.minutes() != offset) {
                return Err(IssueCodeV1::TimezoneConflict);
            }
            offset
        }
        None if !confirmation_valid => return Ok(None),
        None => confirmation.ok_or(IssueCodeV1::TimezoneRequired)?.minutes(),
    };
    let utc = value.local_ms - i64::from(offset) * 60_000;
    let valid = u64::try_from(utc)
        .ok()
        .filter(|value| *value < numbers::UTC_END);
    valid.map(Some).ok_or(IssueCodeV1::TimestampOutOfRange)
}
