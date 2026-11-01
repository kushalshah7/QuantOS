# QOS-LINUX-002 — bounded interactive Codespaces validation

- **Authorization:** owner's explicit 2026-11-01 request, conditional on the
  account prerequisite below. One agent; no delegation.
- **Status:** BLOCKED before resource creation: current included compute/storage
  availability and controls preventing paid overage are not yet verified.
- **Repository:** `kushalshah7/QuantOS`.
- **Branch:** `feature/qos-linux-002-codespaces-validation`; PR -> `develop`.
- **Exact base:** `8c7140ad3a4f43690e699fd3ac6045af2bc2b371`.
- **Risk:** Medium: account quota, disposable cloud resource and platform evidence.

## Goal and accepted baseline

Validate one fresh disposable Codespace using the existing development container,
then publish a bounded evidence handoff. PR #12's Linux infrastructure and PR #11's
Rust-only SMA generator are accepted; aggregate P2 and full Linux-platform
acceptance remain pending. The base's
[post-merge CI 41317030493](https://github.com/kushalshah7/QuantOS/actions/runs/41317030493)
passed all four jobs, with 132 Rust tests including 25 SMA tests on each native
architecture. This is existing CI evidence, not an interactive Codespaces result.

Read only this card, relevant [Linux instructions](../linux.md), prior Linux card
sections, `.devcontainer/devcontainer.json` and `.devcontainer/verify.sh`, plus
applicable AGENTS/protocol/Git/CI rules. No broad repository or dataset inspection.

## Exact allowed edits

- This new card: `docs/tasks/QOS-LINUX-002-CODESPACES-VALIDATION.md`.
- New evidence: `docs/linux-codespaces-validation.md`.
- `docs/linux.md`.
- `PROGRESS.md`.

Existing-document edits are limited to current acceptance status, evidence links
and remaining gaps; preserve historical receipts. No source/tests, financial
contracts, dependencies, locks, toolchain, workflow, container or other file edits.
This card must be committed, pushed and remotely verified before execution.

## Mandatory pre-creation account gate

Before creating anything, verify **current remaining included compute and storage**
and **existing controls preventing paid overage**, through owner confirmation or
accessible account evidence. Account type and published plan allowances do not
establish remaining usage. Alerts alone do not establish a spending block.

The owner may confirm sufficient current included usage and the existing blocking
control, such as no valid payment method or a Codespaces hard-stop control that
prevents paid compute/storage overage. Do not request or record credentials,
payment details or private billing reports. Record the verification source/date
and gate outcome without publishing unnecessary account information.

Do not change billing, paid-usage settings, prebuilds, secrets or account access.
If the prerequisite cannot be verified, finish/push this card and return BLOCKED
with the missing confirmation. Do not create a resource or claim validation.

## Execution, only after the gate is satisfied

1. Recheck the exact accepted base, matching task branch and absence of conflicting
   task/output paths. Use the isolated task worktree. Preserve the owner's modified
   execution document, ignored Market-Data/, prompts and all other work; no stash,
   reset, cleanup, overwrite, private-data inspection or transfer.
2. Create exactly one fresh disposable Codespace from the pushed task branch,
   using the existing devcontainer and the smallest supported **two-core** machine.
   Verify machine availability; stop rather than substitute a larger/paid machine.
   Set a short idle timeout where creation options support it; target one bounded
   session of at most 30 minutes including startup and cleanup. No automatic retry
   creating a second resource. Record its task-owned identity privately for cleanup.
3. Confirm actual provider startup and post-create completion from session state
   and lifecycle output. Record actual checkout SHA, architecture, non-root UID
   and pinned tool versions. A recovery/fallback container or failed post-create
   command is not a successful validation of the configured container.
4. Run only the following commands inside that Codespace, capturing each exit code
   and concise output. Keep command/session timeouts bounded; stop on failure.

```sh
git rev-parse HEAD
uname -srm
id -u
bash .devcontainer/verify.sh
bash .devcontainer/verify.sh --smoke
CARGO_BUILD_JOBS=2 cargo test -p quantos-core --test sma --locked
```

Expected observations: Linux, non-root identity, repository-pinned versions,
`Environment verification: PASS`,
`Synthetic native CLI success/rejection smoke: PASS`, and successful execution of
all existing SMA tests (current baseline 25; report the actual output/count).
The checkout must match the pushed session checkpoint, not a later documentation
commit. Do not infer editor or browser-preview behavior from these terminal checks.

These remote commands are the sole runtime exception to LOW-RESOURCE mode.
No host installations/builds, local Docker/VM, full Codespace suites, frontend
setup, benchmarks, extra financial workflow or real/private data.

## Evidence, cleanup and final review gate

- Export concise session observations/commands/exits and the exact session SHA
  into `docs/linux-codespaces-validation.md` after actual execution. Keep this
  session receipt distinct from the final documentation head. Do not publish
  credentials, payment/account details, raw environment dumps or private logs.
- Verify executable, fixture, dependency, toolchain, workflow and container inputs
  remain unchanged from the exact base; only the four permitted Markdown paths
  may differ. Evidence must identify PASSED, FAILED and NOT RUN separately.
- Stop this task-created Codespace on success **or failure**. After evidence is
  safely exported and no unique work remains, delete only that disposable resource.
  Do not stop/delete any pre-existing owner Codespace. Record verified stop/delete
  results; a requested action alone is not cleanup proof.
- If runtime/configuration repairs are required, record the failure and proposed
  scope amendment; stop before editing any unapproved path or creating a replacement.
- Run bounded documentation/scope/link/whitespace checks, inspect sensitive-content
  boundaries, commit and immediately push coherent checkpoints. Stop on failed push.
  Open one PR to develop after the session/evidence work; inspect fresh final-head
  CI and actual logs. All four PR jobs (rust-quality, web-quality, linux-arm64,
  linux-devcontainer) must pass. Add no jobs and do not rerun successful CI for
  reassurance. CI regression evidence does not substitute for the session receipt.
- Stop for independent review. No merge, worker, new financial interface, release,
  deployment or phase acceptance. ARM64 development-container execution, Mac SMA
  runtime and browser financial parity remain unverified unless separately evidenced.

## Preparation receipt / current blocker

- Preflight PASS: remote develop matches the exact base; existing post-merge CI
  above passed all four jobs. No conflicting task, feature branch, PR, output path
  or worktree destination existed. A clean isolated worktree was created from the
  public tracked tree only; no setup scripts, installations or owner-data copies.
- Account evidence unavailable: the read-only current-month Codespaces usage-summary
  request returned HTTP 404, and GitHub CLI reported that its authentication lacks
  the required `user` scope. No scope refresh or account-setting change was made.
  The available in-app browser is signed out; no billing/account evidence was read.
- Missing owner confirmation: current remaining included compute/storage and the
  existing control preventing paid overage. A request for these facts is pending;
  the conditional resource-creation authorization has not been activated.
- Codespace created: **NO**. Provider startup, post-create verification, remote
  environment/CLI/SMA commands and resource cleanup: **NOT RUN / NOT APPLICABLE**.
  No host runtime checks, Docker builds, suites or datasets were executed.
- This is preparation only, not a successful platform-validation receipt. The
  pushed task-card SHA/branch and exact blocker are returned in the chat handoff.
