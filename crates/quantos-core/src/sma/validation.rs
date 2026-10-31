use super::{Calc, MAX_SAFE_INTEGER, SmaErrorCodeV1, SmaInputV1, error};

fn nonempty(value: &str) -> bool {
    value
        .chars()
        .any(|c| !matches!(c, ' ' | '\t' | '\r' | '\n'))
}

pub(super) fn validate(input: &SmaInputV1) -> Calc<()> {
    if input.schema_version != "1" {
        return Err(error(SmaErrorCodeV1::InvalidInput, None, "schema_version"));
    }
    // Completion/interval/provenance types have no invalid constructible variants.
    for (field, period, min, max) in [
        ("fast_period", input.fast_period, 1, 9_999),
        ("slow_period", input.slow_period, 2, 10_000),
    ] {
        if !(min..=max).contains(&period) {
            return Err(error(SmaErrorCodeV1::InvalidConfig, None, field));
        }
    }
    if input.fast_period >= input.slow_period {
        return Err(error(SmaErrorCodeV1::InvalidConfig, None, "periods"));
    }
    if !nonempty(&input.metadata.source) {
        return Err(error(
            SmaErrorCodeV1::InvalidMetadata,
            None,
            "metadata.source",
        ));
    }
    if input.candles.is_empty() || input.candles.len() as u128 > u128::from(MAX_SAFE_INTEGER) {
        return Err(error(SmaErrorCodeV1::InvalidInput, None, "candles"));
    }
    let first = &input.candles[0];
    for (i, candle) in input.candles.iter().enumerate() {
        let invalid = |field: &str| {
            error(
                SmaErrorCodeV1::InvalidCandle,
                Some(i as u64),
                &format!("candles[{i}].{field}"),
            )
        };
        if !nonempty(&candle.symbol) || candle.symbol != first.symbol {
            return Err(invalid("symbol"));
        }
        if !nonempty(&candle.exchange) || candle.exchange != first.exchange {
            return Err(invalid("exchange"));
        }
        if candle.source != input.metadata.source {
            return Err(invalid("source"));
        }
        if candle.ts_utc_ms >= 4_102_444_800_000
            || (i > 0
                && candle.ts_utc_ms.checked_sub(input.candles[i - 1].ts_utc_ms) != Some(300_000))
        {
            return Err(invalid("ts_utc_ms"));
        }
        for (field, price) in [
            ("open", candle.open),
            ("high", candle.high),
            ("low", candle.low),
            ("close", candle.close),
        ] {
            if !(1..=MAX_SAFE_INTEGER).contains(&price) {
                return Err(invalid(field));
            }
        }
        if !(candle.low <= candle.open
            && candle.open <= candle.high
            && candle.low <= candle.close
            && candle.close <= candle.high)
        {
            return Err(invalid("ohlc"));
        }
        if candle.volume > MAX_SAFE_INTEGER {
            return Err(invalid("volume"));
        }
    }
    Ok(())
}
