use super::support::*;
use quantos_core::data::{AdjustmentStatusV1, RedistributionStatusV1};
use quantos_core::execution::SignalDecisionV1;
use quantos_core::sma::*;
use std::cmp::Ordering;

// Test-only direct-window reference: sum each current/previous slice afresh,
// without rolling recurrence, saved relation or any production helper.
fn direct_windows(i: &SmaInputV1) -> Vec<SignalDecisionV1> {
    let fast = i.fast_period as usize;
    let slow = i.slow_period as usize;
    let order = |end: usize| {
        let a = i.candles[end + 1 - fast..=end]
            .iter()
            .map(|c| u128::from(c.close))
            .sum::<u128>();
        let b = i.candles[end + 1 - slow..=end]
            .iter()
            .map(|c| u128::from(c.close))
            .sum::<u128>();
        (a * u128::from(i.slow_period)).cmp(&(b * u128::from(i.fast_period)))
    };
    (0..i.candles.len())
        .map(|index| {
            if index < slow {
                return H;
            }
            let before = order(index - 1);
            let now = order(index);
            if before != Ordering::Greater && now == Ordering::Greater {
                E
            } else if before != Ordering::Less && now == Ordering::Less {
                X
            } else {
                H
            }
        })
        .collect()
}

#[test]
fn rolling_matches_independent_direct_windows_on_all_small_patterns() {
    // 3^6 patterns, 15 period pairs: a bounded correctness check, not a benchmark.
    for seed in 0..729_u32 {
        let mut value = seed;
        let mut closes = [0_u64; 6];
        for c in &mut closes {
            *c = [1, 2, 5][(value % 3) as usize];
            value /= 3;
        }
        for slow in 2..=6 {
            for fast in 1..slow {
                let i = input(fast, slow, &closes);
                assert_eq!(
                    report(&i).decisions,
                    direct_windows(&i),
                    "seed={seed}, f={fast}, s={slow}"
                );
            }
        }
    }
    for (fast, slow, closes) in [
        (1, 2, vec![M, M - 1, M, M - 1]),
        (2, 3, vec![M - 2, M, M - 1, M - 2, M]),
    ] {
        let i = input(fast, slow, &closes);
        assert_eq!(report(&i).decisions, direct_windows(&i));
    }
}

#[test]
fn sc12_every_valid_prefix_and_future_mutation_preserves_decisions() {
    let i = input(2, 3, &[300, 200, 100, 200, 300, 200, 100, 100]);
    let baseline = report(&i);
    for n in 1..=i.candles.len() {
        let mut prefix = i.clone();
        prefix.candles.truncate(n);
        assert_eq!(report(&prefix).decisions, baseline.decisions[..n]);
    }
    let mut changed = i.clone();
    let c = &mut changed.candles[7];
    c.open = 400;
    c.high = 400;
    c.low = 400;
    c.close = 400;
    c.volume = 0;
    let r = report(&changed);
    assert_eq!(r.decisions, vec![H, H, H, H, E, H, X, E]);
    assert_eq!(r.decisions[..7], baseline.decisions[..7]);
}

#[test]
fn valid_non_close_mutations_do_not_change_decisions() {
    let i = input(2, 3, &[300, 200, 100, 200, 300, 200, 100, 100]);
    let baseline = report(&i).decisions;
    let mut changed = i.clone();
    changed.metadata.source = " स्रोत\u{a0}".into();
    changed.metadata.synthetic = false;
    changed.metadata.adjustment_status = AdjustmentStatusV1::SplitAdjusted;
    changed.metadata.redistribution_status = RedistributionStatusV1::PrivateOnly;
    for (n, c) in changed.candles.iter_mut().enumerate() {
        c.open = c.close + 50;
        c.high = c.close + 100;
        c.low = 1;
        c.volume = if n % 2 == 0 { 0 } else { M };
        c.ts_utc_ms += 17;
        c.symbol = "\u{a0}".into();
        c.exchange = "EXACT LABEL ".into();
        c.source = changed.metadata.source.clone();
    }
    assert_eq!(report(&changed).decisions, baseline);
}

#[test]
fn all_constructible_provenance_declarations_are_preserved() {
    for adjustment in [
        AdjustmentStatusV1::Unknown,
        AdjustmentStatusV1::Unadjusted,
        AdjustmentStatusV1::SplitAdjusted,
        AdjustmentStatusV1::TotalReturnAdjusted,
    ] {
        for redistribution in [
            RedistributionStatusV1::Unknown,
            RedistributionStatusV1::PrivateOnly,
            RedistributionStatusV1::Permitted,
        ] {
            for synthetic in [false, true] {
                let mut i = input(1, 2, &[200, 100, 200]);
                i.metadata.adjustment_status = adjustment.clone();
                i.metadata.redistribution_status = redistribution.clone();
                i.metadata.synthetic = synthetic;
                assert_eq!(report(&i).decisions, vec![H, H, E]);
            }
        }
    }
}

#[test]
fn repeated_results_serialization_and_inputs_are_unchanged_on_success_and_failure() {
    for invalid in [false, true] {
        let mut i = input(1, 2, &[200, 100, 200, 100]);
        if invalid {
            i.candles[3].close = 0;
        }
        let before = i.clone();
        let input_bytes = serde_json::to_vec(&i).unwrap();
        let first = generate_sma_v1(&i);
        let first_bytes = serde_json::to_vec(&first).unwrap();
        for _ in 0..8 {
            let next = generate_sma_v1(&i);
            assert_eq!(next, first);
            assert_eq!(serde_json::to_vec(&next).unwrap(), first_bytes);
            assert_eq!(i, before);
            assert_eq!(serde_json::to_vec(&i).unwrap(), input_bytes);
        }
    }
}

#[test]
fn closed_error_code_encoding_includes_unreachable_arithmetic_variant() {
    // Enum serialization only; this does not fabricate a generator overflow or
    // add a public/private injection control for mathematically impossible input.
    for (code, text) in [
        (SmaErrorCodeV1::InvalidInput, "INVALID_INPUT"),
        (SmaErrorCodeV1::InvalidConfig, "INVALID_CONFIG"),
        (SmaErrorCodeV1::InvalidMetadata, "INVALID_METADATA"),
        (SmaErrorCodeV1::InvalidCandle, "INVALID_CANDLE"),
        (SmaErrorCodeV1::ArithmeticOverflow, "ARITHMETIC_OVERFLOW"),
    ] {
        assert_eq!(serde_json::to_value(code).unwrap(), text);
    }
}
