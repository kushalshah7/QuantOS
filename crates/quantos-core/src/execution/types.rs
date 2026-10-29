use crate::data::{NormalizedCandleV1, ProvenanceV1};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BarCompletionV1 {
    AllComplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CorporateActionPolicyV1 {
    AsSuppliedNoCashflows,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalDecisionV1 {
    Hold,
    EnterLong,
    ExitLong,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionConfigV1 {
    pub initial_cash_paise: u64,
    pub entry_quantity: u64,
    pub fee_fixed_paise: u64,
    pub fee_bps: u64,
    pub market_slippage_bps: u64,
    pub participation_bps: u64,
    pub stop_loss_bps: Option<u64>,
    pub take_profit_bps: Option<u64>,
    pub corporate_action_policy: CorporateActionPolicyV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionInputV1 {
    pub schema_version: String,
    pub candles: Vec<NormalizedCandleV1>,
    pub metadata: ProvenanceV1,
    pub bar_completion: BarCompletionV1,
    pub config: ExecutionConfigV1,
    pub decisions: Vec<SignalDecisionV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionErrorCodeV1 {
    InvalidInput,
    InvalidConfig,
    InvalidMetadata,
    InvalidCandle,
    ArithmeticOverflow,
    InvalidExecutionPrice,
    ReconciliationFailure,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionErrorV1 {
    pub code: ExecutionErrorCodeV1,
    pub bar_index: Option<u64>,
    pub field: Option<String>,
    pub message: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionWarningV1 {
    LiquidityProxy,
    UnknownAdjustment,
    UnadjustedPrices,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionPhaseV1 {
    Open,
    Range,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionSideV1 {
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionFillReasonV1 {
    EntrySignal,
    ExitSignal,
    StopLoss,
    TakeProfit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionRejectionV1 {
    InsufficientLiquidity,
    InsufficientCash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionFillV1 {
    pub sequence: u64,
    pub bar_index: u64,
    pub ts_utc_ms: u64,
    pub phase: ExecutionPhaseV1,
    pub reason: ExecutionFillReasonV1,
    pub side: ExecutionSideV1,
    pub position_id: u64,
    pub quantity: u64,
    pub reference_price_paise: u64,
    pub fill_price_paise: u64,
    pub notional_paise: u64,
    pub fee_paise: u64,
    pub slippage_paise: u64,
    pub cash_after_paise: u64,
    pub position_quantity_after: u64,
    pub entry_basis_after_paise: u64,
    pub realized_pnl_after_paise: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExecutionEventKindV1 {
    OrderRejected {
        side: ExecutionSideV1,
        reason: ExecutionFillReasonV1,
        quantity: u64,
        cause: ExecutionRejectionV1,
    },
    DecisionIgnored {
        decision: SignalDecisionV1,
    },
    SignalExpired {
        decision: SignalDecisionV1,
    },
    SignalSuperseded {
        decision: SignalDecisionV1,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionEventV1 {
    pub sequence: u64,
    pub bar_index: u64,
    pub ts_utc_ms: u64,
    pub phase: ExecutionPhaseV1,
    pub event: ExecutionEventKindV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionSnapshotV1 {
    pub bar_index: u64,
    pub ts_utc_ms: u64,
    pub cash_paise: u64,
    pub position_quantity: u64,
    pub entry_basis_paise: u64,
    pub market_value_paise: u64,
    pub equity_paise: u64,
    pub realized_pnl_paise: i64,
    pub unrealized_pnl_paise: i64,
    pub total_fees_paise: u64,
    pub total_slippage_paise: u64,
    pub remaining_capacity: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionSummaryV1 {
    pub initial_cash_paise: u64,
    pub ending_cash_paise: u64,
    pub ending_equity_paise: u64,
    pub open_quantity: u64,
    pub open_entry_basis_paise: u64,
    pub realized_pnl_paise: i64,
    pub unrealized_pnl_paise: i64,
    pub total_fees_paise: u64,
    pub total_slippage_paise: u64,
    pub fill_count: u64,
    pub closed_positions: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExecutionReportV1 {
    pub metadata: ProvenanceV1,
    pub bar_completion: BarCompletionV1,
    pub config: ExecutionConfigV1,
    pub decisions: Vec<SignalDecisionV1>,
    pub warnings: Vec<ExecutionWarningV1>,
    pub fills: Vec<ExecutionFillV1>,
    pub events: Vec<ExecutionEventV1>,
    pub snapshots: Vec<ExecutionSnapshotV1>,
    pub summary: ExecutionSummaryV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ExecutionResultV1 {
    Success {
        schema_version: &'static str,
        report: Box<ExecutionReportV1>,
    },
    Failure {
        schema_version: &'static str,
        error: ExecutionErrorV1,
    },
}
