use crate::data::{NormalizedCandleV1, ProvenanceV1};
use crate::execution::{BarCompletionV1, SignalDecisionV1};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SmaInputV1 {
    pub schema_version: String,
    pub candles: Vec<NormalizedCandleV1>,
    pub metadata: ProvenanceV1,
    pub bar_completion: BarCompletionV1,
    pub fast_period: u64,
    pub slow_period: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SmaReportV1 {
    pub metadata: ProvenanceV1,
    pub bar_completion: BarCompletionV1,
    pub fast_period: u64,
    pub slow_period: u64,
    pub decisions: Vec<SignalDecisionV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SmaErrorCodeV1 {
    InvalidInput,
    InvalidConfig,
    InvalidMetadata,
    InvalidCandle,
    ArithmeticOverflow,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SmaErrorV1 {
    pub code: SmaErrorCodeV1,
    pub bar_index: Option<u64>,
    pub field: String,
    pub message: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SmaResultV1 {
    Success {
        schema_version: &'static str,
        report: SmaReportV1,
    },
    Failure {
        schema_version: &'static str,
        error: SmaErrorV1,
    },
}
