# QOS-0004 — Parse CSV bytes and verify the normalization pipeline

- **Status:** APPROVED (owner submitted the QOS-0004 prompt)
- **Phase:** P1 subtask only; not complete-P1 acceptance
- **Branch:** `feature/qos-0004-csv-parser` (from updated `origin/develop`)
- **PR target:** `develop`
- **Risk:** medium (byte offsets, quoting and atomic parser failure)
- **Verified base:** accepted PR #3 squash merge and unchanged remote develop
  `a38ddfbb998f855faf8a12480d60761c44d1755c`.
- One agent; clean isolated worktree. Leave the original checkout's untracked
  owner prompts/content untouched. QOS-0003 prompt was absent when filesystem
  preservation was checked; QOS-0004 prompt was present and hashed read-only.

## Goal
Parse actual UTF-8 CSV bytes into exact field strings and prove through new
integration tests that those tables feed the accepted normalizer correctly.

## Read first (maximum 2–5 relevant files/groups)
- `MODULE_MAP.md`
- `docs/data-contract.md`
- `docs/api-contracts.md`
- Core data exports/result types
- Existing synthetic CSV fixtures and relevant test interfaces

Inspect the task template, governance and progress as needed, not the entire repo
or MASTER_PLAN. `docs/csv-contract.md` records the owner's locked CSV policies.

## Allowed edits
- `crates/quantos-core/src/data.rs` (CSV module and public re-exports only)
- `crates/quantos-core/src/data/csv.rs`, `crates/quantos-core/src/data/csv/**`
- `crates/quantos-core/tests/csv.rs`, `crates/quantos-core/tests/csv_cases/**`
- `docs/csv-contract.md`
- `docs/api-contracts.md` (CSV API and remaining limits)
- `MODULE_MAP.md` (CSV route/boundary note only)
- `docs/tasks/QOS-0004-P1-CSV-PARSER.md` (create first, then handoff)
- `PROGRESS.md` (accepted QOS-0003 evidence and compact QOS-0004 entry)

All other paths are read-only: normalization implementation/types/codes and tests,
accepted data/golden contracts, fixtures, manifests/lockfile/toolchain/workflow,
CLI, WASM and web. No dependencies, private data or public combined-import API.

## Locked contract / invariants
- `quantos_core::data::parse_csv_v1(input: &[u8]) -> CsvParseResultV1`.
- Re-export `CsvParseResultV1`, `CsvParseIssueV1`, `CsvParseCodeV1` from data.
  Use existing Serde, Serialize only. Result is a tagged Rust enum with exactly
  the success/failure JSON shapes in `docs/csv-contract.md`.
- Closed CSV-only codes: INVALID_UTF8, EMPTY_INPUT, UNEXPECTED_QUOTE,
  INVALID_AFTER_QUOTE, UNTERMINATED_QUOTED_FIELD, INVALID_RECORD_SEPARATOR.
  Never alter the 25 normalization codes.
- Validate entire input UTF-8 first: errors have row null and original-byte
  `valid_up_to()` offset, preceding all CSV syntax errors.
- Remove exactly one leading BOM at byte zero; empty/BOM-only input errors at
  original length. Preserve every other BOM character.
- Comma delimiter only; outside quotes accept LF/CRLF, mixed allowed; bare CR
  errors at that CR. No guessing, comments or spreadsheet separator directives.
- Quote opens only at field start; double quotes unescape to one inside quotes.
  Preserve all other content, including CR/LF/CRLF, NUL and Unicode, exactly.
- Quotes in unquoted fields error there; after closing quote only delimiter,
  valid separator or EOF. Other bytes error there, except bare CR remains the
  record-separator error. Spaces have no special privilege.
- Unclosed quote errors at original EOF. After UTF-8 validation emit only the
  first syntax error in byte order; no recovery, skipped rows or partial table.
- Preserve empty/trailing fields and blank records; final separator adds no
  phantom record. Quoted empty/header-only input is parser success.
- First record is headers; rows follow. Parser does not validate header names,
  widths, identity, data presence or numeric/time semantics.
- Header record is 1; increment only at separators outside quotes. Embedded
  newlines do not advance record number. Syntax error row is current record.
- Preserve exact strings except CSV quote unescaping/single initial BOM removal.
  No trimming, folding, Unicode/numeric conversion or missing-value substitution.
- Small linear parser; no IO, network, unsafe code, persistence or generic framework.

## Exact implementation
1. Create and push this card plus `docs/csv-contract.md` before implementation;
   this approved checkpoint requires no additional approval gate.
2. Implement the CSV-only API/types and add only module/re-export declarations
   to the existing data module. Keep normalization behavior unchanged.
3. Transcribe CSV01–CSV12 with independent expectations and add focused BOM,
   multibyte-offset/UTF-8-precedence, separator/quote/space, empty/blank/header-only,
   multiline-row and determinism cases. Assert complete serialized union shapes.
4. In new tests only, compose successful parsed fields with explicit metadata,
   options and schema_version into normalize_v1. Reuse read-only fixtures through
   include_bytes!: exact valid three candles, invalid OHLC, duplicate timestamp,
   missing timezone and confirmed Kolkata (13500000). Also test quoted canonical
   fields, preserved multiline source, blank row -> ROW_WIDTH at row 2, and no
   partial table/normalization on malformed CSV.
5. Format with existing pinned rustfmt, inspect scoped diffs/sensitive content,
   push coherent checkpoints and open one PR to develop. Stop on push failure.
6. Inspect final-head CI test output proving new CSV/pipeline tests and unchanged
   normalization/P0 suites ran; record concise evidence and stop before merge.

## Verification and proof
- LOW-RESOURCE: local compilation/tests/builds, installations, servers, benchmarks
  and real dataset processing NOT RUN/forbidden. Only existing pinned rustfmt,
  lightweight text/Git inspection and `git diff --check` are permitted.
- Existing CI remains unchanged: Rust fmt, Clippy, `cargo test --workspace --locked`,
  WASM target compilation, and existing web locked install/lint/typecheck/test/build.
- Actual final-head `tests/csv.rs` and `tests/normalization.rs` output must be
  inspected, not just a badge or earlier-head results. P0 tests remain intact.
- Parser success is syntactic success, never a valid-market-data assertion.
- WASM evidence is compilation only; macOS/browser/native-WASM parity NOT RUN.

## Review/stop gate
- Allowed paths only; required final-head checks green; actual suites verified;
  remote SHA verified and one PR to develop ready for owner review.
- Follow CODEX_PROTOCOL bounded-fix rule; missing semantics/out-of-scope changes
  require BLOCKED with one proposed amendment. Never widen API/normalizer policy.
- No merge/release/deployment, CLI/browser work, P1 acceptance or another task.

## Handoff (Codex fills after work)
- Status: REVIEW (implementation delivered; owner acceptance pending)
- Changed paths: approved data-module declarations, CSV module and new CSV test/
  support files, CSV/API contracts, module router, this card and PROGRESS only.
  Normalizer, existing tests/fixtures, manifests/lockfile/workflow unchanged.
- Local tests/builds: NOT RUN (LOW-RESOURCE). Existing pinned rustfmt/formatter
  check succeeded; staged diff check exited 0; sensitive-pattern inspection found
  no matching filenames. No compilation, test execution or installation locally.
- CI / CSV-pipeline execution evidence: implementation-head
  [push CI](https://github.com/kushalshah7/QuantOS/actions/runs/38714803613) and
  [PR CI](https://github.com/kushalshah7/QuantOS/actions/runs/38714949929) passed.
  Actual [Rust log](https://github.com/kushalshah7/QuantOS/actions/runs/38714803613/job/116354776582)
  inspected: tests/csv.rs 30 passed; tests/normalization.rs 20 passed; P0 fixture/
  CLI tests passed. CSV01–CSV12, focused edges and all pipeline tests ran.
  Final handoff head/CI and actual final-head test output are recorded in PR #4.
- Remote commit SHA: verified implementation checkpoint
  `1ca06457830e036d8fb31982e98bc9ac35f5a333`; subsequent handoff's final SHA is
  recorded in PR #4 to avoid self-referential commit hashes.
- PR link: https://github.com/kushalshah7/QuantOS/pull/4 -> develop; unmerged.
- Remaining risks / owner review: no known functional blocker; owner acceptance
  pending, not P1 acceptance. macOS/browser/native-WASM runtime parity NOT RUN.
  Original checkout is not edited; prompt availability at preflight is above.
