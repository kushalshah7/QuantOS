use quantos_core::data::*;
use quantos_core::execution::*;
use serde_json::Value;

pub(super) const M: u64 = 9_007_199_254_740_991;
pub(super) const ENTER: SignalDecisionV1 = SignalDecisionV1::EnterLong;
pub(super) const EXIT: SignalDecisionV1 = SignalDecisionV1::ExitLong;
pub(super) const HOLD: SignalDecisionV1 = SignalDecisionV1::Hold;

pub(super) fn input(prices: &[u64], decisions: &[SignalDecisionV1]) -> ExecutionInputV1 {
    ExecutionInputV1 {
        schema_version: "1".to_owned(),
        candles: prices
            .iter()
            .enumerate()
            .map(|(i, &price)| NormalizedCandleV1 {
                symbol: "SYNTH".to_owned(),
                exchange: "TEST".to_owned(),
                interval: IntervalV1::FiveMinutes,
                ts_utc_ms: i as u64 * 300_000,
                open: price,
                high: price,
                low: price,
                close: price,
                volume: 1000,
                source: "synthetic".to_owned(),
            })
            .collect(),
        metadata: ProvenanceV1 {
            source: "synthetic".to_owned(),
            synthetic: true,
            adjustment_status: AdjustmentStatusV1::Unadjusted,
            redistribution_status: RedistributionStatusV1::Unknown,
        },
        bar_completion: BarCompletionV1::AllComplete,
        config: ExecutionConfigV1 {
            initial_cash_paise: 100_000,
            entry_quantity: 2,
            fee_fixed_paise: 0,
            fee_bps: 0,
            market_slippage_bps: 0,
            participation_bps: 10_000,
            stop_loss_bps: None,
            take_profit_bps: None,
            corporate_action_policy: CorporateActionPolicyV1::AsSuppliedNoCashflows,
        },
        decisions: decisions.to_vec(),
    }
}

pub(super) fn bracket_input() -> ExecutionInputV1 {
    let mut i = input(&[10000, 10000, 10000], &[ENTER, HOLD, HOLD]);
    i.config.stop_loss_bps = Some(200);
    i.config.take_profit_bps = Some(300);
    i.candles[1].high = 10050;
    i.candles[1].low = 9950;
    i
}

pub(super) fn report(input: &ExecutionInputV1) -> ExecutionReportV1 {
    let ExecutionResultV1::Success {
        schema_version,
        report,
    } = execute_v1(input)
    else {
        panic!("expected success: {:?}", execute_v1(input));
    };
    assert_eq!(schema_version, "1");
    for s in &report.snapshots {
        assert_eq!(s.equity_paise, s.cash_paise + s.market_value_paise);
        assert_eq!(
            i128::from(s.equity_paise),
            i128::from(input.config.initial_cash_paise)
                + i128::from(s.realized_pnl_paise)
                + i128::from(s.unrealized_pnl_paise)
        );
        assert!(s.cash_paise <= M && s.equity_paise <= M);
    }
    let mut sequences: Vec<u64> = report
        .fills
        .iter()
        .map(|f| f.sequence)
        .chain(report.events.iter().map(|e| e.sequence))
        .collect();
    sequences.sort_unstable();
    assert_eq!(sequences, (0..sequences.len() as u64).collect::<Vec<_>>());
    assert!(
        report
            .fills
            .windows(2)
            .all(|f| f[0].sequence < f[1].sequence)
    );
    assert!(
        report
            .events
            .windows(2)
            .all(|e| e[0].sequence < e[1].sequence)
    );
    assert_eq!(report.summary.fill_count, report.fills.len() as u64);
    assert_eq!(
        report.summary.total_fees_paise,
        report.fills.iter().map(|f| f.fee_paise).sum::<u64>()
    );
    assert_eq!(
        report.summary.total_slippage_paise,
        report.fills.iter().map(|f| f.slippage_paise).sum::<u64>()
    );
    assert_eq!(
        report.summary.closed_positions,
        report
            .fills
            .iter()
            .filter(|f| f.side == ExecutionSideV1::Sell)
            .count() as u64
    );
    assert_eq!(report.metadata, input.metadata);
    assert_eq!(report.config, input.config);
    assert_eq!(report.decisions, input.decisions);
    assert_eq!(report.bar_completion, input.bar_completion);
    *report
}

pub(super) fn failure(
    input: &ExecutionInputV1,
    code: ExecutionErrorCodeV1,
    bar: Option<u64>,
    field: &str,
) -> Value {
    let result = execute_v1(input);
    let ExecutionResultV1::Failure {
        schema_version,
        error,
    } = &result
    else {
        panic!("expected atomic failure");
    };
    assert_eq!(*schema_version, "1");
    assert_eq!(
        (error.code, error.bar_index, error.field.as_deref()),
        (code, bar, Some(field))
    );
    assert!(!error.message.is_empty());
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 3);
    assert_eq!(json["status"], "failure");
    assert!(json.get("report").is_none() && json.get("fills").is_none());
    assert_eq!(json["error"].as_object().unwrap().len(), 4);
    json
}
