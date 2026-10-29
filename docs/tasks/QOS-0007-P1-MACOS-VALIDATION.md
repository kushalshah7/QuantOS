# QOS-0007 — P1 personal-Mac validation

- **Authorization:** Owner's 2026-10-29 instruction to pull the GitHub repository
  on their Mac and complete the next task. This executes the pending personal-
  machine checks in QOS-0006; the office-laptop LOW-RESOURCE restriction does
  not apply to this run. It does not grant aggregate P1 reviewer acceptance.
- **Phase:** P1 verification only.
- **Branch:** `feature/qos-0007-macos-validation`, from `origin/develop`
  `5b6a5005851b63224b4841a3d9dcdac4cf5cb7fa` (merged QOS-0006 / PR #6).
- **PR target:** `develop`.
- **Risk:** Low; local tool installation, synthetic execution and documentation.

## Goal and read boundary

Execute the outstanding personal-machine validation on macOS and deliver actual
commands/results for owner review. Read `docs/p1-acceptance.md`, `docs/cli-import.md`,
`PROGRESS.md`, the pinned toolchain/manifests and existing CI workflow. Existing
runtime/fixture/contract behavior is fixed; no new product phase is selected.

## Allowed edits and local setup

- This task card and its handoff.
- `docs/p1-macos-validation.md` — dated environment, commands, observations and limits.
- `docs/p1-acceptance.md` — current verification-status prose and evidence links only;
  preserve historical instructions, expected values, contracts and golden tables.
- `PROGRESS.md` — accurate merged QOS-0006 status and compact QOS-0007 handoff.
- `README.md` — current verification-status prose and link only.
- Install exact pinned Rust/Node/npm tools in the user's local tool directories;
  install locked dependencies/build outputs only in existing ignored directories.
  Synthetic temporary configs/reports/logs stay outside version control.

No runtime, test, fixture, dependency, lockfile, toolchain, workflow, financial
contract, governance or historical task-card changes. No real data or credentials.

## Execution and acceptance

1. Verify clean clone, remote `develop` SHA, merged PR #6 and unchanged executable
   inputs relative to the acceptance packet's implementation SHA
   `0568b5ec5b77e23e484baee012156557a81d8236`. Push this card before execution.
2. Install the repository-pinned Rust 1.99.0 (rustfmt, Clippy, WASM target),
   Node 24.19.0 and npm 11.17.0; report actual versions/host architecture.
3. At the exact implementation SHA in a clean detached checkout, execute the
   packet's valid, invalid-OHLC and explicit-timezone imports; compare actual
   exits, full candle values, issue keys, metadata and unchanged input hashes.
   Temporary configs are synthetic only. Return to the task branch afterward.
4. On the task branch run the README's full local quality suite once: Rust fmt,
   Clippy, workspace tests, WASM target check, no-argument CLI; deterministic npm
   install, lint, typecheck, tests and build. Record actual outputs and counts.
5. Review the written P1 criteria against observed results. Update only allowed
   current-status prose; retain owner acceptance PENDING and browser/WASM parity
   NOT RUN. No browser import binding exists to test parity.
6. Review exact changed paths/diff and sensitive patterns, commit/push verified
   checkpoints, open one PR to `develop`, inspect required CI at its final head,
   verify remote SHA and clean working tree, then stop for owner review.

## Stop conditions

Stop after two materially different diagnostic/fix attempts for an unavailable
tool or failed check, immediately on push failure, or on any reproducible runtime
defect or need to edit outside this allowlist. Report a focused follow-up; do not
change dependencies or source to force a pass. No merge, P1 owner acceptance,
P2, release, deployment, broker operation or real-data processing is authorized.

## Handoff

- Status: IN PROGRESS; validation results not yet recorded.
- Evidence / local checks / CI / PR: pending execution.
- Owner reviewer acceptance: PENDING.
