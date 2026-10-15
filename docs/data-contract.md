# Candle and provenance contract

Status: architecture constraints from `MASTER_PLAN.md`; no P0 importer.

Normalized records must carry `symbol`, `exchange`, `interval`, `ts_utc_ms`,
`open`, `high`, `low`, `close`, `volume`, and source/provenance metadata.

- Internal time is UTC milliseconds. Exchange-session interpretation for India
  explicitly uses `Asia/Kolkata`; never infer the machine's local timezone.
- Input mappings must be documented. Missing input timezone requires a user
  choice/confirmation before normalization; ambiguous times cannot silently
  become UTC. P0's `missing-timezone.csv` illustrates this future requirement.
- Validate ordering, duplicate times, OHLC consistency, nonnegative volume,
  finite prices, and expected missing bars. Report gaps; do not fabricate bars.
- Adjustment/provenance metadata must communicate split/dividend ambiguity.
  Holding a third-party dataset does not establish redistribution rights.
- Public fixtures are invented and explicitly marked synthetic. Private inputs
  remain under ignored `data/` and must never enter Git or CI artifacts/logs.

`fixtures/synthetic/` illustrates valid bars and invalid OHLC/time cases. Its
P0 tests check the fixtures themselves, not future importer behavior.

Before P1, the approved data task must lock supported intervals, input mappings,
price/volume representation and precision, provenance fields, duplicate/order
handling, timezone edge cases, missing-bar/calendar rules, and the error schema.
These details remain unspecified here; they are not permission to invent them.
