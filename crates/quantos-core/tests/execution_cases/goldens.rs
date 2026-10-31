use super::support::*;
use quantos_core::execution::*;
use serde_json::json;

#[test]
fn e01_next_open_and_exact_serialized_report() {
    let i = input(&[10000, 11000, 12000], &[ENTER, EXIT, HOLD]);
    let r = report(&i);
    assert_eq!(
        serde_json::to_value(execute_v1(&i)).unwrap(),
        json!({
            "status":"success", "schema_version":"1", "report":{
                "metadata":{"source":"synthetic","synthetic":true,"adjustment_status":"unadjusted","redistribution_status":"unknown"},
                "bar_completion":"all_complete",
                "config":{"initial_cash_paise":100000,"entry_quantity":2,"fee_fixed_paise":0,"fee_bps":0,"market_slippage_bps":0,"participation_bps":10000,"stop_loss_bps":null,"take_profit_bps":null,"corporate_action_policy":"as_supplied_no_cashflows"},
                "decisions":["enter_long","exit_long","hold"],
                "warnings":["liquidity_proxy","unadjusted_prices"],
                "fills":[
                    {"sequence":0,"bar_index":1,"ts_utc_ms":300000,"phase":"open","reason":"entry_signal","side":"buy","position_id":1,"quantity":2,"reference_price_paise":11000,"fill_price_paise":11000,"notional_paise":22000,"fee_paise":0,"slippage_paise":0,"cash_after_paise":78000,"position_quantity_after":2,"entry_basis_after_paise":22000,"realized_pnl_after_paise":0},
                    {"sequence":1,"bar_index":2,"ts_utc_ms":600000,"phase":"open","reason":"exit_signal","side":"sell","position_id":1,"quantity":2,"reference_price_paise":12000,"fill_price_paise":12000,"notional_paise":24000,"fee_paise":0,"slippage_paise":0,"cash_after_paise":102000,"position_quantity_after":0,"entry_basis_after_paise":0,"realized_pnl_after_paise":2000}
                ],
                "events":[],
                "snapshots":[
                    {"bar_index":0,"ts_utc_ms":0,"cash_paise":100000,"position_quantity":0,"entry_basis_paise":0,"market_value_paise":0,"equity_paise":100000,"realized_pnl_paise":0,"unrealized_pnl_paise":0,"total_fees_paise":0,"total_slippage_paise":0,"remaining_capacity":0},
                    {"bar_index":1,"ts_utc_ms":300000,"cash_paise":78000,"position_quantity":2,"entry_basis_paise":22000,"market_value_paise":22000,"equity_paise":100000,"realized_pnl_paise":0,"unrealized_pnl_paise":0,"total_fees_paise":0,"total_slippage_paise":0,"remaining_capacity":998},
                    {"bar_index":2,"ts_utc_ms":600000,"cash_paise":102000,"position_quantity":0,"entry_basis_paise":0,"market_value_paise":0,"equity_paise":102000,"realized_pnl_paise":2000,"unrealized_pnl_paise":0,"total_fees_paise":0,"total_slippage_paise":0,"remaining_capacity":998}
                ],
                "summary":{"initial_cash_paise":100000,"ending_cash_paise":102000,"ending_equity_paise":102000,"open_quantity":0,"open_entry_basis_paise":0,"realized_pnl_paise":2000,"unrealized_pnl_paise":0,"total_fees_paise":0,"total_slippage_paise":0,"fill_count":2,"closed_positions":1}
            }
        })
    );
    assert_eq!(
        r.fills.iter().map(|f| f.bar_index).collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn e02_eligible_terminal_signal_expires_without_a_fill() {
    let r = report(&input(&[10000], &[ENTER]));
    assert!(r.fills.is_empty());
    assert_eq!(r.summary.ending_equity_paise, 100000);
    assert_eq!(
        serde_json::to_value(&r.events).unwrap(),
        json!([{
            "sequence":0,"bar_index":0,"ts_utc_ms":0,"phase":"close",
            "event":{"kind":"signal_expired","decision":"enter_long"}
        }])
    );
}

#[test]
fn e03_costs_counted_once_and_exact_open_position_basis() {
    let mut i = input(&[10000, 10000, 11000], &[ENTER, EXIT, HOLD]);
    i.config.market_slippage_bps = 10;
    i.config.fee_fixed_paise = 7;
    i.config.fee_bps = 10;
    let r = report(&i);
    assert_eq!(
        (
            r.fills[0].fill_price_paise,
            r.fills[0].fee_paise,
            r.fills[0].cash_after_paise
        ),
        (10010, 28, 79952)
    );
    assert_eq!(
        (r.fills[1].fill_price_paise, r.fills[1].fee_paise),
        (10989, 29)
    );
    assert_eq!(
        (
            r.summary.ending_cash_paise,
            r.summary.realized_pnl_paise,
            r.summary.total_fees_paise,
            r.summary.total_slippage_paise
        ),
        (101901, 1901, 57, 42)
    );
    assert_eq!(
        (
            r.snapshots[1].entry_basis_paise,
            r.snapshots[1].equity_paise,
            r.snapshots[1].unrealized_pnl_paise
        ),
        (20048, 99952, -48)
    );
}

#[test]
fn e04_adverse_rounding_is_integer_exact() {
    let mut i = input(&[10001, 10001, 10001], &[ENTER, EXIT, HOLD]);
    i.config.entry_quantity = 1;
    i.config.market_slippage_bps = 1;
    i.config.fee_bps = 1;
    let r = report(&i);
    assert_eq!(
        (r.fills[0].fill_price_paise, r.fills[0].fee_paise),
        (10003, 2)
    );
    assert_eq!(
        (r.fills[1].fill_price_paise, r.fills[1].fee_paise),
        (9999, 1)
    );
    assert_eq!(
        (
            r.summary.ending_cash_paise,
            r.summary.realized_pnl_paise,
            r.summary.total_fees_paise,
            r.summary.total_slippage_paise
        ),
        (99993, -7, 3, 4)
    );
}

#[test]
fn e05_fee_affordability_rejection_and_exact_cash_boundary() {
    let mut i = input(&[10000, 10000], &[ENTER, HOLD]);
    i.config.initial_cash_paise = 20000;
    i.config.fee_fixed_paise = 1;
    let r = report(&i);
    assert!(r.fills.is_empty());
    assert_eq!(
        (r.summary.ending_cash_paise, r.summary.total_fees_paise),
        (20000, 0)
    );
    assert!(matches!(
        r.events[0].event,
        ExecutionEventKindV1::OrderRejected {
            cause: ExecutionRejectionV1::InsufficientCash,
            ..
        }
    ));
    i.config.initial_cash_paise = 20001;
    let r = report(&i);
    assert_eq!(
        (
            r.fills.len(),
            r.summary.ending_cash_paise,
            r.summary.open_quantity,
            r.summary.total_fees_paise
        ),
        (1, 0, 2, 1)
    );
}

#[test]
fn e06_liquidity_uses_previous_volume_not_entry_bar_final_volume() {
    let mut i = input(&[10000, 10000], &[ENTER, HOLD]);
    i.candles[0].volume = 10;
    i.candles[1].volume = 100000;
    i.config.participation_bps = 1000;
    let a = report(&i);
    assert!(a.fills.is_empty());
    assert_eq!(
        (
            a.summary.ending_cash_paise,
            a.summary.total_fees_paise,
            a.snapshots[1].remaining_capacity
        ),
        (100000, 0, 1)
    );
    assert!(matches!(
        a.events[0].event,
        ExecutionEventKindV1::OrderRejected {
            cause: ExecutionRejectionV1::InsufficientLiquidity,
            ..
        }
    ));
    i.candles[1].volume = 0;
    assert_eq!(a, report(&i));
}

#[test]
fn e07_stop_wins_ambiguous_range() {
    let mut i = bracket_input();
    i.candles[2].high = 10400;
    i.candles[2].low = 9700;
    let r = report(&i);
    assert_eq!(
        (
            r.fills[1].reason,
            r.fills[1].phase,
            r.fills[1].fill_price_paise
        ),
        (
            ExecutionFillReasonV1::StopLoss,
            ExecutionPhaseV1::Range,
            9800
        )
    );
    assert_eq!(
        (r.summary.ending_cash_paise, r.summary.realized_pnl_paise),
        (99600, -400)
    );
}

#[test]
fn e08_stop_gap_uses_worse_open() {
    let mut i = bracket_input();
    i.candles[2].open = 9600;
    i.candles[2].high = 9700;
    i.candles[2].low = 9500;
    i.candles[2].close = 9600;
    let r = report(&i);
    assert_eq!(
        (
            r.fills[1].reason,
            r.fills[1].phase,
            r.fills[1].reference_price_paise
        ),
        (
            ExecutionFillReasonV1::StopLoss,
            ExecutionPhaseV1::Open,
            9600
        )
    );
    assert_eq!(
        (r.summary.ending_cash_paise, r.summary.realized_pnl_paise),
        (99200, -800)
    );
}

#[test]
fn e09_target_gap_uses_target_without_favorable_improvement() {
    let mut i = bracket_input();
    i.candles[2].open = 10500;
    i.candles[2].high = 10600;
    i.candles[2].low = 10400;
    i.candles[2].close = 10500;
    let r = report(&i);
    assert_eq!(
        (
            r.fills[1].reason,
            r.fills[1].phase,
            r.fills[1].fill_price_paise
        ),
        (
            ExecutionFillReasonV1::TakeProfit,
            ExecutionPhaseV1::Open,
            10300
        )
    );
    assert_eq!(
        (r.summary.ending_cash_paise, r.summary.realized_pnl_paise),
        (100600, 600)
    );
}

#[test]
fn e10_entry_bar_protection_and_shared_capacity_rejection() {
    let mut i = bracket_input();
    i.candles.truncate(2);
    i.decisions.truncate(2);
    i.candles[1].high = 10400;
    i.candles[1].low = 9700;
    let r = report(&i);
    assert_eq!(
        (r.fills[0].bar_index, r.fills[0].phase),
        (1, ExecutionPhaseV1::Open)
    );
    assert_eq!(
        (r.fills[1].bar_index, r.fills[1].phase),
        (1, ExecutionPhaseV1::Range)
    );
    assert_eq!(
        (
            r.summary.ending_cash_paise,
            r.summary.realized_pnl_paise,
            r.snapshots[1].remaining_capacity
        ),
        (99600, -400, 996)
    );
    i.candles[0].volume = 2;
    let r = report(&i);
    assert_eq!(
        (
            r.fills.len(),
            r.summary.ending_equity_paise,
            r.summary.open_quantity,
            r.snapshots[1].remaining_capacity
        ),
        (1, 100000, 2, 0)
    );
    assert!(matches!(
        r.events[0].event,
        ExecutionEventKindV1::OrderRejected {
            reason: ExecutionFillReasonV1::StopLoss,
            cause: ExecutionRejectionV1::InsufficientLiquidity,
            ..
        }
    ));
}

#[test]
fn e11_terminal_mark_has_no_fabricated_sale_or_fee() {
    let mut i = input(&[10000, 11000, 12000], &[ENTER, HOLD, HOLD]);
    i.candles[2].high = 12100;
    i.candles[2].low = 11400;
    i.candles[2].close = 11500;
    let r = report(&i);
    assert_eq!(r.fills.len(), 1);
    assert_eq!(
        (
            r.summary.ending_cash_paise,
            r.summary.open_entry_basis_paise,
            r.snapshots[2].market_value_paise,
            r.summary.ending_equity_paise
        ),
        (78000, 22000, 23000, 101000)
    );
    assert_eq!(
        (
            r.summary.realized_pnl_paise,
            r.summary.unrealized_pnl_paise,
            r.summary.closed_positions
        ),
        (0, 1000, 0)
    );
}

#[test]
fn e12_missing_grid_fails_before_any_orders() {
    let mut i = input(&[10000, 10000], &[ENTER, HOLD]);
    i.candles[1].ts_utc_ms = 600000;
    failure(
        &i,
        ExecutionErrorCodeV1::InvalidCandle,
        Some(1),
        "candles[1].ts_utc_ms",
    );
}

#[test]
fn e13_later_cash_overflow_returns_no_partial_report() {
    let mut i = input(&[1, 1, M], &[ENTER, EXIT, HOLD]);
    i.config.initial_cash_paise = M;
    i.config.entry_quantity = 1;
    failure(
        &i,
        ExecutionErrorCodeV1::ArithmeticOverflow,
        Some(2),
        "cash_paise",
    );
}
