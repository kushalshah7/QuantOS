use super::{MAX_SAFE_INTEGER, arithmetic::*, types::*};

fn nonempty(value: &str) -> bool {
    value
        .chars()
        .any(|c| !matches!(c, ' ' | '\t' | '\r' | '\n'))
}

pub(super) fn validate(input: &ExecutionInputV1) -> Calc<()> {
    if input.schema_version != "1" {
        return Err(error(
            ExecutionErrorCodeV1::InvalidInput,
            None,
            "schema_version",
        ));
    }
    let c = &input.config;
    let fields = [
        (
            "initial_cash_paise",
            c.initial_cash_paise,
            1,
            MAX_SAFE_INTEGER,
        ),
        ("entry_quantity", c.entry_quantity, 1, MAX_SAFE_INTEGER),
        ("fee_fixed_paise", c.fee_fixed_paise, 0, MAX_SAFE_INTEGER),
        ("fee_bps", c.fee_bps, 0, 10_000),
        ("market_slippage_bps", c.market_slippage_bps, 0, 9_999),
        ("participation_bps", c.participation_bps, 0, 10_000),
    ];
    for (field, value, min, max) in fields {
        if !(min..=max).contains(&value) {
            return Err(error(
                ExecutionErrorCodeV1::InvalidConfig,
                None,
                &format!("config.{field}"),
            ));
        }
    }
    for (field, value, max) in [
        ("stop_loss_bps", c.stop_loss_bps, 9_999),
        ("take_profit_bps", c.take_profit_bps, 10_000),
    ] {
        if value.is_some_and(|v| !(1..=max).contains(&v)) {
            return Err(error(
                ExecutionErrorCodeV1::InvalidConfig,
                None,
                &format!("config.{field}"),
            ));
        }
    }
    if !nonempty(&input.metadata.source) {
        return Err(error(
            ExecutionErrorCodeV1::InvalidMetadata,
            None,
            "metadata.source",
        ));
    }
    if input.candles.is_empty() || input.candles.len() as u128 > u128::from(MAX_SAFE_INTEGER) {
        return Err(error(ExecutionErrorCodeV1::InvalidInput, None, "candles"));
    }
    if input.decisions.len() != input.candles.len() {
        return Err(error(ExecutionErrorCodeV1::InvalidInput, None, "decisions"));
    }
    let first = &input.candles[0];
    for (i, candle) in input.candles.iter().enumerate() {
        let invalid = |field: &str| {
            error(
                ExecutionErrorCodeV1::InvalidCandle,
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
