# QOS-0002 — Lock the minimal candle normalization contract

- **Status:** APPROVED FOR DOCUMENTATION ONLY (owner instruction, 2026-10-15)
- **Phase:** P1 contract preparation; no normalizer implementation
- **Branch:** `feature/qos-0002-data-contract` (from updated `origin/develop`)
- **PR target:** `develop`
- **Risk:** medium
- **Prerequisite met:** owner accepted P0 head
  `9643c2c042b7f725a9ed69368f2551b6b6abb013`; PR #1 was squash-merged through
  GitHub's protection gate as `4e0ae302146f417eb142f908303a86799420cc05`.

## Goal
Specify complete version-1 candle normalization schemas, deterministic validation
and synthetic golden cases so a later approved core task can implement them.

## Read first (maximum 2–5 relevant files)
- `MODULE_MAP.md`
- `docs/data-contract.md`
- `docs/api-contracts.md`
- `fixtures/synthetic/README.md`
- `templates/TASK_TEMPLATE.md`

## Allowed edits
- `docs/data-contract.md`
- `fixtures/synthetic/normalization-cases.md` (new golden specification)
- `fixtures/synthetic/README.md` (reference and fixture scope)
- `docs/tasks/QOS-0002-P1-DATA-CONTRACT.md` (create first, then handoff only)
- `PROGRESS.md` (accepted P0 evidence and one short QOS-0002 entry)

No other files, runtime code, dependencies, lockfiles, workflow changes or datasets.

## Locked contract / invariants
- One symbol/exchange per dataset; only `5m` bars, with timestamps at bar OPEN.
- Support existing canonical CSV headers plus explicit timestamp-column mapping.
  Never guess a header, unit or timezone.
- Explicit integer UTC milliseconds, ISO with Z/offset, or naive ISO with caller-
  confirmed UTC/Asia/Kolkata. Reject conflicting declarations. No host timezone.
- Normalized time range: `0 <= ts_utc_ms < 4102444800000`.
- Parse decimal prices exactly into positive integer paise (`100` -> `10000`).
  At most two meaningful fractional digits; trailing zeros allowed. Never round.
- Volume is a nonnegative integer. All normalized integers must be JavaScript-safe
  integers. Reject malformed, nonfinite and out-of-range numbers.
- Require `low <= open/close <= high`; reject duplicates and descending order.
  Never sort, deduplicate or drop rows silently.
- Atomic results: any error returns no candles; collect independently detectable
  errors in a documented deterministic order.
- Dataset metadata: nonempty source label; synthetic boolean; adjustment status
  `unknown | unadjusted | split_adjusted | total_return_adjusted`; redistribution
  status `unknown | private_only | permitted`. Preserve declarations without
  claiming verification of rights or adjustments.
- Assess gaps only against an explicitly supplied valid timestamp grid. Otherwise
  report not performed. Never infer calendars or fabricate candles.
- Use `schema_version: "1"`; specify typed input/options/output and stable issue
  codes, severities, nullable CSV row/field and descriptive messages. Header row
  is 1. Messages are not assertion keys.
- Documentation interfaces only: preserve all existing P0 runtime interfaces.
  No CSV parser, normalizer, WASM/browser binding or financial engine is created.

## Exact implementation
1. Create this approved card before editing the contract or golden cases.
2. Define the complete schemas, accepted lexical formats, validation dependencies,
   stable issue codes, deterministic ordering and atomic success/failure rules.
3. Add hand-written synthetic inputs and exact normalized values/issues, including
   UTC/IST, missing timezone, OHLC, duplicates/order, explicit-grid gaps, zero and
   negative volume, precision/trailing zeros, integer/time bounds, timezone
   conflicts and multiple-error atomic failure.
4. Link the golden specification from the fixture README; keep P0 fixture QA
   separate from future normalizer acceptance.
5. Review only allowed diffs, push coherent checkpoints, open one PR to develop,
   inspect existing CI and update a compact evidence handoff. Stop before merge.

## Verification and proof
- Lightweight text/diff inspection, including `git diff --check`, allowed-path
  review and manual arithmetic/schema/case review. These are not runtime tests.
- LOW-RESOURCE: local test/build/toolchain checks **NOT RUN**. One agent; no
  installs, compilation, builds, dev servers, browser automation, benchmarks or
  dataset processing.
- Existing GitHub Actions regression commands (CI only):
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --locked -- -D warnings`
  - `cargo test --workspace --locked`
  - `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked`
  - `npm ci --ignore-scripts`
  - `npm run lint`
  - `npm run typecheck`
  - `npm test`
  - `npm run build`
- Demonstration: a reviewer can derive every golden output from the written
  contract, distinguish input rupees from output paise and locate each issue by
  code/severity/row/field. No unresolved policy placeholders.
- CI tests the unchanged skeleton; it does not validate documentation arithmetic
  or prove that a normalizer exists. macOS verification remains NOT RUN.

## Review/stop gate
- Only the five allowed paths changed; exact cases and schemas reviewed.
- Required CI green on the final head; checkpoints pushed and PR opened.
- No QOS-0002 merge, release, runtime implementation or subsequent task.
- Missing owner semantics or a necessary out-of-scope change: stop BLOCKED and
  request an amendment. Do not invent financial or calendar behavior.

## Handoff (Codex fills after work)
- Status: NOT STARTED
- Paths changed:
- Tests/results: local test/build checks NOT RUN (LOW-RESOURCE).
- Remote commit SHA:
- PR link:
- Risks / next decision: owner review; runtime implementation requires a new task.
