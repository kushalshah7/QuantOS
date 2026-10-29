# QOS-0009 — Rust-only deterministic execution kernel

- **Status:** APPROVED under the owner's 2026-10-29 delegated approval to proceed
  within the established strict scoped workflow. QOS-0008 D01–D12 were reviewed
  and accepted at head `0b8c5db63cdf8fa2d28383f557ed2475e1f5fbc2`; protected
  squash merge `56db214bfe040ea34901c303898e2381dbbd66ae` is this task's base.
- **Phase:** P2, one provided-signal kernel only; not aggregate P2 acceptance.
- **Branch:** `feature/qos-0009-execution-kernel`; PR -> `develop`.
- **Risk:** High: financial arithmetic, chronology and a new Rust-only API.

## Goal and read boundary

Implement the accepted provided-signal execution model with exact accounting,
atomic errors and executable synthetic goldens. Read the accepted model/cases,
`docs/execution-api-v1.md`, existing P1 public candle/provenance types, and the
small module router. No strategy generation or broad repository exploration.

## Allowed edits

- `crates/quantos-core/src/execution.rs`, `src/execution/**` within that crate.
- `crates/quantos-core/src/lib.rs` — export the new module and update comments only;
  preserve existing foundation status and P1 exports/behavior.
- `crates/quantos-core/tests/execution.rs`, `tests/execution_cases/**`.
- This card; `docs/execution-api-v1.md` (exact interface before code).
- `docs/backtest-execution-v1-draft.md`, `docs/backtest-semantics.md`,
  `fixtures/synthetic/backtest-execution-cases.md` — acceptance/current-implementation
  prose, references and explicit technical clarifications only; preserve D01–D12
  financial formulas/precedence and E01–E13 numerical expectations.
- `docs/api-contracts.md`, `MODULE_MAP.md`, `README.md`, `PROGRESS.md` — new surface
  routing/current capability and compact acceptance/evidence records.
- `docs/tasks/QOS-0008-P2-EXECUTION-CONTRACT.md` — acceptance/merge handoff only.

No other production/tests, P1 schemas/goldens, CSV/JSON data, dependencies,
manifests/lockfiles, toolchain, CI, CLI, browser/WASM export, broker integration,
real dataset, release or deployment edits.

## Locked interface and financial invariants

- Public entry: `quantos_core::execution::execute_v1(&ExecutionInputV1) ->
  ExecutionResultV1`; exact Serialize-only types/fields/errors/order are locked in
  `docs/execution-api-v1.md` before implementation. No JSON parser/Deserialize API.
- Input owns existing P1 `NormalizedCandleV1`/`ProvenanceV1` values, explicit
  configuration, completion declaration and one typed close decision per bar.
  Revalidate representable P1 values at this boundary; no coercion/defaults.
- Follow accepted D01–D12 and E01–E13: next-OPEN signal fills, one long position,
  fixed integer quantity, paise, explicit costs, previous-volume shared budget,
  protective precedence, terminal marking and checked/atomic reconciliation.
- No intrabar signal entries; entry-bar protection follows a successful OPEN
  entry. No use of current/future bar final volume for an earlier OPEN fill.
- Failure returns only version/status/error, never partial fills/snapshots/report.
  Pure function: no IO, clock, environment, randomness, telemetry or side effects.
- Technical details resolved explicitly in the API document: deterministic first
  error, quote checks before rejection gates, liquidity before cash rejection,
  commit-only fee/slippage totals, global fill/event sequence, contextual ignored
  decisions before terminal expiry. These refine unassigned API/audit behavior,
  not the accepted money model; document any further amendment before editing.

## Bounded independent review

This task explicitly authorizes **one read-only subagent** to review only this
card, the execution API/model/cases, new execution source/tests and the P1 type
interface. Review scope: causality, costs, overflow, atomicity, reconciliation and
golden coverage. It may inspect/check locally but must not edit, commit, push,
merge, create other agents or change scope. Main agent alone implements and ships.

## Execution and verification

1. Verify approved PR #8 exact head/checks/threads/protected merge, sync exact base,
   push this scope and exact API contract before code; record accepted status.
2. Add minimal typed API, validation, arithmetic and state machine. Keep helpers
   private; existing dependencies suffice. Add independent E01–E13 assertions.
3. Add focused boundary/causality tests: prefix invariance, current-volume mutation,
   metadata/input preservation, deterministic output, contextual signal events,
   fee/cash/liquidity rejections, protective priorities, zero/overflow values and
   unchanged P1 integration. Run `cargo test -p quantos-core --test execution --locked`.
4. Run `cargo fmt --all -- --check`, workspace Clippy with `--locked -- -D warnings`,
   `cargo test --workspace --locked`, and WASM target check. This cross-module
   public addition justifies one full local Rust acceptance run. CI runs web checks.
5. Resolve supported findings within scope, review exact diff/secret patterns,
   commit/push checkpoints immediately, open one PR to develop, inspect actual
   final-head CI and remote SHA, leave clean tree, stop at review handoff.

Stop immediately on push failure, missing authority, needed dependency/security/
financial change outside this card, or after two materially different failed
diagnostic attempts. Standing approval permits routine task decisions and recorded
technical amendments; it does not authorize new phases, live trading or releases.
Do not merge this implementation PR as an unattended follow-on or mark P2 accepted.

## Handoff

- Status: REVIEW in [PR #9](https://github.com/kushalshah7/QuantOS/pull/9) to develop;
  implementation complete, aggregate P2 acceptance pending. Do not auto-merge.
- Source/model approval: accepted PR #8 head/merge above; owner delegation recorded.
- Scope/API lock pushed first: `6c5f05af02bb75e33bba2b0afaa6ab6f8bfd5d54`.
  Implementation `869307d05433c48ff9987f1a7bb16523b2140d96`; final test checkpoint
  `4b8aae566b4fc6856f1c560021e743cf953b4fd4`, both pushed and remotely verified.
- Local PASS: focused execution 38/38 before final coverage addition; final
  workspace 107/107 (39 execution), fmt, Clippy with warnings denied and WASM
  target compile. One read-only reviewer found no source blocker; the suggested
  entry-bar slipped-stop case was added and passed. No production fix required.
- Code-checkpoint PR CI `40869656105` and push CI `40869650711` PASS at `4b8aae5`.
  Actual PR output inspected: 107 Rust tests plus web lint/typecheck/test/build.
  Final documentation-handoff SHA/checks are in PR #9; no known functional blocker.
- Scoped diff/secret/link checks PASS; D01–D12 financial formulas and E01–E13
  numerical rows preserved; P1 sources/tests/fixtures and build inputs unchanged.
- Limits: no strategy/indicator generation, CLI/UI binding, metrics or real-data result.
