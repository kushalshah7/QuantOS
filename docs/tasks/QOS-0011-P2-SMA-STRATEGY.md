# QOS-0011 — Rust-only SMA crossover signal generator

- **Status:** APPROVED for this bounded implementation by the owner's pasted
  2026-11-01 request. Accepted SD01–SD10 / SC01–SC13 remain authoritative;
  aggregate P2 acceptance is pending.
- **Branch:** `feature/qos-0011-sma-strategy`; PR -> `develop`.
- **Exact base:** `fcd55862fac170c3326da5850b2b0e9c031c1274`, accepted PR #10
  protected squash merge of reviewed head `43a4577f25c41dcdbb60d9b6170355f683fef6ee`.
  The merge message explicitly approves SD01–SD10 and SC01–SC13 as a contract;
  this separate owner request authorizes its implementation.
- **Risk:** High: exact indicator arithmetic, chronology and a new public API.
- **Agent mode:** one agent; no subagents or delegation.

## Goal and read boundary

Implement `quantos_core::sma::generate_sma_v1` under the exact
[API lock](../strategy-sma-api-v1.md), pushed with this card before code.
Read only the approved SMA contract/cases, execution API, module router and
existing public candle/provenance/completion/decision types. No broad repository
exploration or private-data inspection.

## Allowed edits

Within `crates/quantos-core/`:

- `src/sma.rs`, `src/sma/**`.
- `tests/sma.rs`, `tests/sma_cases/**`.
- `src/lib.rs`: module export/comment only; preserve existing exports/status.

Documentation:

- This card and `docs/strategy-sma-api-v1.md`.
- `docs/strategy-sma-v1-draft.md`, `fixtures/synthetic/sma-crossover-cases.md`.
- `docs/tasks/QOS-0010-P2-SMA-CONTRACT.md`.
- `docs/backtest-semantics.md`, `docs/api-contracts.md`.
- `MODULE_MAP.md`, `README.md`, `PROGRESS.md`.

Existing-document edits are acceptance/current-capability/routing prose only.
Preserve approved financial formulas/decision rules, numerical examples, public
APIs and historical evidence. No existing P1/execution source/tests, dependencies,
manifests, lockfiles, toolchains, CI, CSV/JSON fixtures, CLI, WASM or UI changes.

## Locked interface and implementation invariants

- Exact public signature, fields, derives, serialization/error order and typed
  invalid-value limits are fixed in `docs/strategy-sma-api-v1.md` before code.
- Reuse P1 data/provenance and execution completion/decision types unchanged.
  No Deserialize, JSON parser, new dependency or public test-only controls.
- Validate the entire input first, with approved first-error order. No partial
  decisions/report on error; no coercion, default, repair or input mutation.
- Checked u128 rolling sums with subtract-before-add updates and exact checked
  cross-products. Linear processing in candle count; constant rolling state
  apart from the output, using the input slice for outgoing CLOSE values.
- Preserve warm-up/first-availability Hold, equality crossings, final signals,
  state-independent decisions, determinism and valid-prefix invariance.
- Strategy source cannot call execution or perform IO/clock/environment access.
  SMA -> existing kernel composition belongs only in this task's tests.

## Verification and execution sequence

1. Verify PR #10 accepted head/merge/approval and successful exact-merge CI;
   expected remote base and no existing QOS-0011 task/branch/PR before creating.
   Preserve all owner work. Push this card and API lock before implementation.
2. Add only the locked API, validation and exact rolling generator. Add independent
   literal SC01–SC13 assertions where constructible, and an independently summed
   direct-window reference on bounded small synthetic inputs.
3. Cover period endpoints/short inputs, near-M/maximum-period arithmetic,
   validation precedence/late atomic errors, output/input preservation, repeated
   serialization, valid prefixes and non-CLOSE mutations. Actually compose SC13
   with the unchanged kernel, including rejection/ledger values and SC05 expiry.
   Do not claim runtime rejection of values Rust types prevent constructing.
4. LOW-RESOURCE override: **no local installations, compilation, Cargo/npm suites,
   builds, product CLI, benchmarks or datasets**. Existing-tool formatting/text
   checks are allowed. CI must execute the new tests and full regression checks:
   existing fmt/Clippy/Rust tests/WASM compile and web jobs. Inspect actual logs.
5. Review exact scope, immutable contracts, sensitive content and diff; commit
   and immediately push coherent checkpoints, open one PR to develop, verify
   final remote head/required CI, report actual counts and stop at PR review.

Stop on failed push or an unexpected financial/interface/scope requirement;
never silently amend approved semantics. After two materially different failed
diagnostic attempts on a defect, report the precise blocker. No merge, new
strategy, phase acceptance, release, deployment or next task. WASM compilation
does not prove browser financial parity. Local runtime checks remain NOT RUN.

## Handoff

- Status: IMPLEMENTED, AWAITING OWNER REVIEW in
  [PR #11](https://github.com/kushalshah7/QuantOS/pull/11) to develop. No merge or
  aggregate P2 acceptance is authorized.
- Preflight PASS: accepted PR #10 head/merge/approval match above; exact-merge CI
  `41197152381` succeeded at the base with both Rust/web jobs. Remote develop
  matches; no QOS-0011 task/branch/PR or new-output-path collision existed.
- Owner checkout contains a tracked modification to
  `docs/backtest-execution-v1-draft.md` and untracked Market-Data/. Neither was
  read for private contents, copied or modified. Implementation uses a separate
  clean Git worktree at the accepted base, with no setup/install scripts.
- Scope/API checkpoint `11c465f44cb692e06912f4688f8a0cbc67fb6a9f` was pushed
  and remotely verified before source edits. Implementation checkpoint
  `bcd6bfa9cb18d0fb6084da06fa5b580e6fc52332` was committed/pushed immediately.
- Implementation CI PASS: [PR run 41247455570](https://github.com/kushalshah7/QuantOS/actions/runs/41247455570)
  and [push run 41247440623](https://github.com/kushalshah7/QuantOS/actions/runs/41247440623)
  both succeeded at that exact source SHA. PR job steps/logs were inspected:
  fmt, Clippy with warnings denied, 132 Rust tests (107 existing + 25 SMA),
  WASM target compilation; web lint/typecheck, one test and build passed.
- The 25 SMA tests assert applicable SC01–SC13 vectors/boundaries/errors,
  direct-window reference comparisons on 729 six-bar patterns and all 15 period
  pairs, near-M/maximum-period arithmetic, deterministic serialization/input
  preservation, prefixes/non-CLOSE mutations, actual SC13 ledger/rejection and
  SC05 terminal expiry. No public overflow injection or decoder is present.
- Local PASS: existing rustfmt check on only the new SMA source/test roots;
  allowlist, immutable tables/fences/inherited rules/history, links, whitespace
  and staged sensitive-content inspection. No owner files entered the task tree.
- Final documentation-head SHA, remote equality and required CI/log results are
  recorded in PR #11's description after the final push, avoiding a self-referential
  evidence commit. The source checkpoint above identifies the implemented code.
- Local runtime checks: NOT RUN under the explicit LOW-RESOURCE override.
- CI runs the executable synthetic assertions; QOS-0010's earlier Python checks
  were arithmetic-only. Historical Mac results cover the earlier kernel/P1, not
  this SMA implementation. Browser/native-WASM financial runtime parity remains
  unverified; WASM compilation alone does not establish it. No known implementation
  blocker at handoff; owner review remains required.

## Authorized accepted-Linux integration — 2026-11-01

The owner explicitly authorized resuming QOS-0011 on its existing branch and
PR #11 for integration and fresh CI verification only. Starting feature head:
`e7a7c542115ad1878de50c4962c52ed9476a9c6a`. The original implementation base
`fcd55862fac170c3326da5850b2b0e9c031c1274`, work order, source decisions and
historical evidence above remain unchanged.

- Integrate exactly accepted develop `81515e66449d2fde91d33b366c0435b696da6047`
  using a normal merge commit on `feature/qos-0011-sma-strategy`. Preflight
  verified the open PR/starting head/target, remote develop, and all four passing
  jobs in exact post-merge CI
  [41298982925](https://github.com/kushalshah7/QuantOS/actions/runs/41298982925).
- This authorizes inheriting PR #12's accepted infrastructure paths despite the
  original SMA prohibition on authoring CI changes. Preserve that infrastructure
  exactly as accepted; preserve all SMA/runtime source/tests, APIs and financial
  contracts exactly as at the starting feature head. MODULE_MAP.md must retain
  both the SMA entry and the inherited Linux entry through the normal merge.
- This appended integration/evidence section is the only additional manual edit.
  No rebase, history rewrite, force push, additional PR, source/dependency fix or
  infrastructure redesign. Stop on unexpected conflicts or required changes.
- Reuse the clean isolated task worktree. Preserve the owner's execution-document
  edit, Market-Data/, prompts and other tracked/untracked/ignored work; do not
  inspect private datasets, stash, clean, overwrite or reset owner work.
- Verify exact tree boundaries, shell/embedded-Python syntax and whitespace;
  commit and immediately push coherent checkpoints. Local installations,
  compilation, Docker builds, suites, product CLI, benchmarks and datasets remain
  NOT RUN under LOW-RESOURCE mode.
- Inspect fresh final-head PR CI/logs: rust-quality, web-quality, linux-arm64 and
  linux-devcontainer must all pass. Confirm actual native Rust/SMA counts on both
  architectures (expected baseline 132 total / 25 SMA, never a hardcoded test
  assertion), Docker-context regression, image/CLI smoke and WASM compilation.
- The integration SHA, integrated develop SHA, actual fresh CI receipts and
  verification limits are recorded in PR #11's description; earlier receipts
  above remain historical and cannot substitute for the integrated-head run.
- Stop for independent source review. No merge into develop, SMA/aggregate P2
  acceptance, Codespaces creation, billing change, worker, release, deployment
  or new task. Interactive Codespaces startup, ARM64 development-container
  execution, Mac runtime and browser financial parity remain unverified.
