# Backtest constraints

Status: P1 data work is accepted. The owner accepted the bounded
[QOS-0008 execution model](backtest-execution-v1-draft.md) through PR #8 under
2026-10-29 delegated approval. QOS-0009 implements that provided-signal model
under the separate [locked Rust API](execution-api-v1.md). On 2026-10-31 the
owner accepted QOS-0009 head `6c4720566ea919eaf50221ccff0fc7e107c3eef3`;
PR #9 merged as `b4ee5eb9c6321fbc33e0c7c0a26b9aee5fc909df`, kernel only.
[QOS-0010's SMA proposal](strategy-sma-v1-draft.md) is documentation-only and
requires strategy review before implementation. The six inherited constraints
below remain unchanged. Aggregate P2 acceptance, strategy generation, risk
metrics and financial browser/WASM parity are pending.

1. Signals observe completed candles only; a close-derived signal cannot fill
   before the next bar's open. Future tests must prove no same-candle lookahead.
2. Account for configurable fees, slippage, liquidity assumptions, cash,
   integer share quantity, positions, and realized/unrealized P&L. Do not feed
   display-rounded values into engine accounting. Prefer integer paise or an
   explicitly specified decimal representation where practical.
3. Same-bar stop-loss/take-profit touches require deterministic precedence with
   a documented conservative default. OHLC alone cannot reveal tick order.
4. Identical input and configuration must produce identical native and WASM
   results, with any numerical tolerance explicitly specified and tested.
5. Define exchange calendars, risk-free rate, sampling, annualization, missing
   bars, and non-trading days before publishing performance metrics.
6. Every reported financial result must be derived from engine output; static
   example values cannot masquerade as actual returns, trades, or risk metrics.

The P2 work order must approve exact execution precedence, gap/last-bar behavior,
money and fee arithmetic/rounding, position sizing, adjustment treatment,
calendar conventions, ledger reconciliation, and golden acceptance examples.
P0 deliberately makes no choices for these unresolved financial semantics.

For the first bounded kernel, D01–D12 resolve the approved execution choices;
QOS-0009 locks types/errors and tests E01–E13. Calendars and performance metrics
remain outside that scope. The P0 paragraph above records the original gate.
