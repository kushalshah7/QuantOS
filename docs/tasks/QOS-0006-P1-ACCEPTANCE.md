# QOS-0006 — P1 acceptance evidence and current-status documentation

- **Status:** APPROVED by the owner's QOS-0006 execution instruction.
- **Phase:** P1; aggregate phase acceptance remains pending.
- **Branch:** `feature/qos-0006-p1-acceptance`, from exact `origin/develop`
  `0568b5ec5b77e23e484baee012156557a81d8236`.
- **PR target:** `develop`
- **Risk:** low; documentation and evidence only.

## Goal

Deliver one source-backed P1 acceptance packet and accurate current-status prose,
without changing runtime behavior or granting phase acceptance.

## Read first (maximum 2–5 relevant files)

- `MODULE_MAP.md` for the smallest contract/test routes.
- `PROGRESS.md` for accepted subtask history and contradictory status wording.
- `MASTER_PLAN.md`, section 1 item 9 and section 6, for the phase gate only.
- `docs/cli-import.md` for exact future manual commands and report observations.
- `docs/api-contracts.md` for implemented surfaces and verification limits.

Inspect only affected status paragraphs, linked contracts/test interfaces and
small relevant CI output sections needed to support the acceptance packet.

## Allowed edits

- `docs/tasks/QOS-0006-P1-ACCEPTANCE.md` — this card and its handoff.
- `docs/p1-acceptance.md` — new compact packet, preferably 100–150 lines.
- `PROGRESS.md` — accepted QOS-0005 evidence, qualification/removal of obsolete
  review status and compact QOS-0006 handoff.
- `README.md` — current stage/capability summary and packet link only.
- `docs/data-contract.md` — implementation-status introduction and
  verification-limit paragraphs only.
- `fixtures/synthetic/normalization-cases.md` — introductory implementation
  status and closing verification/proof-limit prose only.
- `fixtures/synthetic/README.md` — implementation/test-status prose and links only.
- `docs/api-contracts.md` — current-status/verification prose and packet link only.

## Locked contract / invariants

- All schemas, signatures, invariants, lexical rules, issue catalogs, arithmetic,
  golden inputs/outputs/tables and fixture/config bytes are immutable.
- Existing scope is native CSV parsing, explicit normalization and CLI JSON
  reports: one symbol/exchange, 5m bar OPENs, rupees to paise, explicit time,
  declared provenance and explicit-grid-only gaps. Do not reinterpret contracts.
- Markdown goldens are hand-written specifications; executable Rust assertions
  provide runtime evidence. CI does not execute Markdown or prove its arithmetic.
- Distinguish Ubuntu CI, local runtime, personal-machine hands-on, macOS and
  WASM target compilation from browser/native-WASM runtime parity.
- P1 requires automated tests, hands-on validation and written reviewer acceptance.
  The packet's decision is READY FOR OWNER REVIEW; hands-on validation is NOT RUN.
- No code, tests, data/config fixtures, dependencies, lockfiles, toolchain,
  workflow, governance/master-plan or other task-card edits; no real datasets.

## Exact implementation

1. Fetch origin; independently verify PR #5's accepted head
   `a393e59aaae05ca9f53c1b7fd6522269e90f0022`, squash merge and exact develop base.
   Stop before edits if develop moved; report intervening changes.
2. Preserve all present untracked/ignored owner files in the original checkout;
   create a clean isolated worktree. Commit/push this approved card first.
3. Recheck merged PR metadata and implementation-base CI run `38727027369`,
   including actual relevant Rust job output. Link immutable implementation
   source/test files at the base SHA, not mutable branch tips.
4. Write a compact criterion-to-contract/test/CI/result table, subtask merge
   evidence, verification statuses and remaining acceptance requirements.
5. Supply a written reviewer checklist and exact future personal-machine synthetic
   valid/invalid import, timezone and provenance checks; mark unperformed steps
   NOT RUN. Execute none of those commands on this office laptop.
6. Correct only allowed stale present-tense status prose and add packet links.
   Preserve historical evidence and distinguish later-phase unimplemented work.
7. Review every hunk, push coherent checkpoints, open one PR to develop and
   inspect existing required CI on its exact final head. Stop at owner review.

## Verification and proof

- `git diff --check`: no whitespace errors; review staged and full task diffs.
- `git diff --name-only 0568b5ec5b77e23e484baee012156557a81d8236...HEAD`:
  only the eight allowed paths. Inspect every hunk for prose-only boundaries;
  immutable tables/code blocks/rules and all executable files stay unchanged.
- GitHub implementation-base CI: exact SHA, successful `rust-quality` and
  `web-quality`, actual test-group output and WASM compilation evidence.
- GitHub final-head CI: existing required `rust-quality`/`web-quality` succeed;
  this is regression evidence, not validation of documentation arithmetic,
  manual usability or untested behavior.
- LOW-RESOURCE: text/Git operations only. Local compilation, tests, builds, CLI
  execution, installation, dependency resolution, servers, browser automation,
  benchmarks and dataset processing NOT RUN. macOS/manual checks NOT RUN.
- User-observable delivery: linked packet explicitly READY FOR OWNER REVIEW,
  with remaining hands-on/reviewer gate and no aggregate P1 acceptance claim.

## Review/stop gate

- Commit/push each coherent checkpoint; stop immediately on push failure.
- If develop moved, scope/interfaces require amendment or a reproducible
  functional issue appears, report it and a focused proposed follow-up; do not fix
  code or change a contract. Pending hands-on validation alone does not block
  completion of this documentation task.
- No merge, release, deployment, P1 acceptance/waiver or P2 work.

## Handoff (Codex fills after work)

- Status: NOT STARTED; approved card checkpoint precedes documentation edits.
- Paths changed: this card only so far.
- Tests/results: local runtime checks NOT RUN; evidence audit pending.
- Remote commit SHA: recorded by the pushed card checkpoint and PR history.
- PR link: pending.
- Risks / next decision: personal-machine hands-on validation and written owner
  acceptance remain pending; this task does not waive either gate.
