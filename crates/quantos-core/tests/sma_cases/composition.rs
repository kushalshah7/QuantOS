use super::support::*;
use quantos_core::execution::*;
use quantos_core::sma::SmaInputV1;

fn kernel_input(i: &SmaInputV1) -> ExecutionInputV1 {
    ExecutionInputV1 {
        schema_version: i.schema_version.clone(),
        candles: i.candles.clone(),
        metadata: i.metadata.clone(),
        bar_completion: i.bar_completion,
        decisions: report(i).decisions,
        config: ExecutionConfigV1 {
            initial_cash_paise: 100000,
            entry_quantity: 2,
            fee_fixed_paise: 0,
            fee_bps: 0,
            market_slippage_bps: 0,
            participation_bps: 10000,
            stop_loss_bps: None,
            take_profit_bps: None,
            corporate_action_policy: CorporateActionPolicyV1::AsSuppliedNoCashflows,
        },
    }
}

fn execute(i: &ExecutionInputV1) -> ExecutionReportV1 {
    match execute_v1(i) {
        ExecutionResultV1::Success {
            schema_version,
            report,
        } => {
            assert_eq!(schema_version, "1");
            for s in &report.snapshots {
                assert_eq!(s.equity_paise, s.cash_paise + s.market_value_paise);
                assert_eq!(
                    i128::from(s.equity_paise),
                    i128::from(i.config.initial_cash_paise)
                        + i128::from(s.realized_pnl_paise)
                        + i128::from(s.unrealized_pnl_paise)
                );
            }
            *report
        }
        other => panic!("expected kernel success: {other:?}"),
    }
}

fn sc13() -> SmaInputV1 {
    let mut i = input(2, 3, &[300, 200, 100, 200, 300, 200, 100, 100]);
    i.candles[5].open = 250;
    i.candles[5].high = 250;
    i.candles[7].open = 150;
    i.candles[7].high = 150;
    i
}

#[test]
fn sc13_actual_composition_has_exact_next_open_fills_and_ledger() {
    let i = sc13();
    let before = i.clone();
    let k = kernel_input(&i);
    let k_before = k.clone();
    assert_eq!(k.decisions, vec![H, H, H, H, E, H, X, H]);
    let r = execute(&k);
    assert_eq!(r.decisions, k.decisions);
    assert_eq!(r.metadata, i.metadata);
    assert!(r.events.is_empty());
    assert_eq!(
        r.fills
            .iter()
            .map(|f| (
                f.bar_index,
                f.ts_utc_ms,
                f.phase,
                f.side,
                f.quantity,
                f.fill_price_paise,
                f.cash_after_paise,
                f.entry_basis_after_paise,
                f.realized_pnl_after_paise,
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                5,
                1500000,
                ExecutionPhaseV1::Open,
                ExecutionSideV1::Buy,
                2,
                250,
                99500,
                500,
                0
            ),
            (
                7,
                2100000,
                ExecutionPhaseV1::Open,
                ExecutionSideV1::Sell,
                2,
                150,
                99800,
                0,
                -200
            ),
        ]
    );
    assert_eq!(r.fills[0].reason, ExecutionFillReasonV1::EntrySignal);
    assert_eq!(r.fills[1].reason, ExecutionFillReasonV1::ExitSignal);
    assert_eq!((r.fills[0].position_id, r.fills[1].position_id), (1, 1));
    for (bar, value, unrealized, equity) in [(5, 400, -100, 99900), (6, 200, -300, 99700)] {
        let s = &r.snapshots[bar];
        assert_eq!(
            (
                s.cash_paise,
                s.entry_basis_paise,
                s.market_value_paise,
                s.unrealized_pnl_paise,
                s.equity_paise
            ),
            (99500, 500, value, unrealized, equity)
        );
    }
    assert_eq!(
        (
            r.snapshots[5].remaining_capacity,
            r.snapshots[7].remaining_capacity
        ),
        (998, 998)
    );
    assert_eq!(
        (
            r.summary.ending_cash_paise,
            r.summary.ending_equity_paise,
            r.summary.realized_pnl_paise,
            r.summary.unrealized_pnl_paise,
            r.summary.total_fees_paise,
            r.summary.total_slippage_paise,
            r.summary.fill_count,
            r.summary.closed_positions,
            r.summary.open_quantity
        ),
        (99800, 99800, -200, 0, 0, 0, 2, 1, 0)
    );
    assert_eq!(i, before);
    assert_eq!(k, k_before);
}

#[test]
fn sc13_liquidity_rejection_keeps_signal_vector_and_ignores_flat_exit() {
    let mut i = sc13();
    let baseline = report(&i);
    i.candles[4].volume = 1;
    assert_eq!(report(&i), baseline);
    let r = execute(&kernel_input(&i));
    assert!(r.fills.is_empty());
    assert_eq!(r.events.len(), 2);
    assert_eq!(
        (
            r.events[0].bar_index,
            r.events[0].phase,
            r.events[0].sequence
        ),
        (5, ExecutionPhaseV1::Open, 0)
    );
    assert_eq!(
        r.events[0].event,
        ExecutionEventKindV1::OrderRejected {
            side: ExecutionSideV1::Buy,
            reason: ExecutionFillReasonV1::EntrySignal,
            quantity: 2,
            cause: ExecutionRejectionV1::InsufficientLiquidity,
        }
    );
    assert_eq!(
        (
            r.events[1].bar_index,
            r.events[1].phase,
            r.events[1].sequence
        ),
        (6, ExecutionPhaseV1::Close, 1)
    );
    assert_eq!(
        r.events[1].event,
        ExecutionEventKindV1::DecisionIgnored { decision: X }
    );
    assert_eq!(r.snapshots[5].remaining_capacity, 1);
    assert_eq!(
        (
            r.summary.ending_cash_paise,
            r.summary.ending_equity_paise,
            r.summary.open_quantity,
            r.summary.fill_count,
            r.summary.total_fees_paise
        ),
        (100000, 100000, 0, 0, 0)
    );
}

#[test]
fn sc05_final_generated_signal_is_preserved_and_expires_in_kernel() {
    let i = input(1, 2, &[200, 100, 200]);
    let k = kernel_input(&i);
    assert_eq!(k.decisions, vec![H, H, E]);
    let r = execute(&k);
    assert!(r.fills.is_empty());
    assert_eq!(r.events.len(), 1);
    assert_eq!(
        (r.events[0].bar_index, r.events[0].phase),
        (2, ExecutionPhaseV1::Close)
    );
    assert_eq!(
        r.events[0].event,
        ExecutionEventKindV1::SignalExpired { decision: E }
    );
    assert_eq!(
        (r.summary.ending_cash_paise, r.summary.ending_equity_paise),
        (100000, 100000)
    );
}

#[test]
fn sc04_portfolio_state_ignores_repeated_decisions_without_changing_generator() {
    let i = input(1, 2, &[100, 100, 200, 200, 300, 300, 200, 200, 100]);
    let k = kernel_input(&i);
    assert_eq!(k.decisions, vec![H, H, E, H, E, H, X, H, X]);
    let r = execute(&k);
    assert_eq!(r.decisions, k.decisions);
    assert_eq!(
        r.fills.iter().map(|f| f.bar_index).collect::<Vec<_>>(),
        vec![3, 7]
    );
    assert_eq!(
        r.events
            .iter()
            .map(|e| (e.bar_index, e.event.clone()))
            .collect::<Vec<_>>(),
        vec![
            (4, ExecutionEventKindV1::DecisionIgnored { decision: E }),
            (8, ExecutionEventKindV1::DecisionIgnored { decision: X }),
        ]
    );
    assert_eq!(
        (r.summary.ending_equity_paise, r.summary.realized_pnl_paise),
        (100000, 0)
    );
}
