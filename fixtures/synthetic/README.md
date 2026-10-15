# Synthetic contract examples

All rows were invented for this repository. `SYNTH` and `TEST` are fictional;
timestamps start at the Unix epoch or use a deliberately ambiguous local time.
These files contain no observations from a market, broker, or proprietary feed.

- `valid-candles.csv`: three consistent, increasing UTC bars.
- `invalid-ohlc.csv`: intentionally inconsistent high/low and open/close.
- `duplicate-timestamp.csv`: intentionally repeated UTC timestamp.
- `missing-timezone.csv`: intentionally lacks an offset/timezone.
- `import-config.json`: explicit synthetic provenance/options for the native
  [CLI import example](../../docs/cli-import.md); no private inputs or defaults.
- [normalization-cases.md](normalization-cases.md): schema-1 hand-written input/
  expected-output cases for the owner-approved normalization contract, including
  exact paise/UTC arithmetic, validation, provenance and atomic failure.

Phase 0 tests verify that the CSV examples have the advertised properties. They
do not implement or certify an importer or execute the normalization cases.
QOS-0002 specifies the contract only; a later approved core task must implement
and test it. Input CSV prices are rupees; documented normalized prices are paise.
Every golden case is invented, even when testing a deliberately supplied
`synthetic: false` provenance declaration. CI does not validate the golden
arithmetic or prove that a normalizer exists.
