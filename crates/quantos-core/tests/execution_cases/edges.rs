use super::support::*;
use quantos_core::data::*;
use quantos_core::execution::*;
use serde_json::json;

#[test]
fn sale_cash_is_bounded_after_fee_not_before_it() {
    let mut i = input(&[1, 1, 3], &[ENTER, EXIT, HOLD]);
    i.config.initial_cash_paise = M;
    i.config.entry_quantity = 1;
    i.config.fee_fixed_paise = 1;
    let r = report(&i);
    assert_eq!(r.fills[0].cash_after_paise, M - 2);
    // Sale proceeds temporarily take the wider intermediate to M+1; its fee
    // restores M. Rejecting that intermediate would reject a valid final ledger.
    assert_eq!(r.summary.ending_cash_paise, M);
    assert_eq!(r.summary.ending_equity_paise, M);
    assert_eq!(r.summary.realized_pnl_paise, 0);
    assert_eq!(r.summary.total_fees_paise, 2);
}

#[test]
fn quote_failures_precede_liquidity_and_cash_gates() {
    let mut i = input(&[2, 2], &[ENTER, HOLD]);
    i.config.entry_quantity = M;
    i.candles[0].volume = 0;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(1),
        "notional_paise",
    );

    let mut i = input(&[1, 1, 1], &[ENTER, EXIT, HOLD]);
    i.config.entry_quantity = 1;
    i.config.market_slippage_bps = 9999;
    i.candles[1].volume = 0;
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidExecutionPrice,
        Some(2),
        "fill_price_paise",
    );

    let mut i = input(&[1, M], &[ENTER, HOLD]);
    i.config.market_slippage_bps = 1;
    i.candles[0].volume = 0;
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidExecutionPrice,
        Some(1),
        "fill_price_paise",
    );

    let mut i = input(&[1, 1], &[ENTER, HOLD]);
    i.config.fee_fixed_paise = M;
    i.config.fee_bps = 1;
    i.candles[0].volume = 0;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(1),
        "fee_paise",
    );
}

#[test]
fn rejection_does_not_validate_uncommittable_brackets_or_charge_costs() {
    for no_liquidity in [false, true] {
        let mut i = input(&[1, 1], &[ENTER, HOLD]);
        i.config.initial_cash_paise = 1;
        i.config.stop_loss_bps = Some(1); // floor(1 * .9999) = 0 if it could fill.
        i.config.fee_fixed_paise = M;
        if no_liquidity {
            i.candles[0].volume = 0;
        }
        let r = report(&i);
        assert!(r.fills.is_empty());
        assert_eq!(r.summary.ending_cash_paise, 1);
        assert_eq!(r.summary.total_fees_paise, 0);
        assert_eq!(r.summary.total_slippage_paise, 0);
        assert_eq!(
            r.snapshots[1].remaining_capacity,
            if no_liquidity { 0 } else { 1000 }
        );
        assert_eq!(
            r.events[0].event,
            ExecutionEventKindV1::OrderRejected {
                side: ExecutionSideV1::Buy,
                reason: ExecutionFillReasonV1::EntrySignal,
                quantity: 2,
                cause: if no_liquidity {
                    ExecutionRejectionV1::InsufficientLiquidity
                } else {
                    ExecutionRejectionV1::InsufficientCash
                },
            }
        );
    }
}

#[test]
fn committable_bracket_prices_must_be_positive_and_safe() {
    let mut i = input(&[1, 1], &[ENTER, HOLD]);
    i.config.stop_loss_bps = Some(1);
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidExecutionPrice,
        Some(1),
        "stop_price_paise",
    );
    let mut i = input(&[M, M], &[ENTER, HOLD]);
    i.config.initial_cash_paise = M;
    i.config.entry_quantity = 1;
    i.config.take_profit_bps = Some(1);
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidExecutionPrice,
        Some(1),
        "target_price_paise",
    );
}

#[test]
fn cumulative_fee_overflow_discards_prior_successful_cycles() {
    let p = M / 4;
    let mut i = input(
        &[p, p, 2 * p, p, 2 * p, p, 2 * p],
        &[ENTER, EXIT, ENTER, EXIT, ENTER, EXIT, HOLD],
    );
    i.config.initial_cash_paise = M;
    i.config.entry_quantity = 1;
    i.config.fee_bps = 5000;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(6),
        "total_fees_paise",
    );
}

#[test]
fn cumulative_slippage_overflow_discards_prior_successful_fill() {
    let p = M / 4;
    let mut i = input(&[p, p, M], &[ENTER, EXIT, HOLD]);
    i.config.initial_cash_paise = M;
    i.config.entry_quantity = 1;
    i.config.market_slippage_bps = 9999;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(2),
        "total_slippage_paise",
    );
}

#[test]
fn late_mark_and_equity_overflow_are_atomic() {
    let mut i = input(&[1, 1, M], &[ENTER, HOLD, HOLD]);
    i.config.initial_cash_paise = M;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(2),
        "market_value_paise",
    );
    i.config.entry_quantity = 1;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(2),
        "equity_paise",
    );
}

#[test]
fn deterministic_first_error_is_validation_before_execution() {
    let mut i = input(&[1, 1, M], &[ENTER, EXIT, HOLD]);
    i.config.initial_cash_paise = M;
    i.schema_version = "1 ".into();
    i.config.entry_quantity = 0;
    i.metadata.source = " \t\r\n".into();
    i.candles[2].volume = M + 1;
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidInput,
        None,
        "schema_version",
    );
    i.schema_version = "1".into();
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidConfig,
        None,
        "config.entry_quantity",
    );
    i.config.entry_quantity = 1;
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidMetadata,
        None,
        "metadata.source",
    );
    i.metadata.source = "synthetic".into();
    // Without the invalid final volume, the sale would overflow cash. All
    // candles are validated first, so the invalid candle wins deterministically.
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidCandle,
        Some(2),
        "candles[2].volume",
    );
    i.candles.clear();
    failure(&i, ExecutionErrorCodeV1::InvalidInput, None, "candles");
    i = input(&[1, 1], &[HOLD]);
    i.candles[0].open = 0;
    failure(&i, ExecutionErrorCodeV1::InvalidInput, None, "decisions");
}

#[test]
fn config_outside_each_declared_range_fails_without_coercion() {
    // Each tuple changes exactly one field; every value is otherwise representable
    // by the public Rust type. Missing/unknown typed enum variants cannot compile.
    for (name, value) in [
        ("initial_cash_paise", 0),
        ("initial_cash_paise", M + 1),
        ("entry_quantity", 0),
        ("entry_quantity", u64::MAX),
        ("fee_fixed_paise", M + 1),
        ("fee_bps", 10001),
        ("market_slippage_bps", 10000),
        ("participation_bps", 10001),
        ("stop_loss_bps", 0),
        ("stop_loss_bps", 10000),
        ("take_profit_bps", 0),
        ("take_profit_bps", 10001),
    ] {
        let mut i = input(&[1], &[HOLD]);
        match name {
            "initial_cash_paise" => i.config.initial_cash_paise = value,
            "entry_quantity" => i.config.entry_quantity = value,
            "fee_fixed_paise" => i.config.fee_fixed_paise = value,
            "fee_bps" => i.config.fee_bps = value,
            "market_slippage_bps" => i.config.market_slippage_bps = value,
            "participation_bps" => i.config.participation_bps = value,
            "stop_loss_bps" => i.config.stop_loss_bps = Some(value),
            "take_profit_bps" => i.config.take_profit_bps = Some(value),
            _ => unreachable!(),
        }
        failure(
            &i,
            ExecutionErrorCodeV1::InvalidConfig,
            None,
            &format!("config.{name}"),
        );
    }
    let mut i = input(&[M], &[HOLD]);
    i.config.initial_cash_paise = M;
    i.config.entry_quantity = M;
    i.config.fee_fixed_paise = M;
    i.config.fee_bps = 10000;
    i.config.market_slippage_bps = 9999;
    i.config.participation_bps = 0;
    i.config.stop_loss_bps = Some(9999);
    i.config.take_profit_bps = Some(10000);
    assert_eq!(report(&i).summary.ending_equity_paise, M);
}

#[test]
fn normalized_candles_are_revalidated_at_the_typed_boundary() {
    for field in [
        "symbol",
        "exchange",
        "source",
        "ts_utc_ms",
        "open",
        "high",
        "low",
        "close",
        "ohlc",
        "volume",
    ] {
        let mut i = input(&[100, 100], &[HOLD, HOLD]);
        let c = &mut i.candles[1];
        match field {
            "symbol" => c.symbol = "OTHER".into(),
            "exchange" => c.exchange = "OTHER".into(),
            "source" => c.source = "raw-private-sentinel".into(),
            "ts_utc_ms" => c.ts_utc_ms = 0,
            "open" => c.open = 0,
            "high" => c.high = M + 1,
            "low" => c.low = u64::MAX,
            "close" => c.close = 0,
            "ohlc" => c.low = 101,
            "volume" => c.volume = M + 1,
            _ => unreachable!(),
        }
        let f = failure(
            &i,
            ExecutionErrorCodeV1::InvalidCandle,
            Some(1),
            &format!("candles[1].{field}"),
        );
        assert!(!f.to_string().contains("raw-private-sentinel"));
    }
    for field in ["symbol", "exchange"] {
        let mut i = input(&[100], &[HOLD]);
        if field == "symbol" {
            i.candles[0].symbol = " \t\n\r".into();
        } else {
            i.candles[0].exchange.clear();
        }
        failure(
            &i,
            ExecutionErrorCodeV1::InvalidCandle,
            Some(0),
            &format!("candles[0].{field}"),
        );
    }
    let mut i = input(&[100], &[HOLD]);
    i.candles[0].ts_utc_ms = 4_102_444_800_000;
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].ts_utc_ms",
    );
    i.candles[0].ts_utc_ms -= 1;
    report(&i);
}

#[test]
fn metadata_and_unicode_identity_are_preserved_without_certification() {
    for (adjustment, warning) in [
        (
            AdjustmentStatusV1::Unknown,
            Some(ExecutionWarningV1::UnknownAdjustment),
        ),
        (
            AdjustmentStatusV1::Unadjusted,
            Some(ExecutionWarningV1::UnadjustedPrices),
        ),
        (AdjustmentStatusV1::SplitAdjusted, None),
        (AdjustmentStatusV1::TotalReturnAdjusted, None),
    ] {
        let mut i = input(&[100], &[HOLD]);
        i.metadata.source = " स्रोत \u{a0}".into();
        i.metadata.synthetic = false;
        i.metadata.redistribution_status = RedistributionStatusV1::PrivateOnly;
        i.metadata.adjustment_status = adjustment;
        i.candles[0].source = i.metadata.source.clone();
        i.candles[0].symbol = "\u{a0}".into(); // P1 emptiness uses ASCII whitespace only.
        let r = report(&i);
        let mut expected = vec![ExecutionWarningV1::LiquidityProxy];
        expected.extend(warning);
        assert_eq!(r.warnings, expected);
    }
}

#[test]
fn deterministic_serialization_and_input_preservation_on_success_and_failure() {
    for invalid in [false, true] {
        let mut i = bracket_input();
        if invalid {
            i.candles[2].open = 0;
        }
        let before = i.clone();
        let bytes = serde_json::to_vec(&i).unwrap();
        let first = execute_v1(&i);
        for _ in 0..8 {
            let next = execute_v1(&i);
            assert_eq!(first, next);
            assert_eq!(
                serde_json::to_vec(&first).unwrap(),
                serde_json::to_vec(&next).unwrap()
            );
            assert_eq!(i, before);
            assert_eq!(serde_json::to_vec(&i).unwrap(), bytes);
        }
    }
}

#[test]
fn future_extension_and_mutation_preserve_prior_fills_and_marks() {
    let mut extended = input(
        &[100, 110, 120, 130, 140],
        &[ENTER, EXIT, ENTER, EXIT, HOLD],
    );
    let original = report(&extended);
    for length in 1..extended.candles.len() {
        let mut prefix = extended.clone();
        prefix.candles.truncate(length);
        prefix.decisions.truncate(length);
        let r = report(&prefix);
        assert_eq!(r.snapshots, original.snapshots[..length]);
        // Eligible terminal decisions expire in a prefix but schedule in an
        // extension, so terminal audit events are intentionally not compared.
        let earlier: Vec<_> = original
            .fills
            .iter()
            .filter(|f| f.bar_index < length as u64)
            .cloned()
            .collect();
        assert_eq!(r.fills, earlier);
    }
    extended.candles[3].open = 200;
    extended.candles[3].high = 200;
    extended.candles[3].low = 200;
    extended.candles[3].close = 200;
    extended.candles[3].volume = 0;
    extended.decisions[3] = HOLD;
    extended.decisions[4] = ENTER;
    let changed = report(&extended);
    assert_eq!(changed.snapshots[..3], original.snapshots[..3]);
    assert_eq!(
        changed
            .fills
            .iter()
            .filter(|f| f.bar_index < 3)
            .collect::<Vec<_>>(),
        original
            .fills
            .iter()
            .filter(|f| f.bar_index < 3)
            .collect::<Vec<_>>()
    );
}

#[test]
fn current_range_and_final_volume_cannot_change_an_open_signal_fill() {
    let i = input(&[100, 100, 100], &[ENTER, HOLD, HOLD]);
    let baseline = report(&i);
    let mut changed = i.clone();
    changed.candles[1].high = 1000;
    changed.candles[1].low = 1;
    changed.candles[1].volume = 0;
    assert_eq!(report(&changed).fills, baseline.fills);
}

#[test]
fn opening_protection_supersedes_exit_even_if_rejected_and_does_not_retry_same_bar() {
    let mut i = bracket_input();
    i.decisions[1] = EXIT;
    i.candles[1].volume = 0;
    i.candles[2].open = 9600;
    i.candles[2].low = 9500;
    let r = report(&i);
    assert_eq!(r.fills.len(), 1);
    assert_eq!(r.events.len(), 2);
    assert_eq!(r.events[0].sequence, 1);
    assert_eq!(r.events[0].phase, ExecutionPhaseV1::Open);
    assert_eq!(
        r.events[0].event,
        ExecutionEventKindV1::SignalSuperseded { decision: EXIT }
    );
    assert_eq!(r.events[1].sequence, 2);
    assert!(matches!(
        r.events[1].event,
        ExecutionEventKindV1::OrderRejected {
            reason: ExecutionFillReasonV1::StopLoss,
            cause: ExecutionRejectionV1::InsufficientLiquidity,
            ..
        }
    ));
    assert_eq!(r.summary.open_quantity, 2);
}

#[test]
fn rejected_open_signal_exit_suppresses_later_range_exit_attempt() {
    let mut i = bracket_input();
    i.decisions[1] = EXIT;
    i.candles[1].volume = 0;
    i.candles[2].low = 9700;
    let r = report(&i);
    assert_eq!(r.events.len(), 1);
    assert!(matches!(
        r.events[0].event,
        ExecutionEventKindV1::OrderRejected {
            reason: ExecutionFillReasonV1::ExitSignal,
            cause: ExecutionRejectionV1::InsufficientLiquidity,
            ..
        }
    ));
    assert_eq!(r.events[0].phase, ExecutionPhaseV1::Open);
    assert_eq!(r.summary.open_quantity, 2);
}

#[test]
fn rejected_protection_can_retry_only_on_a_later_bar() {
    let mut i = input(&[10000, 10000, 9600, 9500], &[ENTER, HOLD, HOLD, HOLD]);
    i.config.stop_loss_bps = Some(200);
    i.candles[1].volume = 0;
    let r = report(&i);
    assert_eq!(r.events.len(), 1);
    assert_eq!(r.fills.len(), 2);
    assert_eq!(r.fills[1].bar_index, 3);
    assert_eq!(r.fills[1].fill_price_paise, 9500);
    assert_eq!(r.fills[1].reason, ExecutionFillReasonV1::StopLoss);
    assert_eq!(r.summary.realized_pnl_paise, -1000);
}

#[test]
fn target_limit_keeps_exact_target_despite_market_slippage() {
    let mut i = bracket_input();
    i.config.market_slippage_bps = 100;
    i.candles[2].open = 11000;
    i.candles[2].high = 11000;
    let r = report(&i);
    assert_eq!(r.fills[0].fill_price_paise, 10100);
    assert_eq!(r.fills[1].reason, ExecutionFillReasonV1::TakeProfit);
    assert_eq!(r.fills[1].fill_price_paise, 10403);
    assert_eq!(r.fills[1].slippage_paise, 0);
    assert_eq!(r.summary.ending_cash_paise, 100606);
    assert_eq!(r.summary.total_slippage_paise, 200);
}

#[test]
fn independently_disabled_brackets_do_not_exit() {
    let mut i = bracket_input();
    i.candles[2].low = 9700;
    i.candles[2].high = 10400;
    i.config.stop_loss_bps = None;
    let r = report(&i);
    assert_eq!(r.fills[1].reason, ExecutionFillReasonV1::TakeProfit);
    i.config.stop_loss_bps = Some(200);
    i.config.take_profit_bps = None;
    assert_eq!(report(&i).fills[1].reason, ExecutionFillReasonV1::StopLoss);
    i.config.stop_loss_bps = None;
    assert_eq!(report(&i).fills.len(), 1);
}

#[test]
fn rejected_entry_is_one_shot_and_does_not_consume_position_id() {
    let mut i = input(
        &[100, 100, 100, 100, 100, 100],
        &[ENTER, ENTER, EXIT, ENTER, EXIT, HOLD],
    );
    i.candles[0].volume = 0;
    let r = report(&i);
    assert_eq!(r.events.len(), 1);
    assert_eq!(
        r.fills
            .iter()
            .map(|f| (f.bar_index, f.position_id))
            .collect::<Vec<_>>(),
        vec![(2, 1), (3, 1), (4, 2), (5, 2)]
    );
    assert_eq!(r.summary.closed_positions, 2);
    i.decisions[1..].fill(HOLD);
    assert!(report(&i).fills.is_empty()); // Original rejection never auto-retries.
}

#[test]
fn close_decisions_use_post_protection_state_and_terminal_context() {
    let mut i = input(&[100, 100, 100], &[EXIT, ENTER, EXIT]);
    let r = report(&i);
    assert_eq!(
        r.events[0].event,
        ExecutionEventKindV1::DecisionIgnored { decision: EXIT }
    );
    assert_eq!(
        r.events[1].event,
        ExecutionEventKindV1::SignalExpired { decision: EXIT }
    );
    assert_eq!(r.summary.open_quantity, 2);
    i.decisions[2] = ENTER;
    assert_eq!(
        report(&i).events[1].event,
        ExecutionEventKindV1::DecisionIgnored { decision: ENTER }
    );
    let mut i = bracket_input();
    i.candles[1].low = 9700;
    i.decisions[1] = ENTER;
    let r = report(&i);
    assert_eq!(
        r.fills
            .iter()
            .map(|f| (f.bar_index, f.side))
            .collect::<Vec<_>>(),
        vec![
            (1, ExecutionSideV1::Buy),
            (1, ExecutionSideV1::Sell),
            (2, ExecutionSideV1::Buy)
        ]
    );
}

#[test]
fn zero_participation_and_sell_cash_rejection_preserve_position() {
    let mut i = input(&[100, 100], &[ENTER, HOLD]);
    i.config.participation_bps = 0;
    assert!(report(&i).fills.is_empty());
    // Entry uses all cash including its fee. Exit proceeds cannot pay the same
    // per-fill fee after a price collapse, so rejection must keep the position.
    let mut i = input(&[100, 100, 1], &[ENTER, EXIT, HOLD]);
    i.config.initial_cash_paise = 300;
    i.config.fee_fixed_paise = 100;
    let r = report(&i);
    assert_eq!(r.summary.ending_cash_paise, 0);
    assert_eq!(r.summary.open_quantity, 2);
    assert_eq!(r.summary.total_fees_paise, 100);
    assert_eq!(r.summary.realized_pnl_paise, 0);
    assert_eq!(r.summary.unrealized_pnl_paise, -298);
    assert!(matches!(
        r.events[0].event,
        ExecutionEventKindV1::OrderRejected {
            side: ExecutionSideV1::Sell,
            cause: ExecutionRejectionV1::InsufficientCash,
            ..
        }
    ));
}

#[test]
fn existing_csv_normalization_values_feed_kernel_without_p1_changes() {
    let CsvParseResultV1::Success { headers, rows, .. } = parse_csv_v1(include_bytes!(
        "../../../../fixtures/synthetic/valid-candles.csv"
    )) else {
        panic!("valid synthetic CSV");
    };
    let normalized = normalize_v1(&json!({
        "schema_version":"1", "headers":headers, "rows":rows,
        "metadata":{"source":"synthetic","synthetic":true,"adjustment_status":"unknown","redistribution_status":"unknown"},
        "options":{"timestamp_column":"ts_utc_ms","timestamp_format":"utc_ms","confirmed_timezone":null,"expected_timestamps_utc_ms":null}
    }));
    assert_eq!(normalized.status, NormalizationStatusV1::Success);
    let mut i = input(&[], &[ENTER, EXIT, HOLD]);
    i.candles = normalized.candles;
    i.metadata = normalized.metadata.unwrap();
    let r = report(&i);
    assert_eq!(
        r.fills
            .iter()
            .map(|f| f.fill_price_paise)
            .collect::<Vec<_>>(),
        vec![10200, 10300]
    );
    assert_eq!(r.summary.ending_cash_paise, 100200);
    assert_eq!(r.summary.realized_pnl_paise, 200);
}

#[test]
fn timezone_probe() {
    if std::env::var_os("QOS_EXEC_TZ_CHILD").is_some() {
        let r = execute_v1(&input(&[100, 110, 120], &[ENTER, EXIT, HOLD]));
        println!("EXEC_TZ_RESULT={}", serde_json::to_string(&r).unwrap());
    }
}

#[test]
fn execution_is_identical_in_isolated_host_timezones() {
    let mut outputs = vec![];
    for zone in ["UTC", "Asia/Kolkata", "America/New_York"] {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "execution_cases::edges::timezone_probe",
                "--nocapture",
            ])
            .env("QOS_EXEC_TZ_CHILD", "1")
            .env("TZ", zone)
            .output()
            .unwrap();
        assert!(out.status.success());
        let stdout = String::from_utf8(out.stdout).unwrap();
        outputs.push(
            stdout
                .lines()
                .find(|line| line.starts_with("EXEC_TZ_RESULT="))
                .unwrap()
                .to_owned(),
        );
    }
    assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]));
}
