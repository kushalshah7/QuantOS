use super::{MAX_SAFE_INTEGER, types::*};

pub(super) type Calc<T> = Result<T, ExecutionErrorV1>;

pub(super) fn error(
    code: ExecutionErrorCodeV1,
    bar_index: Option<u64>,
    field: &str,
) -> ExecutionErrorV1 {
    let message = match code {
        ExecutionErrorCodeV1::InvalidInput => "Invalid execution input.",
        ExecutionErrorCodeV1::InvalidConfig => "Invalid execution configuration.",
        ExecutionErrorCodeV1::InvalidMetadata => "Invalid execution provenance.",
        ExecutionErrorCodeV1::InvalidCandle => "Invalid normalized candle.",
        ExecutionErrorCodeV1::ArithmeticOverflow => "Unsafe execution arithmetic.",
        ExecutionErrorCodeV1::InvalidExecutionPrice => {
            "Computed execution price is outside the allowed range."
        }
        ExecutionErrorCodeV1::ReconciliationFailure => "Execution ledger does not reconcile.",
    };
    ExecutionErrorV1 {
        code,
        bar_index,
        field: Some(field.to_owned()),
        message,
    }
}

#[derive(Clone, Copy)]
pub(super) struct Context(pub usize);

impl Context {
    pub fn fail(self, code: ExecutionErrorCodeV1, field: &str) -> ExecutionErrorV1 {
        // Validation bounds the slice length and indices before execution.
        error(code, Some(self.0 as u64), field)
    }

    pub fn amount(self, value: i128, field: &str) -> Calc<u64> {
        if !(0..=i128::from(MAX_SAFE_INTEGER)).contains(&value) {
            return Err(self.fail(ExecutionErrorCodeV1::ArithmeticOverflow, field));
        }
        Ok(value as u64)
    }

    pub fn signed(self, value: i128, field: &str) -> Calc<i64> {
        if !(-i128::from(MAX_SAFE_INTEGER)..=i128::from(MAX_SAFE_INTEGER)).contains(&value) {
            return Err(self.fail(ExecutionErrorCodeV1::ArithmeticOverflow, field));
        }
        Ok(value as i64)
    }

    pub fn price(self, value: i128, field: &str) -> Calc<u64> {
        if !(1..=i128::from(MAX_SAFE_INTEGER)).contains(&value) {
            return Err(self.fail(ExecutionErrorCodeV1::InvalidExecutionPrice, field));
        }
        Ok(value as u64)
    }

    pub fn add(self, a: i128, b: i128, field: &str) -> Calc<i128> {
        a.checked_add(b)
            .ok_or_else(|| self.fail(ExecutionErrorCodeV1::ArithmeticOverflow, field))
    }

    pub fn mul(self, a: i128, b: i128, field: &str) -> Calc<i128> {
        a.checked_mul(b)
            .ok_or_else(|| self.fail(ExecutionErrorCodeV1::ArithmeticOverflow, field))
    }

    pub fn ceil_bps(self, numerator: i128, field: &str) -> Calc<i128> {
        Ok(self.add(numerator, 9_999, field)? / 10_000)
    }
}

pub(super) struct Quote {
    pub price: u64,
    pub notional: u64,
    pub fee: u64,
}

pub(super) fn quote(
    ctx: Context,
    side: ExecutionSideV1,
    reference: u64,
    quantity: u64,
    limit: bool,
    config: &ExecutionConfigV1,
) -> Calc<Quote> {
    let price = if limit {
        reference
    } else {
        let factor = match side {
            ExecutionSideV1::Buy => 10_000 + config.market_slippage_bps,
            ExecutionSideV1::Sell => 10_000 - config.market_slippage_bps,
        };
        let numerator = ctx.mul(reference.into(), factor.into(), "fill_price_paise")?;
        let value = match side {
            ExecutionSideV1::Buy => ctx.ceil_bps(numerator, "fill_price_paise")?,
            ExecutionSideV1::Sell => numerator / 10_000,
        };
        ctx.price(value, "fill_price_paise")?
    };
    let notional = ctx.amount(
        ctx.mul(price.into(), quantity.into(), "notional_paise")?,
        "notional_paise",
    )?;
    let rate_fee = ctx.ceil_bps(
        ctx.mul(notional.into(), config.fee_bps.into(), "fee_paise")?,
        "fee_paise",
    )?;
    let fee = ctx.amount(
        ctx.add(config.fee_fixed_paise.into(), rate_fee, "fee_paise")?,
        "fee_paise",
    )?;
    Ok(Quote {
        price,
        notional,
        fee,
    })
}

pub(super) fn brackets(
    ctx: Context,
    price: u64,
    config: &ExecutionConfigV1,
) -> Calc<(Option<u64>, Option<u64>)> {
    let stop = config
        .stop_loss_bps
        .map(|bps| {
            let n = ctx.mul(price.into(), (10_000 - bps).into(), "stop_price_paise")?;
            ctx.price(n / 10_000, "stop_price_paise")
        })
        .transpose()?;
    let target = config
        .take_profit_bps
        .map(|bps| {
            let n = ctx.mul(price.into(), (10_000 + bps).into(), "target_price_paise")?;
            ctx.price(ctx.ceil_bps(n, "target_price_paise")?, "target_price_paise")
        })
        .transpose()?;
    Ok((stop, target))
}
