# SMA crossover v1 — proposed strategy contract

**DRAFT / NON-EXECUTABLE / STRATEGY APPROVAL REQUIRED.** QOS-0010 authorizes
documentation preparation only. Every SD choice below is PROPOSED, including
the recommended crossover/equality convention. No strategy, indicator API or
backtest has been implemented or run by this task.

The accepted [execution model](backtest-execution-v1-draft.md) and
[Rust execution API](execution-api-v1.md) remain unchanged. The owner accepted
QOS-0009 head `6c4720566ea919eaf50221ccff0fc7e107c3eef3`; PR #9 was squash-merged
as `b4ee5eb9c6321fbc33e0c7c0a26b9aee5fc909df`. This approves the provided-signal
kernel only. Aggregate P2 acceptance is pending. See the
[preparation work order](tasks/QOS-0010-P2-SMA-CONTRACT.md) and
[invented acceptance cases](../fixtures/synthetic/sma-crossover-cases.md).

## Proposed decisions requiring review

| ID | PROPOSED choice | Consequence / limit |
| --- | --- | --- |
| SD01 | One symbol/exchange, normalized completed contiguous 5m candles; use CLOSE only. | No inferred bars/sessions, sampling repair, other price source or adjustment inference. |
| SD02 | Required exact integer fast period 1..9999 and slow period 2..10000, with fast < slow; no defaults. | A period counts input candles, not clock duration or exchange sessions. The 10000-bar cap is a proposed resource bound, not an optimized parameter. |
| SD03 | Trailing unweighted sums including the current completed CLOSE; averages are exact fractions. | No floating point, display rounding, truncated averages or initial partial-window averages. |
| SD04 | Hold until two consecutive bars have both full-window averages. | First joint availability is index slow-1; earliest possible signal is index slow. A pre-existing above/below relationship does not cause an initial entry/exit. |
| SD05 | Up: previous fast <= slow and current fast > slow; down: previous fast >= slow and current fast < slow. | Equality at the current bar is Hold; equality at the previous bar can start a crossing. Use the immediately previous bar, without remembering the last non-equal regime. |
| SD06 | Exactly one Hold/EnterLong/ExitLong per candle, independent of holdings/orders/protection. | Repeated same-direction signals after equality are retained. ExitLong is a long exit instruction, never a short entry. |
| SD07 | Use checked unsigned 128-bit rolling sums and comparison intermediates; output no sums/averages as numeric API fields. | Valid sums may exceed the JS-safe price bound M; private exact sums are not prices. Bound proof below covers every permitted period/price. |
| SD08 | Validate all input before calculation; deterministic first error; atomic failure with no partial decisions. | No sorting, coercion, fallback periods, clamping or best-effort prefix result. |
| SD09 | Preserve inputs/provenance/config exactly; deterministic pure calculation with valid-future prefix invariance. | No IO, time/TZ lookup, environment, randomness, portfolio feedback or rights/adjustment certification. |
| SD10 | Pass the full decision vector to the existing kernel unchanged, with separately explicit execution config. | Strategy decisions do not fill trades. Next-OPEN execution, rejection/ignored events, protective precedence and terminal expiry stay with QOS-0009. |

## Proposed semantic boundary and validation

Required semantic inputs: version exactly `"1"`; existing P1 normalized candles
and provenance; explicit `all_complete`; `fast_period` and `slow_period` as SD02.
Missing, negative, fractional, boolean or string periods are invalid; never
coerce, round or default them. These are proposed semantic requirements, not a
published Rust function or JSON parser. A later, separately approved implementation
card must lock exact Rust types, error enum/serialization and any decoder surface
before code. No existing API signature/type is changed by this proposal.

Proposed validation order and fault tuples `(category, bar_index, field)`:

1. Version and completion declaration, in that order: `INVALID_INPUT`, no bar,
   `schema_version` or `bar_completion`.
2. Fast type/range, then slow type/range: `INVALID_CONFIG`, no bar, `fast_period`
   or `slow_period`. Then fast < slow: `INVALID_CONFIG`, no bar, `periods`.
3. Provenance: `INVALID_METADATA`, no bar, first bad metadata field in order
   source/synthetic/adjustment_status/redistribution_status. Source must contain a
   character other than ASCII space/tab/CR/LF; boolean/enums must match P1 types.
4. Nonempty candles and JS-safe length: `INVALID_INPUT`, no bar, `candles`.
5. Candles in input order: `INVALID_CANDLE`, that zero-based index and
   `candles[i].field`. Field order: symbol, exchange, interval, source,
   ts_utc_ms, open, high, low, close, OHLC consistency (`ohlc`), volume.

Reuse P1 values unchanged: nonempty single symbol/exchange under the same
ASCII-whitespace check, interval exactly 5m, source exactly equal to provenance,
timestamp integer 0 <= t < 4102444800000, OHLC positive integer paise <=
M=9007199254740991, low <= open/close <= high, volume integer 0..M. Adjacent
OPEN timestamps must differ exactly 300000 ms. No alignment to a guessed session
or calendar, and no inference from the host clock. Typed fields may prevent
invalid variants/types from construction; no malformed-JSON coverage is implied.
Even fields not used by the SMA calculation are validated at this boundary.

All validation precedes all SMA calculation; the first error wins. Failure has
no decisions or average trace, including when an invalid late candle follows
otherwise usable history. Messages must be fixed descriptions, never private
cell/source values. The error tuples above are proposed semantics, not existing
public enum/JSON objects.

Nonempty input shorter than slow is valid and returns only Hold. Input of exactly
slow candles also returns only Hold, even if fast is above/below slow on its final
bar. Empty input is invalid rather than a zero-length successful strategy.

## Proposed exact rolling arithmetic

Let C[i] be integer CLOSE paise, f the fast period, s the slow period and i a
zero-based input index. For either period n, a full window is available iff
i >= n-1. Its sum and average are:

```text
S_n[i] = sum(C[j] for j = i-n+1 .. i)
A_n[i] = S_n[i] / n                         # exact rational paise
```

While filling a window, accumulate its partial sum but expose no average. Once
full, update by **subtracting the outgoing CLOSE before adding the new CLOSE**:

```text
S_n[i] = (S_n[i-1] - C[i-n]) + C[i]          # i >= n
```

No float conversion, division rounding, UI-formatted price or previous-window
exclusion of C[i] is permitted. Compare the two averages by cross multiplication:

```text
L[i] = S_f[i] * s
R[i] = S_s[i] * f
relation[i] = less / equal / greater according to L[i] versus R[i]
```

Both products share the positive denominator f*s, so their comparison is exactly
the comparison of the two rational averages. A half-paise or third-paise average
is valid internally; it does not create a fractional execution price or share.

Proposed bounds: 0 <= partial/full sum <= n*M <= 10000*M =
90071992547409910000. Each comparison product <= 10000*10000*M =
900719925474099100000000 < 2^128. With f < s the actual bound is tighter.
Values >M in these private sums/products are valid. Do not reject them as unsafe
prices or serialize them as JS-number trace fields. The proposed result contains
decisions plus preserved version/declarations/provenance/periods, not an average
or sum series. Markdown traces below are mathematical expectations, not an API.

Use checked u128 subtraction/addition/multiplication and enforce these bounds.
After validation, calculation failure would be `ARITHMETIC_OVERFLOW` at the first
bar/field: update fast sum (`fast_sum`), slow sum (`slow_sum`), then fast product
(`fast_product`), then slow product (`slow_product`). Under the stated bounds this
is unreachable for valid input; it still must fail atomically if an invariant or
future implementation change violates the proof. Never wrap, clamp or switch to
floating point. P1 input values >M remain validation failures, not widened prices.

## Proposed close decisions, equality and warm-up

```text
i < s-1: Hold                              # slow window unavailable
i == s-1: Hold                            # first joint availability
i >= s:
  L[i-1] <= R[i-1] and L[i] > R[i]: EnterLong
  L[i-1] >= R[i-1] and L[i] < R[i]: ExitLong
  otherwise: Hold
```

At i=s, previous refers to i=s-1, where both windows are full. Therefore at least
s+1 candles are needed to generate any non-Hold decision; its earliest next-OPEN
fill needs at least s+2 candles. Equality at the current bar never emits a signal.
The sequences greater/equal/greater and less/equal/less can emit a repeated
EnterLong or ExitLong on their last bar; there is no latch, debounce or holdings
filter. Strictly above/above or below/below emits Hold.

The final candle follows the same strategy rule as every other candle; do not
replace its crossing with Hold because a next bar is absent. Consequently every
valid prefix's decisions equal the matching prefix of a valid extension. Valid
future price/volume/decision-independent data changes cannot alter earlier
decisions. An invalid extension instead fails whole-input validation; that is
not a prefix-invariance counterexample. Inputs are never mutated.

## Proposed composition with the accepted kernel

The strategy only generates the full decision vector. A later caller constructs
the existing ExecutionInputV1 using unchanged candles/provenance/declarations,
that vector and explicitly supplied QOS-0009 execution config. No config is
invented or inferred by the strategy. Kernel validation remains in force.

- A close-i crossing can first attempt a signal fill at OPEN i+1.
- Holdings, cash, capacity, ignored/superseded/expired events and protective exits
  cannot feed back into the strategy's vector. Rejected fills do not retry or
  suppress future crossings at the strategy layer.
- An eligible final-bar signal expires in the kernel; Hold emits nothing and an
  incompatible action is ignored, as already specified by the execution API.
- Closing prices are taken as supplied. No corporate-action cashflows or rights
  certification is added; kernel warnings/accounting remain unchanged.

The [synthetic integration example](../fixtures/synthetic/sma-crossover-cases.md#sc13-kernel-composition-expectations)
derives next-OPEN fills by hand. This task does not execute that composition.

## Review and implementation gate

Review all SD01–SD10 together, especially the period cap, first-availability Hold,
equality-reset crossings, state independence and wide private arithmetic. The
case-specific periods/costs are invented test inputs, not recommended trading
parameters or implicit defaults. No profitability or market performance claim.

No SMA strategy implementation, EMA, breakout, metrics, inferred bars, calendars,
CLI/UI/WASM binding or browser parity is authorized. A later owner-approved card
must lock the exact implementation interface and executable acceptance tests.
Existing CI validates unchanged executable code only; it does not run this
proposal or validate its Markdown. Approval of drafting is not strategy approval,
PR merge authorization or aggregate P2 acceptance.
