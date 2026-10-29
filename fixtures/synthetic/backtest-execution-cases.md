# Proposed execution goldens — synthetic, hand-written, not engine output

**DRAFT / OWNER REVIEW REQUIRED.** These are invented acceptance examples for
[the proposed v1 execution model](../../docs/backtest-execution-v1-draft.md).
No engine, strategy, backtest or real-market result was run to produce them.
Independent exact arithmetic checks validate numerical expectations only;
future Rust tests must separately prove chronology, fills, errors and parity.

## Common explicit example settings

All prices/cash/fees below are integer paise, quantities/volume integer shares.
Use one SYNTH/TEST/5m identity, exact matching synthetic provenance, unadjusted
declared prices, explicit `all_complete` and `as_supplied_no_cashflows` declarations.
OPEN timestamps are 0, 300000, 600000, ...; no bars are invented or missing.
Unless overridden: initial cash 100000, quantity 2, fee_fixed 0, fee_bps 0,
slippage_bps 0, participation_bps 10000, every volume 1000, both brackets null.
These are **case-specific explicit values**, not proposed implicit API defaults.

For plain market cases use bar 0 O/H/L/C = 10000/10100/9900/10000;
bar 1 = 11000/11100/10900/11000; bar 2 = 12000/12100/11900/12000.
`enter, exit, hold` means close decisions `enter_long, exit_long, hold`.

| ID | Explicit variation and proposed expected observation |
| --- | --- |
| E01 Next OPEN | Plain bars, decisions enter/exit/hold: no bar-0 fill; buy 2 at bar-1 OPEN 11000; sell 2 at bar-2 OPEN 12000. Cash after buy 78000; ending cash/equity 102000; realized 2000; unrealized 0. |
| E02 Final decision | Only plain bar 0, decision enter: no following OPEN, so no fills; terminal decision expires; cash/equity 100000 and no position. |
| E03 Costs once | Market reference OPENs 10000 then 11000; quantity 2; slippage 10 bps, fixed fee 7 and fee rate 10 bps. Exact derivation below; ending cash/equity 101901, realized 1901, total fees 57, slippage attribution 42. |
| E04 Rounding | Buy and sell reference OPEN 10001, quantity 1, slippage 1 bps, fee rate 1 bps, fixed 0. Buy price 10003, buy fee 2; sell price 9999, sell fee 1. Ending cash/equity 99993; realized -7, fees 3, slippage attribution 4. |
| E05 Affordability | Cash 20000, quantity 2, reference buy 10000, fixed fee 1. Required 20001 exceeds cash: entry rejected, no fill/fee; cash/equity 20000, no position. |
| E06 Known liquidity | Quantity 2, participation 1000 bps, prior volume 10 gives capacity floor(10*1000/10000)=1. Entry rejected even if this bar's final volume is 100000. No fill/fee; cash/equity 100000. |
| E07 Both touches | Enter 2 at 10000, stop 200 bps / target 300 bps. Entry bar range stays inside 9950..10050. Next bar O/H/L/C=10000/10400/9700/10000 touches both. Stop wins at 9800; proceeds 19600; cash/equity 99600; realized -400. |
| E08 Stop gap | Same entry/brackets; next bar O/H/L/C=9600/9700/9500/9600. Stop reference is OPEN 9600, not 9800; proceeds 19200; ending cash/equity 99200; realized -800. |
| E09 Target gap | Same entry/brackets; next bar O/H/L/C=10500/10600/10400/10500. Conservative limit fills at target 10300, without favorable OPEN improvement; proceeds 20600; cash/equity 100600; realized 600. |
| E10 Entry-bar protection | Enter 2 at 10000 on a bar with O/H/L/C=10000/10400/9700/10000; same brackets. OPEN entry precedes range stop; capacity 1000 covers buy 2 plus sell 2. Ending cash/equity 99600; realized -400. If prior volume is instead 2, buy uses all capacity; stop is rejected, position stays open and terminal CLOSE equity is 100000. |
| E11 Terminal mark | Plain bar 0 then bar 1=11000/11100/10900/11000, bar 2=12000/12100/11400/11500; decisions enter/hold/hold. Buy 2 at 11000, no sale. Ending cash 78000, basis 22000, market value 23000, equity 101000; realized 0, unrealized 1000. No assumed terminal sale fee. |
| E12 Missing grid | OPEN timestamps [0,600000] violate the required 300000-ms execution grid. Entire proposed run fails before orders; no partial successful ledger or fabricated bar. P1 itself is unchanged and does not infer this grid rule. |
| E13 Overflow | M=9007199254740991; initial cash M, quantity 1, zero fees/slippage. After a reference-price-1 buy, cash M-1. A later reference-price-M exit would produce 2M-1=18014398509481981 > M. Entire run fails atomically; no wrapped/clamped cash or earlier partial successful ledger. |

For E03/E04, construct a preceding complete signal bar and valid entry/exit bars
with OPENs equal to the stated references and HIGH=LOW=CLOSE=OPEN. Use enter/exit/
hold decisions so neither order is same-candle. For E05/E06 use one preceding
complete signal bar and a subsequent 10000-price entry bar. E07–E10 also have a
preceding 10000-price signal bar; enter there and hold thereafter. None of these
examples prescribes an indicator or claims a trading edge.

## E03 derivation and reconciliation

```text
buy price       = ceil(10000*10010/10000) = 10010
buy notional    = 2*10010 = 20020
buy fee         = 7 + ceil(20020*10/10000) = 7+21 = 28
cash after buy  = 100000-20020-28 = 79952
entry basis     = 20020+28 = 20048
sell price      = floor(11000*9990/10000) = 10989
sell notional   = 2*10989 = 21978
sell fee        = 7 + ceil(21978*10/10000) = 7+22 = 29
cash after sell = 79952+21978-29 = 101901
realized P&L    = 21978-29-20048 = 1901
total fees      = 28+29 = 57
slippage cost   = 2*(10010-10000) + 2*(11000-10989) = 42
reconciliation  = 100000 + 1901 + 0 = 101901
```

If the entry-bar CLOSE is 10000, equity there is 79952+20000=99952 and
unrealized P&L is 20000-20048=-48; initial cash plus unrealized equals equity.
Do not subtract the separately reported 42 slippage attribution a second time.

## E04 precision boundary

```text
ceil(10001*10001/10000)  = ceil(10002.0001) = 10003
floor(10001*9999/10000)  = floor(9999.9999) = 9999
buy fee                 = ceil(10003/10000) = 2
sell fee                = ceil(9999/10000) = 1
cash after buy          = 100000-10003-2 = 89995
cash after sell         = 89995+9999-1 = 99993
P&L                     = -7 = -4 slippage -3 fees
```

## Required behavioral assertions for the later implementation card

- All E01–E13 numerical outcomes plus full ledger/config/provenance fields,
  explicit rejected/ignored/expired events and error atomicity.
- Valid future-bar changes cannot alter earlier fills or close snapshots;
  changing the entry bar's final volume cannot affect that bar's OPEN capacity.
- Opening protective exits supersede queued exits; a rejected exit has no
  fallback or repeat attempt; cash and shared capacity never become negative.
- Stop/target nulls, individually active brackets, stop priority, gap treatment,
  entry-bar protection and slippage/fee boundaries are tested separately.
- Invalid identities, timestamps, normalization values, decisions/config types,
  zero/out-of-range computed prices/levels, unsafe integers and malformed
  provenance produce complete failures according to the future locked API.
- No test may claim an engine, browser/native-WASM financial parity, source rights
  or market performance from these Markdown examples or arithmetic scratch checks.
