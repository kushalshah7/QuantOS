use super::IssueCodeV1;

pub(super) const MAX_SAFE: u64 = 9_007_199_254_740_991;
pub(super) const UTC_END: u64 = 4_102_444_800_000;

fn digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

// Callers validate the entire lexeme before bounded accumulation. Large valid
// numbers stay range errors, while a malformed suffix still wins over overflow.
fn bounded(text: &str) -> u64 {
    text.bytes().fold(0, |n, b| {
        n.saturating_mul(10)
            .saturating_add(u64::from(b - b'0'))
            .min(MAX_SAFE + 1)
    })
}

fn integer(text: &str) -> Option<(bool, u64)> {
    let negative = text.starts_with('-');
    let body = text.strip_prefix('-').unwrap_or(text);
    digits(body).then(|| (negative, bounded(body)))
}

pub(super) fn utc_ms(text: &str) -> Result<u64, IssueCodeV1> {
    let (negative, value) = integer(text).ok_or(IssueCodeV1::InvalidTimestamp)?;
    if (negative && value != 0) || value >= UTC_END {
        Err(IssueCodeV1::TimestampOutOfRange)
    } else {
        Ok(value)
    }
}

pub(super) fn volume(text: &str) -> Result<u64, IssueCodeV1> {
    let (negative, value) = integer(text).ok_or(IssueCodeV1::InvalidVolume)?;
    if (negative && value != 0) || value > MAX_SAFE {
        Err(IssueCodeV1::VolumeOutOfRange)
    } else {
        Ok(value)
    }
}

pub(super) fn price(text: &str) -> Result<u64, IssueCodeV1> {
    let negative = text.starts_with('-');
    let body = text.strip_prefix('-').unwrap_or(text);
    let (whole, fraction) = match body.split_once('.') {
        Some((whole, fraction)) if digits(whole) && digits(fraction) => {
            (whole, fraction.trim_end_matches('0'))
        }
        None if digits(body) => (body, ""),
        _ => return Err(IssueCodeV1::InvalidPrice),
    };
    if fraction.len() > 2 {
        return Err(IssueCodeV1::PricePrecision);
    }
    let fractional_paise = fraction
        .bytes()
        .enumerate()
        .map(|(i, b)| u64::from(b - b'0') * if i == 0 { 10 } else { 1 })
        .sum::<u64>();
    let paise = bounded(whole) * 100 + fractional_paise;
    if negative || paise == 0 || paise > MAX_SAFE {
        Err(IssueCodeV1::PriceOutOfRange)
    } else {
        Ok(paise)
    }
}
