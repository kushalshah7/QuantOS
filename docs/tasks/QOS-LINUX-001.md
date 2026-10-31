# QOS-LINUX-001 — reproducible Linux development and native CI

- Status: APPROVED by the owner's Linux Integration Directive on 2026-11-01.
- Branch: `feature/qos-linux-001`; PR target: `develop`.
- Exact base: `fcd55862fac170c3326da5850b2b0e9c031c1274` (accepted PR #10).
- One agent. Infrastructure only; no financial/API or dependency changes.
- PR #11 remains separately under review at `e7a7c542115ad1878de50c4962c52ed9476a9c6a`.
  Do not merge it or import its changes into this branch.

## Goal and authority

Extend the existing local-first architecture with a reproducible Codespaces
development container and actual native Linux ARM64 test execution. Retain the
existing Linux x86-64 Rust/web quality jobs and WASM compile check. The owner's
directive authorizes implementation and a reviewable PR, not a merge, release,
paid resource, hosted backend or worker.

Read AGENTS.md, CODEX_PROTOCOL.md, GIT_WORKFLOW.md, DEVOPS_CICD.md, the relevant
architecture/master-plan sections, current workflow/toolchain manifests and the
module router. Then use this card and docs/linux.md; no whole-repo rereads.

## Exact allowed paths

- `.devcontainer/devcontainer.json`
- `.devcontainer/Dockerfile`
- `.devcontainer/install-toolchains.sh`
- `.devcontainer/verify.sh`
- `.devcontainer/downloads.sha256`
- `.dockerignore`
- `.github/workflows/quality.yml`
- `.gitignore`
- `docs/linux.md`
- `docs/decisions/0003-linux-development-and-execution.md`
- This card: `docs/tasks/QOS-LINUX-001.md`
- `DEVOPS_CICD.md` (append Linux policy only)
- `MODULE_MAP.md` (add Linux infrastructure routing only)

No runtime source/tests, financial contracts, package manifests/lockfiles,
existing toolchain pins, AGENTS/protocol rewrites, PR #11 files, owner datasets,
credentials or local owner work. New infrastructure documentation records Linux
as a native development/execution target; it does not add a mandatory backend.

## Implementation contract

1. Push this card before implementation. Use an isolated public-repository
   checkout; never inspect/copy the owner's Market-Data/ or local edits.
2. Minimal single Dockerfile, no Compose, services, Docker socket, privileged
   mode, paid registry, image publishing or automatic prebuilds. Use a verified
   immutable multi-architecture official Python base image containing C/C++
   build tools; its digest pins those tools. Install exact Rust/Node/npm versions
   from existing manifests, with checksum-pinned Rustup/Node downloads.
   No application dependency upgrades. Non-root development user.
3. Codespaces uses two cores by default, no automatic build/test/dev server,
   and a lightweight post-create version verification. Do not create a Codespace
   or change billing: available quota/account spending controls are unverified.
   Document free-quota checks, idle shutdown and explicit stop/delete handling.
4. Docker build context is allowlisted to infrastructure and toolchain manifests.
   Never COPY the repository/data wholesale. Add Market-Data/ to gitignore;
   preserve synthetic fixtures. No data/artifact uploads.
5. Pin existing rust-quality/web-quality runners to Ubuntu 24.04, retaining their
   required names and existing checks. Add native ubuntu-24.04-arm Rust tests,
   with host/architecture assertion and no QEMU/cross-compilation substitution.
   Add one x86 development-container build/version/CLI smoke job. The two new
   jobs run on PRs and protected-branch pushes, avoiding duplicate feature-push
   work. No duplicate formatting/frontend suites, new third-party actions,
   secrets, branch-protection changes, caches/artifact publishing or deployment.
6. Record scope, platform evidence and limitations in docs/linux.md and this
   card. The current base has 107 Rust tests; PR #11's 132 tests are separate
   evidence until merged. Never hardcode suite counts as an acceptance assertion.

## Checks and expected results

Local low-resource checks: shell syntax, strict JSON/schema-shape assertions,
exact allowlist/diff/whitespace, checksum-file formatting, immutable source/build
inputs and private-context exclusion. No local Docker/compilation/installations.

Existing CI: cargo fmt --all -- --check; cargo clippy --workspace --all-targets
--locked -- -D warnings; cargo test --workspace --locked; cargo check -p
quantos-wasm --target wasm32-unknown-unknown --locked; locked npm install,
lint/typecheck/test/build. All must exit 0.

Native ARM64 CI: uname reports Linux/aarch64 and rustc host
aarch64-unknown-linux-gnu; cargo test --workspace --locked exits 0 on that
runner. Logs must show executed tests, not merely compiled binaries.

Container CI: docker build of the exact Codespaces Dockerfile succeeds; non-root
container verifies pinned versions, C/C++/Python availability, then runs a bounded
native CLI synthetic import and checks exact success/candles. A second malformed
input checks the expected rejection/exit. This is container execution evidence,
not proof of a Codespaces account session.

## Completion and stop

One feature PR with remotely verified head; unchanged financial/source/dependency
inputs; all four jobs green on final-head PR CI; concise logs/results reviewed.
Codespaces interactive creation and macOS/browser financial parity are reported
separately as NOT RUN. A manual fresh Codespace check is required before declaring
Codespaces itself accepted; CI builds the same container without consuming its quota.

Commit/push each verified checkpoint. Stop on failed push, needed out-of-scope
changes, unavailable free runner, or two materially different failed fixes.
Do not switch to paid runners/services or waive failing checks. No merge,
worker/systemd/cgroups, real-data processing, new strategy, phase acceptance,
release or deployment. Return PR/SHA, checks and remaining review gates.
