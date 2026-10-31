# Proposed SMA acceptance cases — invented, not strategy output

**DRAFT / STRATEGY REVIEW REQUIRED / NON-EXECUTABLE.** These are hand-written
expectations for [SD01–SD10](../../docs/strategy-sma-v1-draft.md), not results
from a strategy implementation, engine run or historical dataset. Bounded
Python integer/Fraction assertions independently check arithmetic; they cannot
prove that an unimplemented strategy satisfies the proposal.

## Explicit example conventions

Every listed price is integer paise. Index i identifies OPEN time i*300000 UTC ms;
its completed CLOSE is the strategy observation. Identity is SYNTH/TEST/5m,
source `synthetic`, metadata synthetic=true, adjustment_status=unadjusted,
redistribution_status=unknown and completion=all_complete. Except SC13's explicit
OPEN changes, O=H=L=C and volume=1000 on every bar. Periods are explicit in each
case; no default is proposed. M=9007199254740991.

H=Hold, E=EnterLong, X=ExitLong; these are strategy decisions, never fills.
S_f/S_s show **full-window sums only**; a dash means unavailable, not zero.
Averages show exact rational paise, with optional exact simplification. All
cases use the immediately previous full-window relation for a crossover.

## SC01 warm-up, first availability and equality

f=2, s=3; closes [100,100,100]. Expected decisions [H,H,H].

| i | C | S_f | S_s | A_f | A_s | Decision / reason |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 100 | — | — | — | — | H: warm-up |
| 1 | 100 | 200 | — | 200/2=100 | — | H: slow unavailable |
| 2 | 100 | 200 | 300 | 200/2=100 | 300/3=100 | H: first joint availability |

## SC02 short inputs and first joint relation

f=1, s=2. Each row below is a separate complete input; all return only Hold.
Above/below at the first joint bar does not initialize a position or signal.

| Input closes | i | S_f | S_s | A_f | A_s | Decision |
| --- | --- | --- | --- | --- | --- | --- |
| [101] | 0 | 101 | — | 101/1 | — | H: N < s |
| [100,200] | 0 | 100 | — | 100/1 | — | H |
| [100,200] | 1 | 200 | 300 | 200/1 | 300/2=150 | H: first joint, already above |
| [200,100] | 0 | 200 | — | 200/1 | — | H |
| [200,100] | 1 | 100 | 300 | 100/1 | 300/2=150 | H: first joint, already below |

## SC03 upward and downward crossings, no repeated level signal

f=2, s=3; closes [300,200,100,200,300,200,100,100]. Expected
[H,H,H,H,E,H,X,H]. Earlier closes determine only the trailing windows shown.

| i | C | S_f | S_s | A_f | A_s | Decision / relation |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 300 | — | — | — | — | H: warm-up |
| 1 | 200 | 500 | — | 500/2=250 | — | H: warm-up |
| 2 | 100 | 300 | 600 | 300/2=150 | 600/3=200 | H: first joint, less |
| 3 | 200 | 300 | 500 | 300/2=150 | 500/3 | H: less -> less |
| 4 | 300 | 500 | 600 | 500/2=250 | 600/3=200 | E: less -> greater |
| 5 | 200 | 500 | 700 | 500/2=250 | 700/3 | H: greater -> greater |
| 6 | 100 | 300 | 600 | 300/2=150 | 600/3=200 | X: greater -> less |
| 7 | 100 | 200 | 400 | 200/2=100 | 400/3 | H: less -> less |

## SC04 equality resets and repeated same-direction signals

f=1, s=2; closes [100,100,200,200,300,300,200,200,100]. Expected
[H,H,E,H,E,H,X,H,X]. Current equality is Hold; previous equality qualifies for
the next strict crossing. No last-non-equal latch or holdings filter is allowed.

| i | C | S_f | S_s | A_f | A_s | Decision |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 100 | 100 | — | 100/1 | — | H |
| 1 | 100 | 100 | 200 | 100/1 | 200/2=100 | H: first joint |
| 2 | 200 | 200 | 300 | 200/1 | 300/2=150 | E: equal -> greater |
| 3 | 200 | 200 | 400 | 200/1 | 400/2=200 | H: equal |
| 4 | 300 | 300 | 500 | 300/1 | 500/2=250 | E: repeated EnterLong retained |
| 5 | 300 | 300 | 600 | 300/1 | 600/2=300 | H: equal |
| 6 | 200 | 200 | 500 | 200/1 | 500/2=250 | X: equal -> less |
| 7 | 200 | 200 | 400 | 200/1 | 400/2=200 | H: equal |
| 8 | 100 | 100 | 300 | 100/1 | 300/2=150 | X: repeated ExitLong retained |

E at 4 remains in the vector even if execution still holds a position from E at
2; X at 8 remains even if execution is flat. The kernel decides ignored/expired
events using its state, without changing these strategy expectations.

## SC05 earliest possible crossing and final signal preservation

f=1, s=2; closes [200,100,200]. Expected [H,H,E].

| i | C | S_f | S_s | A_f | A_s | Decision |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 200 | 200 | — | 200/1 | — | H |
| 1 | 100 | 100 | 300 | 100/1 | 300/2=150 | H: first joint |
| 2 | 200 | 200 | 300 | 200/1 | 300/2=150 | E: earliest signal index s=2 |

The strategy retains final E. With explicit SC13 kernel configuration, no earlier
signal exists and no following OPEN exists: no fill, eligible E expires at CLOSE
2, ending cash/equity 100000. This is a hand-derived composition expectation.

## SC06 fractional comparisons cannot be rounded before crossing

f=2, s=3; closes [100,100,99,101,100]. Expected [H,H,H,H,E].

| i | C | S_f | S_s | A_f | A_s | Decision |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 100 | — | — | — | — | H |
| 1 | 100 | 200 | — | 200/2=100 | — | H |
| 2 | 99 | 199 | 299 | 199/2 | 299/3 | H: first joint, 597 < 598 |
| 3 | 101 | 200 | 300 | 200/2=100 | 300/3=100 | H: 600 = 600 |
| 4 | 100 | 201 | 300 | 201/2 | 300/3=100 | E: 603 > 600 |

Truncating both averages to 100 at index 4 would miss E. The inequalities use
S_f*s versus S_s*f, never display-rounded averages.

## SC07 safe input prices with sums beyond M

f=1, s=2; closes [M-1,M,M-1]. Expected [H,H,X], even when execution is flat.

| i | C and S_f | S_s | A_f | A_s | Exact comparison / decision |
| --- | --- | --- | --- | --- | --- |
| 0 | 9007199254740990 | — | (M-1)/1 | — | H |
| 1 | 9007199254740991 | 18014398509481981 | M/1 | 18014398509481981/2 | 2M > 2M-1; H first joint |
| 2 | 9007199254740990 | 18014398509481981 | (M-1)/1 | 18014398509481981/2 | 2M-2 < 2M-1; X |

These sums are valid private u128 values. Fractional averages near M and products
differing by one must remain exact; no JS-safe sum cap or float conversion.

## SC08 period cap and wide-arithmetic proof (symbolic)

f=9999, s=10000 with a single close M is valid short input: at i=0 both full sums
and averages are unavailable, partial sums are M, decision H. Bounds alone do
not demand 10000 candles or change short-input success.

For the **symbolic constant sequence** C[i]=M through i=10000, without generating
a fixture/dataset, exact full-window expectations are:

| Indices | S_f | S_s | A_f | A_s | Decision |
| --- | --- | --- | --- | --- | --- |
| 0..9997 | — | — | — | — | H |
| 9998 | 90062985348155169009 | — | (9999*M)/9999=M | — | H |
| 9999 | 90062985348155169009 | 90071992547409910000 | M | (10000*M)/10000=M | H: first joint |
| 10000 | 90062985348155169009 | 90071992547409910000 | M | M | H: equal -> equal |

Both comparison products are 900629853481551690090000. They exceed u64 as well
as M, but fit u128. The universal proposed bound 10000*10000*M is
900719925474099100000000, also < 2^128. No valid configured arithmetic-overflow
case exists under the proof; checked-operation failures still require atomic
errors in a future implementation. These are algebraic bounds, not a stress run.

## SC09 invalid periods and deterministic first configuration error

Use one otherwise valid 100-paise candle. Validation precedes SMA calculation;
no sums, average trace or partial decisions are returned on any row.

| fast | slow | Proposed fault: category / field |
| --- | --- | --- |
| missing, true, "2", 1.5, -1 or 0 | 3 | INVALID_CONFIG / fast_period |
| 10000 | 10000 | INVALID_CONFIG / fast_period (range before relation) |
| 1 | missing, false, "3", 2.5, 1 or 10001 | INVALID_CONFIG / slow_period |
| 3 | 3 | INVALID_CONFIG / periods |
| 4 | 3 | INVALID_CONFIG / periods |

All faults have bar_index absent. f=1/s=2 and f=9999/s=10000 are valid endpoints;
None/zero is not a disabled-period mode. These are semantic fault examples;
QOS-0010 supplies no parser or executable validation API.

## SC10 gaps, duplicates, descending times and late atomic validation

f=1, s=2; otherwise valid closes [100,100]. Each timestamp row is a separate
invalid input: [0,600000] (gap), [0,0] (duplicate), [300000,0] (descending).
Expected fault: INVALID_CANDLE, bar 1, candles[1].ts_utc_ms. No SMA/decisions.

Late variant: closes [200,100,200,100], timestamps [0,300000,600000,1200000].
The last opening should be 900000. Expected INVALID_CANDLE at bar 3,
candles[3].ts_utc_ms, with **no** [H,H,E] partial output. No bar is fabricated.
The prevalidation failure is not an arithmetic or future-invariance failure.

## SC11 price/input boundaries and precedence

f=1, s=2, matching common metadata unless varied. Error rows return no sums,
fractions or decisions. All field/price values not mentioned remain valid.

| Explicit variation | Proposed first fault |
| --- | --- |
| No candles | INVALID_INPUT, no bar, candles |
| schema_version="2", fast=0 | INVALID_INPUT, no bar, schema_version |
| fast=0 and metadata.source empty | INVALID_CONFIG, no bar, fast_period |
| Source only ASCII space/tab/CR/LF | INVALID_METADATA, no bar, metadata.source |
| Bar 0 O=H=L=100; C=0 | INVALID_CANDLE, bar 0, candles[0].close |
| Bar 0 O=H=L=M; C=M+1 | INVALID_CANDLE, bar 0, candles[0].close |
| Bar 0 O=H=L=100; C=101 | INVALID_CANDLE, bar 0, candles[0].ohlc |
| Candle source differs from metadata | INVALID_CANDLE, bar 0, candles[0].source |

M itself is valid (SC07/SC08); M+1 is not rescued by wide SMA storage. All input
fields remain byte-for-byte unchanged on success/failure, and fixed error messages
must not echo these private values. Repeated identical calls must yield identical
decisions/errors; these are future runtime assertions, not executed here.

## SC12 valid future-prefix and mutation expectations

Use f=2/s=3 and the SC03 closes. For every nonempty prefix of lengths 1..8,
the expected decisions are exactly the corresponding prefix of
[H,H,H,H,E,H,X,H]. No final signal suppression or future warm-up adjustment.
Length 7 ends in X, which stays X when length 8 is supplied.

Now change only index 7's complete candle to O=H=L=C=400 and its volume to 0;
all other values remain valid and unchanged. Indices 0..6 retain SC03 sums,
fractions and decisions. Only the new suffix calculation differs:

| i | S_f | S_s | A_f | A_s | Decision |
| --- | --- | --- | --- | --- | --- |
| 7 original C=100 | 200 | 400 | 200/2=100 | 400/3 | H: less -> less |
| 7 changed C=400 | 500 | 700 | 500/2=250 | 700/3 | E: less -> greater |

Both complete vectors are defined by closed prefixes alone. Changing volume or
other **valid non-CLOSE** data cannot change strategy decisions. Invalid future
data instead causes whole-input validation failure, as SC10 explicitly shows.

## SC13 kernel composition expectations

Use SC03's eight closes, f=2/s=3 and its exact sum/fraction trace. Keep common
metadata, volume=1000 and all_complete. O=H=L=C except:

- Bar 5: O=H=250, L=C=200.
- Bar 7: O=H=150, L=C=100.

Explicit existing execution config: initial_cash_paise=100000, entry_quantity=2,
fee_fixed_paise=0, fee_bps=0, market_slippage_bps=0, participation_bps=10000,
stop_loss_bps=null, take_profit_bps=null,
corporate_action_policy=as_supplied_no_cashflows. These are invented case inputs,
not broker costs, defaults or recommended trading parameters.

Expected strategy vector [H,H,H,H,E,H,X,H] is passed unchanged to QOS-0009:

| Trigger CLOSE | Earliest fill OPEN | Timestamp | Side / quantity | Price | Cash after | Position / basis after |
| --- | --- | --- | --- | --- | --- | --- |
| 4, C=300, E | 5 | 1500000 | Buy / 2 | 250 | 99500 | 2 / 500 |
| 6, C=100, X | 7 | 2100000 | Sell / 2 | 150 | 99800 | 0 / 0 |

No fill occurs at either signal candle. Prior volumes at bars 4 and 6 provide
capacity 1000 on bars 5 and 7; after each fill it is 998. At CLOSE 5: market value
400, unrealized -100, equity 99900. At CLOSE 6: market value 200, unrealized -300,
equity 99700. Final cash/equity 99800, realized -200, unrealized 0, fees/slippage
0, two fills and one closed position. Reconciliation: 100000-200=99800.

Liquidity variant: change only volume[4] to 1. The strategy vector is identical;
bar-5 buy is rejected with capacity 1, no fill/fee. X at CLOSE 6 is ignored by the
flat kernel, with no bar-7 sale. Final cash/equity 100000, no position or fills.
These derived observations test the intended separation of signals and execution;
**neither the strategy nor this integration was executed by QOS-0010**.

## Verification boundary

The preparation receipt is in [QOS-0010's handoff](../../docs/tasks/QOS-0010-P2-SMA-CONTRACT.md).
Integer/Fraction checks can verify the sums, rational relations, expected vectors
and hand-derived ledger arithmetic above. Only a separately approved implementation
and executable tests can prove rolling-state behavior, validation/error ordering,
input preservation, causal composition and runtime parity. Existing regression CI
does not discover/execute Markdown cases. No strategy/market result, profitability,
source-rights certification or aggregate P2 acceptance is claimed.
