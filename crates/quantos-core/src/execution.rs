//! Deterministic, provided-signal execution with integer accounting and atomic errors.

mod arithmetic;
mod engine;
mod types;
mod validation;

pub use types::*;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Execute completed normalized candles without IO or modifying the input.
/// Errors expose no partial report, even after earlier internal fills.
pub fn execute_v1(input: &ExecutionInputV1) -> ExecutionResultV1 {
    let result = validation::validate(input).and_then(|()| engine::run(input));
    match result {
        Ok(report) => ExecutionResultV1::Success {
            schema_version: "1",
            report: Box::new(report),
        },
        Err(error) => ExecutionResultV1::Failure {
            schema_version: "1",
            error,
        },
    }
}
