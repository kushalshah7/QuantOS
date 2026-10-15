# Backtest constraints

Status: inherited architectural invariants only. No P0 backtest engine, metrics,
strategy, price conversion, fill model, or financial result exists.

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
