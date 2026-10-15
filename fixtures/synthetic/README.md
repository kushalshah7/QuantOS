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

Phase 0 fixture tests verify the examples' advertised properties. QOS-0002
specified normalization; QOS-0003 implemented it, QOS-0004 added CSV parsing and
QOS-0005 exposes native CLI import. The Rust normalization/CSV suites and actual
CLI subprocess tests now check these synthetic expectations; see the
[P1 acceptance packet](../../docs/p1-acceptance.md) for immutable test links and
implementation-base CI output. Input CSV prices are rupees; normalized prices
are integer paise. Markdown goldens remain hand-written specifications.
Every golden case is invented, even when testing a deliberately supplied
`synthetic: false` provenance declaration. CI executes Rust assertions, not the
Markdown, and does not independently validate document arithmetic or provenance
accuracy/rights. Office-laptop runtime and personal-machine hands-on checks,
macOS and browser/native-WASM runtime parity are NOT RUN. WASM evidence is
compilation only; P1-wide acceptance remains pending.
