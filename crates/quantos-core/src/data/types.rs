use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationStatusV1 {
    Success,
    Failure,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum GapAssessmentV1 {
    NotPerformed { missing_ts_utc_ms: Vec<u64> },
    Performed { missing_ts_utc_ms: Vec<u64> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjustmentStatusV1 {
    Unknown,
    Unadjusted,
    SplitAdjusted,
    TotalReturnAdjusted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RedistributionStatusV1 {
    Unknown,
    PrivateOnly,
    Permitted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProvenanceV1 {
    pub source: String,
    pub synthetic: bool,
    pub adjustment_status: AdjustmentStatusV1,
    pub redistribution_status: RedistributionStatusV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum IntervalV1 {
    #[serde(rename = "5m")]
    FiveMinutes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NormalizedCandleV1 {
    pub symbol: String,
    pub exchange: String,
    pub interval: IntervalV1,
    pub ts_utc_ms: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
    pub source: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssueCodeV1 {
    InvalidRequest,
    UnsupportedSchemaVersion,
    InvalidOptions,
    InvalidMetadata,
    InvalidExpectedGrid,
    EmptyDataset,
    InvalidHeader,
    RowWidth,
    InvalidIdentity,
    MixedDataset,
    UnsupportedInterval,
    SourceMismatch,
    InvalidTimestamp,
    TimezoneRequired,
    TimezoneConflict,
    TimestampOutOfRange,
    DuplicateTimestamp,
    DescendingTimestamp,
    InvalidPrice,
    PricePrecision,
    PriceOutOfRange,
    InvalidVolume,
    VolumeOutOfRange,
    InvalidOhlc,
    MissingExpectedBar,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverityV1 {
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct IssueV1 {
    pub code: IssueCodeV1,
    pub severity: IssueSeverityV1,
    pub row: Option<u64>,
    pub field: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NormalizationResultV1 {
    pub schema_version: &'static str,
    pub status: NormalizationStatusV1,
    pub metadata: Option<ProvenanceV1>,
    pub issues: Vec<IssueV1>,
    pub gap_assessment: GapAssessmentV1,
    pub candles: Vec<NormalizedCandleV1>,
}
