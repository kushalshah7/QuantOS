use super::{Calc, MAX_SAFE_INTEGER, SmaErrorCodeV1, SmaInputV1, error};
use crate::execution::SignalDecisionV1;
use std::cmp::Ordering;

const MAX_PRODUCT: u128 = 10_000 * 10_000 * MAX_SAFE_INTEGER as u128;

fn update_sum(sum: u128, input: &SmaInputV1, i: usize, period: usize, field: &str) -> Calc<u128> {
    let invalid = || error(SmaErrorCodeV1::ArithmeticOverflow, Some(i as u64), field);
    let retained = if i >= period {
        sum.checked_sub(input.candles[i - period].close.into())
            .ok_or_else(invalid)?
    } else {
        sum
    };
    let next = retained
        .checked_add(input.candles[i].close.into())
        .ok_or_else(invalid)?;
    let bound = (period as u128)
        .checked_mul(MAX_SAFE_INTEGER.into())
        .ok_or_else(invalid)?;
    if next > bound {
        return Err(invalid());
    }
    Ok(next)
}

fn product(sum: u128, period: u64, i: usize, field: &str) -> Calc<u128> {
    sum.checked_mul(period.into())
        .filter(|&value| value <= MAX_PRODUCT)
        .ok_or_else(|| error(SmaErrorCodeV1::ArithmeticOverflow, Some(i as u64), field))
}

pub(super) fn decisions(input: &SmaInputV1) -> Calc<Vec<SignalDecisionV1>> {
    // Validation bounds both periods to <=10000 before these index conversions.
    let fast = input.fast_period as usize;
    let slow = input.slow_period as usize;
    let mut fast_sum = 0;
    let mut slow_sum = 0;
    let mut previous = None;
    let mut decisions = Vec::with_capacity(input.candles.len());
    for i in 0..input.candles.len() {
        fast_sum = update_sum(fast_sum, input, i, fast, "fast_sum")?;
        slow_sum = update_sum(slow_sum, input, i, slow, "slow_sum")?;
        if i < slow - 1 {
            decisions.push(SignalDecisionV1::Hold);
            continue;
        }
        let left = product(fast_sum, input.slow_period, i, "fast_product")?;
        let right = product(slow_sum, input.fast_period, i, "slow_product")?;
        let current = left.cmp(&right);
        let decision = match (previous, current) {
            (Some(Ordering::Less | Ordering::Equal), Ordering::Greater) => {
                SignalDecisionV1::EnterLong
            }
            (Some(Ordering::Greater | Ordering::Equal), Ordering::Less) => {
                SignalDecisionV1::ExitLong
            }
            _ => SignalDecisionV1::Hold,
        };
        decisions.push(decision);
        previous = Some(current);
    }
    Ok(decisions)
}
