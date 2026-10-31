//! Exact, completed-bar SMA crossover decisions without execution or IO.

mod rolling;
mod types;
mod validation;

pub use types::{SmaErrorCodeV1, SmaErrorV1, SmaInputV1, SmaReportV1, SmaResultV1};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
type Calc<T> = Result<T, SmaErrorV1>;

fn error(code: SmaErrorCodeV1, bar_index: Option<u64>, field: &str) -> SmaErrorV1 {
    let message = match code {
        SmaErrorCodeV1::InvalidInput => "Invalid SMA input.",
        SmaErrorCodeV1::InvalidConfig => "Invalid SMA configuration.",
        SmaErrorCodeV1::InvalidMetadata => "Invalid SMA provenance.",
        SmaErrorCodeV1::InvalidCandle => "Invalid normalized SMA candle.",
        SmaErrorCodeV1::ArithmeticOverflow => "Unsafe SMA arithmetic.",
    };
    SmaErrorV1 {
        code,
        bar_index,
        field: field.to_owned(),
        message,
    }
}

/// Generate exactly one decision per candle after whole-input validation.
/// Failure exposes no partial decisions; the borrowed input is never modified.
pub fn generate_sma_v1(input: &SmaInputV1) -> SmaResultV1 {
    match validation::validate(input).and_then(|()| rolling::decisions(input)) {
        Ok(decisions) => SmaResultV1::Success {
            schema_version: "1",
            report: SmaReportV1 {
                metadata: input.metadata.clone(),
                bar_completion: input.bar_completion,
                fast_period: input.fast_period,
                slow_period: input.slow_period,
                decisions,
            },
        },
        Err(error) => SmaResultV1::Failure {
            schema_version: "1",
            error,
        },
    }
}
