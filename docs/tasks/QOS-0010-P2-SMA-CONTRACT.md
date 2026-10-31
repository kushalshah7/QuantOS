# QOS-0010 — proposed SMA crossover contract and synthetic cases

- **Status:** APPROVED FOR DOCUMENTATION PREPARATION ONLY by the owner's explicit
  2026-10-31 request. Strategy choices remain PROPOSED and require review before
  a separate implementation work order. Neither this PR's merge nor P2 acceptance
  is authorized.
- **Phase:** P2 strategy specification preparation; one proposed SMA crossover.
- **Branch:** `feature/qos-0010-sma-contract`; PR -> `develop`.
- **Exact base:** `b4ee5eb9c6321fbc33e0c7c0a26b9aee5fc909df`, accepted PR #9
  protected squash merge of head `6c4720566ea919eaf50221ccff0fc7e107c3eef3`.
- **Risk:** Medium; new financial strategy semantics in text only.
- **Agent mode:** one agent; no subagents or delegated work.

## Goal and narrow read boundary

Prepare one reviewable, explicitly unapproved SMA crossover strategy contract
and small invented acceptance examples. Specify exact rolling arithmetic,
warm-up/crossover timing, validation/overflow and unchanged kernel integration.
Read `MODULE_MAP.md`, the relevant P2/SMA portions of `MASTER_PLAN.md`,
`docs/backtest-semantics.md`, `docs/execution-api-v1.md`,
`docs/backtest-execution-v1-draft.md` and QOS-0009's card. Public P1/decision types
may be inspected only if needed to describe the existing interface.

## Allowed edits

New Markdown files only:

- This card, including its handoff.
- `docs/strategy-sma-v1-draft.md`.
- `fixtures/synthetic/sma-crossover-cases.md`.

Existing files, acceptance/current-status/routing prose only:

- `docs/tasks/QOS-0009-P2-EXECUTION-KERNEL.md`.
- `docs/backtest-semantics.md`, `docs/api-contracts.md`.
- `MODULE_MAP.md`, `README.md`, `PROGRESS.md`.

No other edits. Preserve existing financial formulas, API signatures/types,
examples and historical evidence. No runtime source/tests, data files,
dependencies/lockfiles, tooling, CI, broker calls or private-data inspection.
Owner files, including untracked/ignored `Market-Data/` and prompts, stay intact.

## Locked preparation requirements

- Propose one-symbol, long-only, completed contiguous 5m candles using CLOSE.
  Explicit fast/slow periods with fast < slow, bounded ranges and no defaults.
- Propose exact integer rolling sums and rational comparisons, without floats or
  display rounding; resolve storage/intermediate bounds and deterministic errors.
- Recommended up-cross: previous fast <= slow and current fast > slow; down-cross:
  previous fast >= slow and current fast < slow. Specify equality precisely.
- Hold during warm-up and on first joint availability; state earliest possible
  signal, short-input behavior and exactly one decision per candle.
- Decisions are independent of portfolio state. Existing kernel rules retain
  ownership of next-OPEN fills, ignored decisions, rejected orders, protective
  exits, terminal expiry and cash/position accounting. No same-bar signal fills.
- Specify full validation, atomic failure, input preservation, determinism and
  valid-future prefix invariance. Do not infer bars, sessions or calendar repairs.
- No EMA, breakout, metrics, profitability claim, API implementation or CLI/UI/
  WASM binding. Every new strategy choice is labeled PROPOSED until reviewed.
- Synthetic examples must show exact sums, fractions, decisions and indices for
  warm-up, first availability, both crossings, equality, repeated signals,
  invalid periods, gaps, arithmetic boundaries and future-data independence.
  Include a small hand-derived integration example with next-OPEN kernel fills.

## Execution and verification

1. Verify accepted PR #9 head/squash and successful exact-merge CI, expected remote
   develop base, no existing QOS-0010 task/branch/PR and safe owner-file handling.
   Push this task card before drafting the strategy or acceptance cases.
2. Resolve each proposed strategy choice in the draft and link invented cases.
   Correct QOS-0009's current acceptance handoff while keeping aggregate P2 pending.
3. Independently recalculate examples with bounded Python standard-library
   integer/Fraction assertions; scratch calculations remain outside Git. They
   prove stated arithmetic only, not an implemented strategy or runtime parity.
4. Check the exact nine-path allowlist, immutable financial/API/example bodies,
   unchanged executable/build/data inputs, local links, whitespace and secrets.
5. Commit and immediately push coherent checkpoints; open one PR to develop.
   Inspect both required jobs and actual output at final head, verify remote SHA
   and working-tree status, and stop at the PR review gate.

LOW-RESOURCE: no installations, compilation, local suites, builds, product CLI,
benchmarks or real datasets. Existing CI is unchanged-code regression evidence;
it does not execute the proposed strategy or verify Markdown arithmetic.
Stop on push failure, an unexpected base/existing task conflict, unauthorized
needed path/interface/financial change, or two materially different unsuccessful
diagnostic attempts. No merge, implementation, aggregate P2 acceptance, release,
deployment or next task.

## Handoff

- Status: DOCUMENTATION REVIEW in
  [PR #10](https://github.com/kushalshah7/QuantOS/pull/10) to develop. All
  SD01–SD10 strategy choices remain PROPOSED; review precedes implementation.
- Preflight PASS: PR #9 accepted head/merge match above; post-merge CI
  `41193789027` succeeded at the exact base with Rust/web jobs passing. Remote
  develop matches; no existing QOS-0010 task, branch, PR or output-path collision.
- Working checkout was develop at the exact base, with only untracked
  `Market-Data/`; creating this branch changes no tracked tree contents. No owner
  data/prompts inspected, moved, stashed, cleaned or overwritten.
- Scope card pushed first as `529775f45b4bcf6e120ba2cd4ad139fcdeefaf47`, remotely
  verified before drafting the proposal/cases.
- Prepared the proposed close-only SMA model and SC01–SC13 invented case families.
  Bounded stdin-only Python integer/Fraction assertions PASS: 33 SC01–SC06
  numeric trace rows, SC07/SC12 boundary/suffix rows, period endpoints, one-unit
  near-M comparisons, symbolic u128 bounds, all nonempty prefixes and hand-derived
  SC13 cash/equity/capacity arithmetic. No reference script or generated data
  is committed.
- Local runtime checks: NOT RUN under this task's LOW-RESOURCE mandate. Arithmetic
  assertions are not strategy/kernel execution or error/mutation/parity tests.
- Immutability/allowlist/link/whitespace/secret checks PASS: exact nine Markdown
  paths; existing fenced API/formula/example blocks, six inherited backtest rules,
  earlier milestone evidence and QOS-0009's implementation work order unchanged;
  40 local links/anchors resolved at the proposal checkpoint. Execution rules/API/
  goldens and all executable/build/data inputs compare unchanged to the base.
- Proposal checkpoint `bbfb69fe35ea609cc9f88eace878eaa0908e6dd9` pushed and remote
  SHA verified. PR #10's description records the final documentation-handoff head
  and inspected required CI results. CI is unchanged-code regression evidence,
  not execution/proof of the SMA strategy or these Markdown expectations.
- No known preparation blocker. Stop at review; no strategy implementation,
  financial-semantic approval, merge, aggregate P2 acceptance or release.
