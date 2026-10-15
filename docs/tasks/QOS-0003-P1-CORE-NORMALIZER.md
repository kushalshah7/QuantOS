# QOS-0003 — Implement the schema-1 Rust candle normalizer

- **Status:** APPROVED (owner submitted QOS-0003 implementation prompt)
- **Phase:** P1 subtask only; not P1 acceptance
- **Branch:** `feature/qos-0003-core-normalizer` (from updated `origin/develop`)
- **PR target:** `develop`
- **Risk:** high (time, exact prices and atomic data validation)
- **Verified prerequisite:** PR #2 accepted head
  `379f688da0c807cbed8f62b1f6895363f405f31d`, squash merge and unchanged remote
  `develop` SHA `452bde68a8e96210919be97bf0cdd78dea53700c`.
- Use one agent and a clean isolated checkout; preserve the owner's untracked
  instruction file in the original checkout without adding it to this task.

## Goal
Implement the accepted schema-1 in-memory candle normalizer in Rust and prove its
serialized results, error collection and determinism against all golden cases.

## Read first (maximum 2–5 relevant files/groups)
- `MODULE_MAP.md`
- `docs/data-contract.md`
- `fixtures/synthetic/normalization-cases.md`
- `docs/api-contracts.md`
- `crates/quantos-core/src/lib.rs`, existing core tests and core `Cargo.toml`

Inspect the task template, relevant Cargo configuration, `PROGRESS.md` and existing
quality workflow as needed; no repository-wide or MASTER_PLAN read.

## Allowed edits
- `crates/quantos-core/src/lib.rs` (export data module; retain foundation API)
- `crates/quantos-core/src/data.rs`, `crates/quantos-core/src/data/**`
- `crates/quantos-core/tests/normalization.rs`
- `crates/quantos-core/tests/normalization_cases/**` (synthetic cases/support)
- `crates/quantos-core/Cargo.toml`
- `Cargo.lock` (necessary dependency resolution only)
- `docs/api-contracts.md` (Rust-only API and verification limits)
- `MODULE_MAP.md` (data entry and stale skeleton note only)
- `docs/tasks/QOS-0003-P1-CORE-NORMALIZER.md` (create first, then handoff only)
- `PROGRESS.md` (accepted QOS-0002 evidence and compact QOS-0003 handoff)

All other files are read-only, including the data contract, golden specification,
existing P0 tests, toolchain/workspace manifests, workflow, web, WASM and CLI.

## Locked contract / invariants
- The sole new callable domain entry point is
  `quantos_core::data::normalize_v1(input: &serde_json::Value) -> NormalizationResultV1`.
- Export typed results/supporting types from `data`; use closed status, gap,
  issue-code/severity and provenance enums. Serde serialization must match the
  accepted schema's fields, strings, nulls, arrays and union shapes exactly.
  No Deserialize API, CSV/JSON-text parser, WASM export or CLI command.
- Raw Value input permits all independently detectable missing/type/unknown-field
  errors; no strict-deserialization fail-fast shortcut.
- Implement all 25 issue codes, dependency suppression, deterministic issue order,
  metadata preservation, one exact identity pair, `5m` bar OPEN timestamps,
  explicit timezones, Gregorian dates, atomic failure and explicit-grid gaps.
- Use exact rupee-to-paise parsing without floats/rounding. Handle arbitrarily
  long numeric strings through bounded arithmetic, preserving syntax/precision/
  range precedence. Preserve JavaScript-safe integer and timestamp bounds.
- Never use the host timezone, infer calendars, sort/drop rows or disclose raw
  private field values in messages. Stable messages for identical input.
- Approved new direct dependencies only: `serde` with derive and `serde_json`,
  stable 1.x, exactly pinned. Registry-selected candidates are `serde =1.0.229`
  and `serde_json =1.0.154`; verify resolution. Necessary transitives allowed;
  preserve unrelated locked versions. No other direct dependency/build script/
  unsafe code. Retain all foundation signatures and behavior.

## Exact implementation
1. Commit/push this approved card before implementation; stop on push failure.
2. Add the typed normalizer and small internal parsing/validation helpers.
3. Transcribe every golden ID and every alternative into independently expected
   executable cases; assert full serialized structure and ordered issue keys,
   with nonempty messages rather than message-prose assertions.
4. Add focused shape/dependency, header/mapping, metadata, invalid confirmation,
   usable-sibling/identity, timestamp-bridge, leap/date/boundary, precision/range,
   long-number/negative-zero, Unicode-order, multi-error/gap and repeat-call tests.
5. Test TZ independence in isolated subprocesses with TZ=UTC/Asia/Kolkata;
   never mutate parent environment or add unsafe code. Keep existing tests intact.
6. Resolve/pin approved dependencies and format with existing pinned tools only;
   review allowed diffs/sensitive content; push coherent checkpoints immediately.
7. Open one PR, verify final-head CI and actual normalization-suite output,
   update compact evidence and stop before merge.

## Verification and proof
- LOW-RESOURCE: local compilation, tests/builds, servers, benchmarks, dataset
  processing and toolchain installation NOT RUN/forbidden.
- Only local tool exceptions: already installed pinned rustfmt and Cargo
  lockfile resolution. Invoke existing binaries directly to avoid rustup downloads.
  If unavailable or a valid lockfile cannot be produced, stop BLOCKED.
- `git diff --check`, scoped allowed-path and sensitive-content inspection.
- Existing CI (unchanged): `cargo fmt --all -- --check`;
  `cargo clippy --workspace --all-targets --locked -- -D warnings`;
  `cargo test --workspace --locked`;
  `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked`;
  locked npm installation and existing web lint/typecheck/test/build commands.
- Inspect test output proving the new integration suite and golden/edge/TZ
  assertions actually ran on the final pushed head, not a skeleton-only badge.
- WASM compilation is not browser execution/native-WASM parity; macOS NOT RUN.

## Review/stop gate
- Required final-head rust-quality/web-quality CI green; correct suite executed;
  allowed paths only, remote SHA verified and PR to develop open for review.
- Stop on push failure or missing tools/permissions. After two materially
  different CI diagnosis/fix attempts, stop with exact evidence and next action.
- Genuinely unspecified observable semantics require a proposed amendment and
  BLOCKED handoff, never editing the accepted contract to suit implementation.
- No merge/release/deployment, CSV parsing, browser integration or next task.

## Handoff (Codex fills after work)
- Status: NOT STARTED
- Paths changed:
- Dependency versions: registry candidates 1.0.229 / 1.0.154; resolution pending.
- Local tests/builds: NOT RUN (LOW-RESOURCE).
- CI / normalization execution evidence:
- Remote commit SHA:
- PR link:
- Risks / next decision: implementation delivered for review, not P1 accepted.
