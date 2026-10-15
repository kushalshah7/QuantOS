# Synthetic contract examples

All rows were invented for this repository. `SYNTH` and `TEST` are fictional;
timestamps start at the Unix epoch or use a deliberately ambiguous local time.
These files contain no observations from a market, broker, or proprietary feed.

- `valid-candles.csv`: three consistent, increasing UTC bars.
- `invalid-ohlc.csv`: intentionally inconsistent high/low and open/close.
- `duplicate-timestamp.csv`: intentionally repeated UTC timestamp.
- `missing-timezone.csv`: intentionally lacks an offset/timezone.

Phase 0 tests verify that these examples have the advertised properties. They
do not implement or certify an importer. A later approved data task must specify
and test rejection, timezone confirmation, and data-health behavior.
