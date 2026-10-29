# P1 data acceptance packet

**Decision: READY FOR OWNER REVIEW. P1 acceptance is pending.**
Evidence rechecked 2026-10-15 at implementation SHA
`0568b5ec5b77e23e484baee012156557a81d8236` (accepted PR #5 squash merge).
This documentation task grants no hands-on waiver, phase acceptance or P2 authority.

**2026-10-29 verification update:** [Personal-Mac validation](p1-macos-validation.md)
executed the synthetic checklist at that exact implementation SHA. All four import
cases and web checks passed. A raw-byte filename fixture fails during file
creation on macOS; the complete Rust run has 66 passes and one failure. Owner
acceptance remains pending; green Ubuntu CI does not waive this local blocker.

## Scope and automated evidence

The implemented native path is CSV bytes -> explicit normalization -> CLI JSON
data-quality report: one symbol/exchange, 5m bar OPENs, decimal rupees to integer
paise, declared UTC milliseconds/ISO timestamps and explicit timezone confirmation,
declared provenance, and gap warnings only against an explicit valid expected grid.
No calendar inference, sorting/deduplication, partial candles on error or fabricated bars.

[Post-merge run 38727027369](https://github.com/kushalshah7/QuantOS/actions/runs/38727027369)
is a **push run at the implementation SHA**, distinct from older PR-head runs.
Both required jobs succeeded: [rust-quality][R] and [web-quality][W]. Actual Rust
output shows 11 CLI import, 2 output-writer, 30 CSV/pipeline and 20 normalization
tests passed, plus CLI smoke, 2 fixture tests and the adapter test. These counts
are an observed snapshot, never assertions about future suite size.

| Criterion | Contract | Actual executable evidence at the implementation SHA | Observed CI result |
| --- | --- | --- | --- |
| CSV importer fidelity and malformed bytes | [CSV contract][C] | [CSV goldens][CG] CSV01–CSV12; [pipeline][CP] `csv_pipeline_malformed_bytes_never_return_partial_table_or_normalize` | [R]: PASS; parser failures expose no partial table. |
| CLI/config validation and precedence | [CLI contract][I] | [CLI cases][CLI] `json_envelope_duplicates_encoding_trailing_and_nesting_errors`, `io_and_config_csv_precedence_are_sanitized`, `exact_limits_are_allowed_and_oversize_precedes_parsing` | [R]: PASS; actual CLI subprocess reports/exits checked. |
| Exact prices, volume and bounds | [Data contract][D] | [Normalization goldens][NG] `n01_n02_canonical`, P01–P06/V01–V03 and B01–B06 groups | [R]: PASS; rupees/paise, precision and safe-integer/time boundaries asserted. |
| Explicit UTC/ISO/Kolkata handling | [Data contract][D] | [Normalization goldens][NG] T01–T13; [TZ subprocess tests][TZ] `host_timezone_independence_in_isolated_subprocesses` | [R]: PASS; configured time alternatives/conflicts and host-TZ independence checked. |
| Known bad OHLC, duplicates and ordering | [Data contract][D] | [Normalization goldens][NG] D01–D08/A01–A03; [CLI cases][CLI] `atomic_domain_failures_and_explicit_kolkata_confirmation` | [R]: PASS; INVALID_OHLC/DUPLICATE_TIMESTAMP and domain failure dispositions checked. |
| Atomic failure and independent errors | [Data contract][D], [CSV contract][C] | [Normalization goldens][NG] A01–A03; [normalization edges][NE] `malformed_rows_and_cells_keep_independent_siblings`; [pipeline][CP] atomic failure cases | [R]: PASS; no normalized candles on any error. |
| Stable issues and deterministic messages | [Data contract][D], [CLI contract][I] | [Normalization edges][NE] `unicode_codepoint_issue_order_is_not_utf16_or_insertion_order`, `repeated_full_results_and_messages_are_deterministic`; [CLI cases][CLI] `repeated_calls_and_success_failure_leave_inputs_unchanged` | [R]: PASS; issue keys/order and repeated full results checked; descriptive messages are not golden assertion keys. |
| Source/provenance preservation | [Data contract][D] | [Normalization goldens][NG] S01–S12; [normalization edges][NE] `metadata_properties_preserve_valid_source_dependencies`; [CLI cases][CLI] `parser_failure_and_quoted_multiline_source_are_forwarded` | [R]: PASS for declared values; rights and adjustment accuracy are not verified. |
| Explicit-grid gap warnings | [Data contract][D] | [Normalization goldens][NG] G01–G06; [CLI cases][CLI] `gap_warnings_are_success_and_preserve_exact_missing_list` | [R]: PASS; supplied-grid missing list preserved, warnings succeed, absent grid means not_performed. |
| Synthetic-only examples | [Synthetic declarations](../fixtures/synthetic/README.md) | [Fixture tests][F] `valid_fixture_has_sorted_consistent_synthetic_bars`, `invalid_examples_retain_their_intended_contract_errors`; [CLI cases][CLI] valid fixture import | [R]: PASS for fixture assertions; invented origin is declared, not certified by CI. |

All source/test links below pin the immutable implementation SHA. Markdown goldens
remain hand-written specifications; Rust tests transcribe expectations. CI executes
those assertions, not Markdown, and does not independently validate document arithmetic.

## Accepted subtask trail

GitHub metadata independently confirms PRs #1–#5 merged into develop. The owner
accepted each listed head in instructions; this does not claim formal GitHub approvals.

| Subtask / merged PR | Owner-accepted head | Squash merge |
| --- | --- | --- |
| Contract [#2](https://github.com/kushalshah7/QuantOS/pull/2) | `379f688da0c807cbed8f62b1f6895363f405f31d` | `452bde68a8e96210919be97bf0cdd78dea53700c` |
| Normalizer [#3](https://github.com/kushalshah7/QuantOS/pull/3) | `6a7030eb0d08433b1bfddd59cb30b025d4498573` | `a38ddfbb998f855faf8a12480d60761c44d1755c` |
| CSV [#4](https://github.com/kushalshah7/QuantOS/pull/4) | `5200238d6b7faf5a8d7d45c6703b2ebbc7f4e728` | `46679d2f6ce069513a7b0efd845d10770be27a04` |
| Native CLI [#5](https://github.com/kushalshah7/QuantOS/pull/5) | `a393e59aaae05ca9f53c1b7fd6522269e90f0022` | `0568b5ec5b77e23e484baee012156557a81d8236` |

## Verification status and remaining gate

| Verification surface | Status / limit |
| --- | --- |
| Ubuntu implementation CI | PASSED at the implementation SHA; [Rust][R] and [web][W] jobs. |
| Office-laptop runtime compilation/tests/builds/CLI | NOT RUN (LOW-RESOURCE); documentation task uses text/Git only. |
| Manual personal-machine validation | Executed by Codex on the owner's Mac on 2026-10-29; all four synthetic imports matched expected values/exits/provenance. See the [dated receipt](p1-macos-validation.md); owner review remains pending. |
| macOS | Rust suite FAILED (66 passed / one filename-fixture setup failure); formatting, Clippy, WASM target, native smoke and web checks passed. See the [dated receipt](p1-macos-validation.md). |
| WASM target | Compilation PASSED via `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked` in [R]. |
| Browser/native-WASM runtime parity | NOT RUN; browser import bindings/worker integration are unimplemented. |
| Written owner reviewer acceptance | PENDING; supplying this checklist is not reviewer signoff. |

MASTER_PLAN [section 1 item 9 and section 6](../MASTER_PLAN.md#6-milestones-and-approval-gates)
require automated tests, hands-on validation and written reviewer acceptance.
Accepted subtasks, Ubuntu CI and recorded Mac imports leave the macOS test blocker
and written reviewer gate pending. A separate
explicit owner decision is required; aggregate P1 is not ACCEPTED/COMPLETE.
Charts, browser bindings, backtesting, financial metrics and deployment remain
unimplemented later-phase work; UI/WASM still expose P0 surfaces. Real historical
data is unnecessary for these synthetic P1 checks and has not been validated.

Owner reviewer checklist (all items pending; review/signoff NOT RUN):

- [ ] Match the accepted heads, implementation SHA and actual CI output above.
- [ ] Review criterion-to-test coverage and distinguish sampled assertions from untested claims.
- [ ] Record the personal-machine observations below, including failures and provenance.
- [ ] Confirm the unchanged contracts, synthetic-only public examples and verification limits.
- [ ] Write an explicit P1 acceptance/deferral decision; do not infer a waiver or authorize P2 here.

## Original personal-machine checklist — historical NOT RUN instructions

Use a clean personal-machine checkout and a POSIX shell; do not run this checklist
on the office laptop. Commands below are instructions only, not executed evidence.
These original QOS-0006 instructions and expected values are preserved below;
their NOT RUN labels describe that historical documentation task. Actual
2026-10-29 commands and observations are in the [Mac validation receipt](p1-macos-validation.md).

1. **Checkout — NOT RUN.** Stop if the initial status shows local work. Fetch and
   detach at the exact implementation SHA; `rev-parse` must match it.
   ```sh
   git status --short
   git fetch origin --prune
   git switch --detach 0568b5ec5b77e23e484baee012156557a81d8236
   git rev-parse HEAD
   ```
2. **Valid import — NOT RUN.** Use the [existing CLI example][I]:
   ```sh
   cargo run -p quantos-cli --locked -- import --csv fixtures/synthetic/valid-candles.csv --config fixtures/synthetic/import-config.json
   echo "$?"
   ```
   Expect exit 0, outer/inner success, stage normalization, schema_version "1",
   issues [], and gap_assessment {status: not_performed, missing_ts_utc_ms: []}.
   All candles have SYNTH/TEST/5m/source synthetic; prices below are integer paise:

   | ts_utc_ms | open | high | low | close | volume |
   | --- | --- | --- | --- | --- | --- |
   | 0 | 10000 | 10300 | 9900 | 10200 | 10 |
   | 300000 | 10200 | 10400 | 10100 | 10300 | 12 |
   | 600000 | 10300 | 10500 | 10000 | 10100 | 8 |

3. **Known invalid input — NOT RUN.**
   ```sh
   cargo run -p quantos-cli --locked -- import --csv fixtures/synthetic/invalid-ohlc.csv --config fixtures/synthetic/import-config.json
   echo "$?"
   ```
   Expect exit 1, outer/inner failure, stage normalization, candles [], gap assessment
   not_performed, and INVALID_OHLC/error at CSV row 2 with field null. Do not assert message wording.
4. **Explicit timezone — NOT RUN.** In a fresh temporary directory, create this
   synthetic-only config; no repository fixture is edited:
   ```sh
   qos_review_dir="$(mktemp -d)"
   cat > "$qos_review_dir/naive.json" <<'JSON'
   {"schema_version":"1","metadata":{"source":"synthetic","synthetic":true,"adjustment_status":"unknown","redistribution_status":"unknown"},"options":{"timestamp_column":"timestamp","timestamp_format":"iso8601","confirmed_timezone":null,"expected_timestamps_utc_ms":null}}
   JSON
   cargo run -p quantos-cli --locked -- import --csv fixtures/synthetic/missing-timezone.csv --config "$qos_review_dir/naive.json"
   echo "$?"
   sed 's|"confirmed_timezone":null|"confirmed_timezone":"Asia/Kolkata"|' "$qos_review_dir/naive.json" > "$qos_review_dir/kolkata.json"
   cargo run -p quantos-cli --locked -- import --csv fixtures/synthetic/missing-timezone.csv --config "$qos_review_dir/kolkata.json"
   echo "$?"
   ```
   First expect exit 1, candles [], TIMEZONE_REQUIRED/error at row 2/field timestamp.
   With explicit Asia/Kolkata, expect exit 0, issues [], one candle at 13500000 UTC ms
   (09:15 Kolkata = 03:45 UTC), OHLC 10000/10300/9900/10200 and volume 10; no inferred timezone.
5. **Provenance inspection — NOT RUN.** In successful result.metadata expect source
   synthetic, synthetic true, adjustment_status unknown and redistribution_status
   unknown; candle source stays synthetic. Record declarations, not verified rights
   or adjustment accuracy. Record commands, checkout SHA and observations for owner review.

[Documentation PR #6](https://github.com/kushalshah7/QuantOS/pull/6) tracks this packet;
its final-head CI is documentation regression evidence, separate from the runtime snapshot.

[D]: data-contract.md
[C]: csv-contract.md
[I]: cli-import.md
[R]: https://github.com/kushalshah7/QuantOS/actions/runs/38727027369/job/116395316604
[W]: https://github.com/kushalshah7/QuantOS/actions/runs/38727027369/job/116395317012
[NG]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-core/tests/normalization_cases/golden.rs
[NE]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-core/tests/normalization_cases/edges.rs
[TZ]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-core/tests/normalization.rs
[CG]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-core/tests/csv_cases/golden.rs
[CP]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-core/tests/csv_cases/pipeline.rs
[CLI]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-cli/tests/import_cases/cases.rs
[F]: https://github.com/kushalshah7/QuantOS/blob/0568b5ec5b77e23e484baee012156557a81d8236/crates/quantos-core/tests/fixtures.rs
