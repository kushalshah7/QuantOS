# Rust SMA API v1 — QOS-0011 locked interface

The owner approved SD01–SD10 / SC01–SC13 at QOS-0010 head
`43a4577f25c41dcdbb60d9b6170355f683fef6ee`, merged as
`fcd55862fac170c3326da5850b2b0e9c031c1274`, then separately authorized QOS-0011.
This API lock is pushed before code. It implements those semantics without
changing P1/execution interfaces, formulas or examples. Aggregate P2 is pending.

```rust
pub fn generate_sma_v1(input: &SmaInputV1) -> SmaResultV1;
```

All new types are public under `quantos_core::sma`, derive Clone, Debug, PartialEq,
Eq and Serialize. SmaErrorCodeV1 also derives Copy. Struct fields are public.
No Deserialize, parser, file IO, browser export or new dependency is introduced.
Existing NormalizedCandleV1/ProvenanceV1 come from `quantos_core::data`;
BarCompletionV1/SignalDecisionV1 come from `quantos_core::execution` unchanged.

## Input and result types

```rust
pub struct SmaInputV1 {
    pub schema_version: String,
    pub candles: Vec<NormalizedCandleV1>,
    pub metadata: ProvenanceV1,
    pub bar_completion: BarCompletionV1,
    pub fast_period: u64,
    pub slow_period: u64,
}
pub struct SmaReportV1 {
    pub metadata: ProvenanceV1,
    pub bar_completion: BarCompletionV1,
    pub fast_period: u64,
    pub slow_period: u64,
    pub decisions: Vec<SignalDecisionV1>,
}
pub struct SmaErrorV1 {
    pub code: SmaErrorCodeV1,
    pub bar_index: Option<u64>,
    pub field: String,
    pub message: &'static str,
}
pub enum SmaErrorCodeV1 {
    InvalidInput, InvalidConfig, InvalidMetadata, InvalidCandle, ArithmeticOverflow,
}
pub enum SmaResultV1 {
    Success { schema_version: &'static str, report: SmaReportV1 },
    Failure { schema_version: &'static str, error: SmaErrorV1 },
}
```

Result is internally tagged `status`, with snake_case variant names:

```text
{status:"success",schema_version:"1",report:{metadata,bar_completion,fast_period,slow_period,decisions}}
{status:"failure",schema_version:"1",error:{code,bar_index,field,message}}
```

Report declarations/provenance/periods exactly preserve input values. One decision
per candle uses existing strings hold/enter_long/exit_long; no candle copies,
average/sum series, fills, execution config or performance results are returned.
Failure has no report/decisions/trace, including after an invalid late candle.
Absent bar_index serializes as null; field is always a string, never null.
Codes serialize SCREAMING_SNAKE_CASE as shown below. Messages are fixed:

| Code | Exact message |
| --- | --- |
| INVALID_INPUT | Invalid SMA input. |
| INVALID_CONFIG | Invalid SMA configuration. |
| INVALID_METADATA | Invalid SMA provenance. |
| INVALID_CANDLE | Invalid normalized SMA candle. |
| ARITHMETIC_OVERFLOW | Unsafe SMA arithmetic. |

No raw input/source/path/OS error is included in an error message. Success echoes
only the specified input declarations, not a certification of provenance/rights.

## Validation order and representable failures

Follow the approved contract's deterministic first-error order, validating all
candles before calculation:

1. schema_version exactly "1": INVALID_INPUT, no bar, schema_version.
2. Completion is existing AllComplete. No other Rust variant is constructible.
3. fast_period 1..9999, then slow_period 2..10000: INVALID_CONFIG, no bar, the
   corresponding field. Then fast < slow: INVALID_CONFIG, no bar, periods.
4. metadata.source must contain a character other than ASCII space/tab/CR/LF:
   INVALID_METADATA, no bar, metadata.source. Remaining boolean/enum provenance
   fields are type-enforced; preserve them exactly without inference.
5. Nonempty candles and length <=M=9007199254740991: INVALID_INPUT, no bar, candles.
6. Each candle in input order: INVALID_CANDLE, its zero-based index, field
   candles[i].symbol/exchange/interval/source/ts_utc_ms/open/high/low/close/ohlc/volume,
   in that order. Interval is type-enforced FiveMinutes. Identity nonemptiness uses
   the ASCII check above; symbol/exchange exactly match the first candle, source
   exactly matches metadata. Timestamp 0 <= t < 4102444800000 and adjacent delta
   exactly 300000 ms. Each OHLC price 1..M, checked in listed order, then
   low <= open/close <= high, then volume 0..M. No sorting/repair/normalization.

u64 fields prevent negative, fractional, boolean, string or missing Rust values;
required struct fields prevent omission; existing enums/boolean fields prevent
unknown variants/types. These SC09/SC11 semantic invalids are not runtime decoder
tests. There is no decoder here. Greater-than-M u64 prices/volume and zero/out-of-
range periods remain representable and require runtime validation. A slice length
greater than M is guarded but is not claimed as an allocated test case.

## Calculation, errors and complexity

Use the accepted exact formulas and equality/warm-up rules. Private rolling sums
are checked u128, remove the outgoing CLOSE before adding the incoming CLOSE,
and obey sum <= period*M <= 10000*M. Compare checked products S_f*slow and
S_s*fast, each <= 10000*10000*M < 2^128. Values above M in private sums/products
are valid. Never truncate, convert to float or serialize these intermediates.

Calculation error order per bar is fast_sum, slow_sum, fast_product, slow_product;
products are needed only when both windows are full. Any checked operation/bound
failure is ARITHMETIC_OVERFLOW with that bar and field, and discards all decisions.
The bound proof makes this unreachable for valid input. No public testing control,
fabricated input or claim of exercising impossible overflow is permitted.

One pass after full validation, O(number of candles), with two rolling sums and
the previous full-window relation as constant-size state. Read outgoing values
from the borrowed input slice; only the output vector grows. First joint averages
at slow-1 emit Hold; earliest crossing is slow. Retain final-bar crossings and
repeated equality-reset signals, independently of all execution/portfolio state.
Valid-prefix decisions match a valid extension; valid non-CLOSE mutations do not
alter decisions. Invalid extensions fail whole-input validation. No input mutation,
IO, host time/TZ, randomness, hidden defaults or automatic parameter selection.

Strategy code never executes trades. Tests alone may compose the generated vector
with unchanged execute_v1 to prove SC13 next-OPEN/rejection/accounting and SC05
terminal expiry. Local runtime checks are NOT RUN under the owner override;
actual test/CI evidence must be recorded separately after execution in GitHub CI.
