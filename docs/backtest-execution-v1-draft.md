# P2 execution model v1 — proposal for owner review

**DRAFT / NON-EXECUTABLE.** QOS-0008 is authorized to prepare this document,
not to approve its financial choices or implement an engine. P1 was accepted at
PR #7 head `da33ea0e199afe4d732afe6f45dc874a2dc8981e`, merged into develop as
`21ac34874550e89873f677cb395140e0ebbd957f`. Existing public APIs stay unchanged.

The six [inherited constraints](backtest-semantics.md) remain authoritative.
Every rule below is a **proposed** way to make those constraints executable.
[Synthetic cases](../fixtures/synthetic/backtest-execution-cases.md) illustrate
the proposal; their numbers are expectations, not measured strategy performance.

## Owner decisions to lock before implementation

| ID | Proposed bounded choice | Why / limit |
| --- | --- | --- |
| D01 | First kernel: one symbol/exchange, long-only, one position, explicit per-bar signal decisions. | Test execution independently; no invented SMA/EMA/breakout rules or strategy DSL. |
| D02 | Completed normalized 5m candles, strictly contiguous OPEN timestamps exactly 300000 ms apart. | Reject gaps rather than infer exchange sessions, repair bars or assume overnight execution. |
| D03 | Decisions at close schedule only the following OPEN; last-bar decisions expire. | No same-candle signal fills or invented future bars. |
| D04 | Integer paise and integer shares; explicit fixed entry quantity; no margin, shorting, pyramiding or partial fills. | A narrow first ledger, not a portfolio-sizing engine. |
| D05 | Explicit fixed-per-fill fee plus notional-bps fee, rounded up; adverse market slippage, buys up and sells down. | Deterministic conservative arithmetic, not any broker's tariff/tax schedule. |
| D06 | Cash may never go negative; reject an unaffordable fill without charging a fee. | Applies to buys and fee-bearing sells; rejected exits can leave risk open. |
| D07 | Finite per-bar share budget from the previous completed bar's volume and explicit participation bps. | A declared liquidity proxy, not actual liquidity; current/future full-bar volume is unavailable at OPEN. |
| D08 | Optional entry-relative stop/target; opening gaps first, scheduled exits next, range protection afterward; stop wins an ambiguous range. | OHLC does not reveal tick order; no guaranteed stop-loss bound. |
| D09 | Stops are market-like; profit targets are conservative limits at the target price. | Stop gaps use the worse OPEN; target gaps receive no favorable improvement. |
| D10 | Leave terminal positions open and mark at the final CLOSE; no forced terminal sale or hypothetical exit fee. | Distinguish realized and unrealized P&L. |
| D11 | Explicit as-supplied corporate-action policy; echo provenance and warn about unknown/unadjusted treatment. | Do not infer splits/dividends, certify adjustment accuracy or claim redistribution rights. |
| D12 | Checked exact arithmetic, atomic failure and reconciliation after every completed bar. | No wraparound, floating financial accounting or partial successful result after an error. |

All D01–D12 need owner approval or amendment. Approval to draft is not approval of
these choices. Calendars, indicators, strategy generation, risk/annualized metrics,
CLI/WASM/browser entry points and broker models remain separate future tasks.

## Proposed input boundary

Consume P1-compatible normalized candles plus their exact provenance and an
explicit `all_complete` declaration. The caller's declaration is not verified
against a host clock. Revalidate all P1 candle/identity/order/provenance invariants
at an untrusted boundary; a typed object alone is not proof of valid data.
Require at least one candle and D02's contiguous grid; reject the entire input
before processing orders if an invariant fails. No sorting, deduplication,
resampling, timezone guessing, calendar lookup or missing-bar fabrication.

Proposed required configuration (no implicit defaults):

| Field | Proposed value / exact constraint |
| --- | --- |
| initial_cash_paise | Integer 1..M, where M = 9007199254740991. |
| entry_quantity | Integer 1..M; every entry is this quantity, every exit closes the whole position. |
| fee_fixed_paise | Integer 0..M per successful fill, on either side. |
| fee_bps | Integer 0..10000, charged on executed notional on either side. |
| market_slippage_bps | Integer 0..9999; applies to signal orders and stop exits, not target limits. |
| participation_bps | Integer 0..10000; zero means no fill capacity, not unlimited liquidity. |
| stop_loss_bps | Null, or integer 1..9999, relative to actual entry fill price. |
| take_profit_bps | Null, or integer 1..10000, relative to actual entry fill price. |
| corporate_action_policy | Explicit `as_supplied_no_cashflows`; no split/dividend/cashflow processing. |
| decisions | Exactly one enum per input bar: `hold`, `enter_long`, or `exit_long`, interpreted only at that bar's CLOSE. |

The first bar starts flat; it cannot fill a signal order because no previous
bar decision exists. Its decision may schedule bar 1. `enter_long` while holding
and `exit_long` while flat produce an ignored-decision audit event, not a fill.
`hold` schedules nothing. A rejected order expires; it is not retried next bar
unless a later completed-bar decision schedules a fresh order. No same-bar reentry.

These are semantic field proposals, not a published JSON schema or Rust function.
A later implementation card must lock exact public types, property validation,
closed issue codes, deterministic issue/event order and serialization shapes.
QOS-0008 adds none of them to the running application.

## Proposed exact money and cost arithmetic

Use checked exact integers, with wider intermediates where necessary. All stored
money/share/volume counts and externally serialized integers must be JS-safe;
signed P&L may range from -M to M. Price fields must remain 1..M. If any computed
price, bracket, notional, fee, cash, basis, equity or cumulative total is outside
its range, fail the entire run with an arithmetic/input error and no successful
ledger. Do not clamp, wrap, switch to float or return an earlier partial run.

For positive integer reference price P, quantity Q and slippage S:

```text
market_buy_price  = ceil(P * (10000 + S) / 10000)
market_sell_price = floor(P * (10000 - S) / 10000)
notional         = fill_price * Q
fee              = fee_fixed_paise + ceil(notional * fee_bps / 10000)
buy_cash_after   = cash_before - notional - fee
sell_cash_after  = cash_before + notional - fee
```

Evaluate checked price/notional/fee arithmetic before affordability. If arithmetic
is valid but resulting cash would be negative, reject the order with unchanged
cash, shares and fee totals. No fee on rejected/ignored/cancelled orders. A
computed zero-paise price or bracket is an error, never rounded up opportunistically.

Actual prices already include market slippage. Its separate cost attribution is
Q*(fill-reference) for a buy and Q*(reference-fill) for a market sell. Never deduct
that attribution again from cash/P&L. A target-limit fill uses its exact target
price, with no adverse slippage that would violate the limit; normal fees apply.
Market slippage is a declared penalty and is not clipped to the bar's HIGH/LOW;
a modeled price can lie outside the recorded range. It is not an observed trade.

Proposed bracket levels, fixed for the position from the successful entry price E:

```text
stop_price   = floor(E * (10000 - stop_loss_bps) / 10000)
target_price = ceil(E * (10000 + take_profit_bps) / 10000)
```

Null disables that individual bracket. Out-of-range computed levels fail the run.
No trailing stops, price-display rounding or changing brackets after entry.

## Proposed chronology and finite-fill assumption

At each bar i > 0, capacity is floor(volume[i-1] * participation_bps / 10000).
Successful buys and sells each consume their share quantity from this **shared**
bar budget. An order must fit in full and leave nonnegative cash; otherwise it
is rejected without consuming capacity. This proxy uses only known history;
it does not guarantee current liquidity or justify a best/worst-loss bound.

Process in this exact proposed order:

1. For a position carried into the bar, check opening protection first. If OPEN
   <= stop, attempt a stop exit with reference OPEN. Otherwise if OPEN >= target,
   attempt a target-limit exit at target. A triggered protective exit supersedes
   a pending signal exit, even if the protective order cannot fill.
2. If no opening protection triggered, attempt a valid prior-close signal exit
   at OPEN. A prior-close entry may fill at OPEN only when flat at bar start.
   Signal market orders use D05's adverse slippage and successful-fill fees.
3. For a remaining position with no exit attempt yet in this bar, use its range:
   LOW <= stop triggers a stop; HIGH >= target triggers a target; both means stop.
   A stop reference is min(OPEN, stop); a target reference is target. This also
   applies after a successful OPEN entry, so entry-bar protection is active.
   Entry plus exit needs enough shared capacity for both fills.
4. At most one exit attempt per position per bar, successful or rejected; no
   fallback to a lower-priority exit or repeated range attempts. A position whose
   exit is rejected stays open, with its existing bracket, into the next bar.
5. Mark remaining holdings at this bar's CLOSE and reconcile. Only now consume
   this bar's supplied decision to schedule the next OPEN, if one exists.

An ambiguous high/low range does not supply an intrabar timestamp. Report bar
index/OPEN timestamp and an explicit `open` or `range` phase for fills; do not
invent tick time/order. The entry-bar stop reference handles an OPEN below the
new stop after adverse entry slippage. Do not use the range or final volume to
decide an earlier OPEN signal fill. Calendar/session-end/holiday logic is absent.

## Proposed position, ledger and terminal accounting

One entry opens one position and one exit closes it entirely; no partial cost-
basis allocation. Entry basis = executed buy notional + entry fee. On exit:

```text
closed_position_net_pnl = sell_notional - exit_fee - entry_basis
realized_pnl           = sum(closed_position_net_pnl)
market_value           = open_quantity * current_close
unrealized_pnl         = market_value - open_entry_basis   # zero when flat
equity                 = cash + market_value
equity                 = initial_cash + realized_pnl + unrealized_pnl
```

Record every successful fill's bar/phase/reason/side, quantity, reference/actual
price, notional, fee and resulting cash/shares/basis. Preserve order rejections,
ignored decisions and expired terminal signals separately from fills. Keep input
provenance plus the full supplied configuration alongside the result. Every
snapshot must satisfy the reconciliation above; count each fee once.

At the final CLOSE, any open position remains marked, with entry fee in basis,
no assumed sale and no hypothetical exit fee. A final-bar decision expires.
The proposed output contains ledger/accounting summaries only. Sharpe, volatility,
annualization, risk-free rates, exchange calendars and performance charts require
separate approved contracts; no such metric is defined or claimed here.

Unknown/unadjusted provenance carries an explicit corporate-action warning.
Adjusted inputs retain their declaration, but the engine does not verify it or
add cashflows. Do not describe raw/unknown-adjustment P&L as corporate-action-
correct or certify data rights. Real historical datasets are outside this task.

## Review and implementation gate

Review D01–D12 together with cases E01–E13. In particular, approve or amend the
strict contiguous-segment scope, previous-volume proxy, all-or-none fills,
one-exit-attempt rule, target-gap policy and terminal marking before coding.
They are proposed modeling choices, not recovered facts or approved defaults.

After the owner approves the financial proposal, a separate implementation card
must lock Rust/API/error shapes and executable golden assertions. That task must
prove chronology, prefix invariance for valid future data, accounting and error
atomicity. This Markdown draft and its arithmetic checks cannot prove those
runtime properties. No engine or P2 phase acceptance is authorized by this PR.
