# Rust execution API v1 — QOS-0009 locked interface

The owner's delegated approval accepts QOS-0008 D01–D12 as the bounded model.
This API lock adds technical types/audit/error behavior before implementation;
it changes no P1 API, financial formula or golden expectation. Rust-only,
Serialize-only: no Deserialize, JSON entry point, file IO or browser binding.

```rust
pub fn execute_v1(input: &ExecutionInputV1) -> ExecutionResultV1;
```

All types below are public under `quantos_core::execution`, derive Clone, Debug,
PartialEq, Eq and Serialize; small enums also derive Copy. Struct fields are public.
M=9007199254740991; prices positive paise; all nonnegative output integers <=M;
signed P&L i64 in [-M,M]. Enum strings are snake_case unless noted.

## Input

`ExecutionInputV1 { schema_version: String, candles: Vec<NormalizedCandleV1>,
metadata: ProvenanceV1, bar_completion: BarCompletionV1, config: ExecutionConfigV1,
decisions: Vec<SignalDecisionV1> }`. P1 types are reused unchanged.

- BarCompletionV1: AllComplete.
- CorporateActionPolicyV1: AsSuppliedNoCashflows.
- SignalDecisionV1: Hold, EnterLong, ExitLong.
- ExecutionConfigV1: initial_cash_paise, entry_quantity, fee_fixed_paise, fee_bps,
  market_slippage_bps, participation_bps (all u64); stop_loss_bps and
  take_profit_bps (Option<u64>); corporate_action_policy (enum above).
  Ranges are the accepted model's exact ranges, with no defaults.

Typed enums/required Rust fields prevent unknown variants/omitted fields at this
entry. A future JSON/browser decoder must lock its own rejection contract; this
API does not claim to parse malformed JSON or unknown property types.

Validation is deterministic first-error, not P1's aggregated JSON validation:
version; config fields in declaration order; metadata.source; nonempty candles
and safe length; decisions count; then candles in input order, fields symbol,
exchange, source, timestamp, OHLC fields/range/consistency, volume. Identity/source
strings use P1's exact ASCII-whitespace-only empty check and preserve bytes.
Metadata enum/boolean values are type-enforced. Timestamp range is P1's range;
adjacent OPENs differ exactly 300000 ms. No input mutation or inferred values.

## Result, errors and warnings

ExecutionResultV1 is internally tagged `status`:

```text
success: {status:"success",schema_version:"1",report:ExecutionReportV1}
failure: {status:"failure",schema_version:"1",error:ExecutionErrorV1}
```

Rust Success has schema_version: &'static str and report: Box<ExecutionReportV1>;
Failure has schema_version and error. Boxing does not add a JSON wrapper.
ExecutionReportV1 fields: metadata, bar_completion, config, decisions (exact input
clones); warnings: Vec<ExecutionWarningV1>; fills: Vec<ExecutionFillV1>;
events: Vec<ExecutionEventV1>; snapshots: Vec<ExecutionSnapshotV1>;
summary: ExecutionSummaryV1. No partial report exists in Failure.

ExecutionErrorV1: code: ExecutionErrorCodeV1, bar_index: Option<u64>,
field: Option<String>, message: &'static str. Code strings SCREAMING_SNAKE_CASE:
InvalidInput, InvalidConfig, InvalidMetadata, InvalidCandle, ArithmeticOverflow,
InvalidExecutionPrice, ReconciliationFailure. Messages are fixed descriptive
strings, never raw input/path/OS errors; descriptive wording is not a golden key.
Arithmetic error field identifies the computed output, not private cell contents.

ExecutionWarningV1: LiquidityProxy, UnknownAdjustment, UnadjustedPrices. Always
LiquidityProxy first, then the matching adjustment warning for unknown/unadjusted;
neither adjustment warning for declared split/total-return-adjusted prices. No
rights/adjustment certification; exact provenance and policy remain in the report.

## Fill and event audit

- ExecutionPhaseV1: Open, Range, Close (fills use Open/Range, decisions Close).
- ExecutionSideV1: Buy, Sell.
- ExecutionFillReasonV1: EntrySignal, ExitSignal, StopLoss, TakeProfit.
- ExecutionRejectionV1: InsufficientLiquidity, InsufficientCash.
- ExecutionFillV1 fields (u64 unless noted): sequence, bar_index, ts_utc_ms,
  phase, reason, side, position_id, quantity, reference_price_paise,
  fill_price_paise, notional_paise, fee_paise, slippage_paise, cash_after_paise,
  position_quantity_after, entry_basis_after_paise, realized_pnl_after_paise:i64.
- ExecutionEventV1: sequence:u64, bar_index:u64, ts_utc_ms:u64, phase,
  event:ExecutionEventKindV1. The nested event is internally tagged `kind`:
  OrderRejected {side,reason,quantity:u64,cause:ExecutionRejectionV1};
  DecisionIgnored {decision}; SignalExpired {decision}; SignalSuperseded {decision}.

One global monotonic sequence starts at 0 across fills and events; each vector
retains that order. Snapshots are per-close and outside this sequence. Position
IDs start at 1, increment only for successful entries and never reuse an ID.
OPEN protection superseding a queued ExitLong emits SignalSuperseded before the
protective attempt, even when that attempt is rejected. Rejections emit an event
only, never a fill. At CLOSE: Hold emits nothing; an incompatible action emits
DecisionIgnored even on the final bar; an eligible final-bar action emits
SignalExpired; otherwise an eligible action schedules the next OPEN.

Quote calculation checks positive/safe price, notional and per-fill fee before
rejection gates. Insufficient capacity wins over insufficient cash when both
apply. Otherwise negative hypothetical cash is an InsufficientCash rejection;
cash >M is an arithmetic failure. Brackets, new basis, cumulative fees/slippage
and P&L are calculated/checked only for a fill that can commit. A rejected fill
does not create a position or validate hypothetical brackets/charge totals.
All changes commit together after these checks. Arithmetic failure discards the
whole in-memory report, including any earlier successful fills.

Market quotes use accepted adverse slippage; target limits use the exact target.
For OPEN-entry protection, the recorded same-bar range follows the OPEN entry;
there are no later intrabar signal entries that could use an earlier OPEN.
Intrabar timestamps/tick paths are never invented. Follow the accepted one-exit-
attempt, opening/range priority and shared previous-volume budget exactly.

## Close snapshots and summary

ExecutionSnapshotV1 (u64 unless noted): bar_index, ts_utc_ms (bar OPEN identifier),
cash_paise, position_quantity, entry_basis_paise, market_value_paise, equity_paise,
realized_pnl_paise:i64, unrealized_pnl_paise:i64, total_fees_paise,
total_slippage_paise, remaining_capacity. Snapshot is always after CLOSE marking,
before processing that close's decision. Bar 0 has capacity 0 and starts flat.

ExecutionSummaryV1: initial_cash_paise, ending_cash_paise, ending_equity_paise,
open_quantity, open_entry_basis_paise, realized_pnl_paise:i64,
unrealized_pnl_paise:i64, total_fees_paise, total_slippage_paise, fill_count,
closed_positions (other fields u64). Summary matches the last snapshot; no forced
terminal fill, hypothetical fee or annualized/risk/strategy-performance metric.

Every snapshot obeys equity=cash+market_value=initial_cash+realized+unrealized.
All cost/count/ID outputs obey safe-integer bounds. Any invariant failure returns
only Failure, never a panic or partial result. Input equality/serialized bytes,
repeated result equality, valid-future prefix and prior-volume causality require
tests. This API document alone is not execution evidence.
