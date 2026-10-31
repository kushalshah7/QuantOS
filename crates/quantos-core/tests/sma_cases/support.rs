use quantos_core::data::*;
use quantos_core::execution::{BarCompletionV1, SignalDecisionV1};
use quantos_core::sma::*;
use serde_json::json;

pub(super) const M: u64 = 9_007_199_254_740_991;
pub(super) const H: SignalDecisionV1 = SignalDecisionV1::Hold;
pub(super) const E: SignalDecisionV1 = SignalDecisionV1::EnterLong;
pub(super) const X: SignalDecisionV1 = SignalDecisionV1::ExitLong;

pub(super) fn input(fast: u64, slow: u64, closes: &[u64]) -> SmaInputV1 {
    SmaInputV1 {
        schema_version: "1".into(),
        candles: closes
            .iter()
            .enumerate()
            .map(|(i, &price)| NormalizedCandleV1 {
                symbol: "SYNTH".into(),
                exchange: "TEST".into(),
                interval: IntervalV1::FiveMinutes,
                ts_utc_ms: i as u64 * 300_000,
                open: price,
                high: price,
                low: price,
                close: price,
                volume: 1000,
                source: "synthetic".into(),
            })
            .collect(),
        metadata: ProvenanceV1 {
            source: "synthetic".into(),
            synthetic: true,
            adjustment_status: AdjustmentStatusV1::Unadjusted,
            redistribution_status: RedistributionStatusV1::Unknown,
        },
        bar_completion: BarCompletionV1::AllComplete,
        fast_period: fast,
        slow_period: slow,
    }
}

pub(super) fn report(i: &SmaInputV1) -> SmaReportV1 {
    match generate_sma_v1(i) {
        SmaResultV1::Success {
            schema_version,
            report,
        } => {
            assert_eq!(schema_version, "1");
            assert_eq!(report.metadata, i.metadata);
            assert_eq!(report.bar_completion, i.bar_completion);
            assert_eq!(report.fast_period, i.fast_period);
            assert_eq!(report.slow_period, i.slow_period);
            assert_eq!(report.decisions.len(), i.candles.len());
            report
        }
        other => panic!("expected SMA success: {other:?}"),
    }
}

pub(super) fn failure(i: &SmaInputV1, code: SmaErrorCodeV1, bar: Option<u64>, field: &str) {
    let result = generate_sma_v1(i);
    let SmaResultV1::Failure {
        schema_version,
        error,
    } = &result
    else {
        panic!("expected atomic SMA failure");
    };
    let (code_text, message) = match code {
        SmaErrorCodeV1::InvalidInput => ("INVALID_INPUT", "Invalid SMA input."),
        SmaErrorCodeV1::InvalidConfig => ("INVALID_CONFIG", "Invalid SMA configuration."),
        SmaErrorCodeV1::InvalidMetadata => ("INVALID_METADATA", "Invalid SMA provenance."),
        SmaErrorCodeV1::InvalidCandle => ("INVALID_CANDLE", "Invalid normalized SMA candle."),
        SmaErrorCodeV1::ArithmeticOverflow => ("ARITHMETIC_OVERFLOW", "Unsafe SMA arithmetic."),
    };
    assert_eq!(*schema_version, "1");
    assert_eq!(
        (error.code, error.bar_index, error.field.as_str()),
        (code, bar, field)
    );
    assert_eq!(error.message, message);
    // Exact shape also excludes partial reports/decisions, extra null fields,
    // raw cell values and traces; absent bar indices must be JSON null.
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        json!({
            "status":"failure", "schema_version":"1",
            "error":{"code":code_text,"bar_index":bar,"field":field,"message":message}
        })
    );
}
