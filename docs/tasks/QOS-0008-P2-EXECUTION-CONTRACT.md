# QOS-0008 — P2 execution-contract draft for owner review

- **Status:** APPROVED FOR DOCUMENTATION DRAFTING ONLY by the owner's 2026-10-29
  "Yes, do what our setup suggests" response to accepting P1, merging PR #7 and
  preparing this backtest execution contract for review. Proposed financial
  choices are DRAFT; neither engine implementation nor this PR's merge is approved.
- **Phase:** P2 specification preparation, after explicit aggregate P1 acceptance.
- **Branch:** `feature/qos-0008-p2-execution-contract`, from exact `origin/develop`
  `21ac34874550e89873f677cb395140e0ebbd957f` (accepted PR #7 squash merge).
- **PR target:** `develop`.
- **Risk:** Medium; financial semantics are proposed in text, never executed.

## Goal

Deliver a reviewable execution-model proposal with independently checked synthetic
arithmetic and a clear owner-decision boundary before any P2 engine is implemented.

## Read first

- `MODULE_MAP.md` and `docs/backtest-semantics.md` for existing boundaries.
- `docs/data-contract.md` for fixed candle units/time/provenance invariants.
- `docs/api-contracts.md` for the existing public API and unimplemented surfaces.
- `PROGRESS.md` / P1 packet for the accepted base and recorded proof limits.

## Allowed edits

- This card and its handoff.
- `docs/backtest-execution-v1-draft.md` — new, explicitly unapproved financial proposal.
- `fixtures/synthetic/backtest-execution-cases.md` — new invented, hand-written
  proposed acceptance cases and exact arithmetic; no runtime or market-data claim.
- `docs/backtest-semantics.md` — current status and draft link only; preserve all
  six inherited invariants and the required approval boundary verbatim.
- `PROGRESS.md` — owner's P1 acceptance, actual PR #7 merge and compact QOS-0008 evidence.
- Current-status/acceptance prose only in `README.md`, `docs/p1-acceptance.md`,
  `docs/p1-macos-validation.md`, `docs/data-contract.md`, `docs/api-contracts.md`,
  `fixtures/synthetic/README.md`, `fixtures/synthetic/normalization-cases.md`.
- `docs/tasks/QOS-0007-P1-MACOS-VALIDATION.md` — final acceptance/merge handoff only.

No production code, tests, existing CSV/JSON fixtures, manifests, dependencies,
lockfiles, toolchain, workflow, governance/master-plan or existing financial
contract/golden-body changes. Local arithmetic scratch work stays outside Git.

## Locked invariants and proposal boundary

- P1 schema/normalizer/CSV/CLI remain unchanged. Prices are integer paise; bar
  timestamps identify OPENs; no implicit timezone/calendar/data repairs.
- Preserve shared Rust ownership, deterministic behavior and completed-bar signals
  filling no earlier than the next bar open. UI must not own financial arithmetic.
- Every new sizing, fill, cost, stop/target, gap, adjustment or terminal rule is
  labeled PROPOSED until separately approved. Do not invent approved strategy rules.
- The first proposed kernel consumes explicit test signal decisions; strategy
  generation/indicators, metrics, CLI/WASM/browser APIs and broker models need
  separately approved contracts. No new public Rust signature is declared here.
- Generic example fees are model inputs, never claims about a broker or exchange.
- Hand-written examples and independent arithmetic checks do not prove an engine
  exists. Existing CI is unchanged-code regression evidence only.

## Exact implementation

1. Recheck accepted PR #7 head `da33ea0e199afe4d732afe6f45dc874a2dc8981e`, required
   checks/conversations/protected gate; squash-merge only under owner's explicit
   instruction. Verify exact merge base above and clean local fast-forward.
2. Push this scope card first. Record P1 acceptance and merged state accurately;
   preserve historical evidence and distinguish unavailable browser/WASM parity.
3. Draft bounded v1 decisions for one-symbol long-only execution: chronology,
   integer cash/shares, costs/slippage rounding, liquidity assumption, protective
   precedence, gaps/terminal marking, provenance and reconciliation. Make scope
   limits and approval choices explicit, with no runnable API or code.
4. Add synthetic acceptance examples for timing, fees/rounding, affordability,
   liquidity, same-bar/gap stops/targets, terminal positions and overflow. Check
   exact arithmetic independently using Python integer/Fraction calculations.
5. Review exact allowlist, unchanged contract/code/golden bodies, whitespace and
   sensitive patterns; push coherent checkpoints and open one PR to develop.
6. Inspect post-merge base CI and required CI on the final proposed-document head;
   verify remote SHA and clean tree; stop at owner review of the proposal.

## Verification and stop gate

- `git diff --check`: PASS; only the allowed documentation paths may differ.
- Git comparisons: production/test/build/CSV/JSON/workflow inputs identical to
  the accepted base; existing schema, signature, command and golden blocks intact.
- Exact independent arithmetic assertions for every numerical proposed case.
- Existing required `rust-quality` / `web-quality` must pass at final PR head.
  No local runtime suite rerun for prose-only work unless a new concern arises.
- Stop on push failure, reproducible defect, missing owner authority, a needed
  scope change or after two materially different failed diagnostic attempts.
- No engine, new strategy rules, real data, broker calls, contract approval,
  PR merge, P2 acceptance, release or deployment. Owner approval of this draft
  must precede a separate scoped implementation card.

## Handoff

- Status: REVIEW; explicit DRAFT/NON-EXECUTABLE proposal ready for owner review.
- P1: owner accepted reviewed head above; PR #7 merged as exact branch base.
- Local checks: Python integer/Fraction assertions passed numerical expectations
  for E01–E13 (not engine behavior). Exact allowlist/immutable-body/code/table,
  link, sensitive-pattern and whitespace checks PASS. Runtime suites NOT RERUN
  for prose-only work; all executable inputs compare unchanged with the base.
- Proposal checkpoint: `17a402724555ad4bba38c89e63d973bcd2128f2f`, pushed/verified.
  Required Rust/web jobs passed in PR CI `40865611951` and push CI `40865579325`;
  unchanged-code regression evidence only. Base post-merge CI `40864207895` passed
  at `21ac34874550e89873f677cb395140e0ebbd957f`.
- PR: https://github.com/kushalshah7/QuantOS/pull/8 to develop; final documentation-
  handoff SHA/checks recorded in its description. Existing backtest invariants,
  P1 financial/schema/API/golden bodies and CSV/JSON fixtures preserved.
- Next decision: owner review of the explicit P2 proposal; no engine authorization.
