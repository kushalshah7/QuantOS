# QuantOS — Agile DevOps + CI/CD Operating Model (v2.0)

**Authoritative for CI/CD, release and pipeline edits; not mandatory context for every small coding task.** This policy complements `MASTER_PLAN.md` and `GIT_WORKFLOW.md`. If a tool is inaccessible, report the actual blocker rather than claiming success. Follow all safety and data policies in the other documents.

## 1. Delivery philosophy

- **Development process:** Agile, small-batch **Kanban** (not a rigid Waterfall or ceremonial two-week Scrum cycle for a solo founder).
- **Engineering practice:** DevOps with continuous integration, automated quality gates, reproducible builds, and continuous delivery.
- **Deployment policy:** Production only after human-reviewed release approval. Automatic deployment **after** an approved merge to `main` is permitted once the deployment pipeline is tested; never deploy an unreviewed feature to production.
- **Release model:** `main` = stable/public, `develop` = accepted integration, `feature/*`/`fix/*` = short-lived work. Optional `release/*` and `hotfix/*` as described in `GIT_WORKFLOW.md`. One approved task at a time; no autonomous phase advancement.
- **Platform constraint:** All persistent infrastructure must have a no-payment-required free-tier option. Public website should be static and Rust/WASM computations should run in user browsers. Do not require a paid always-on server.

## 2. Agile Kanban workflow

Backlog -> Ready -> In Progress -> Automated Checks -> Human Review -> Accepted -> Released.

For each card or phase: specify a user-facing outcome, architecture contract, acceptance tests, risk level, and exact scope. Limit work in progress to 1 substantial task. Keep tasks small enough to test and commit without accumulating unreviewed changes. Maintain `PROGRESS.md` and GitHub Issues/Projects if available; GitHub is the system of record. Record completed evidence instead of claiming percentage complete without proof.

`Definition of Done` for an accepted task: relevant tests passed; code and documentation are pushed; PR has green required checks; no secrets/licensed broker data are committed; changes are human-reviewed; no known critical defects. After merge, target environment is verified. Passing CI alone is **not** human approval.

## 3. Continuous Integration — implement in P0

Use GitHub Actions with least-privilege tokens (`permissions: contents: read` for test jobs), scoped triggers, caching where safe, and concurrency cancellation for superseded feature-branch runs.

**Run on:** pushes to `feature/**`, `fix/**`, `release/**`, `hotfix/**`, `develop`, `main`, plus pull requests targeting `develop` and `main` (avoid duplicate or unnecessary runs where practical).

**Required independent jobs with stable, unique status names:**

| CI job | Checks |
| --- | --- |
| `rust-quality` | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` |
| `web-quality` | deterministic install from lockfile; ESLint; TypeScript typecheck; frontend tests if configured; production Vite build |
| `wasm-build` | compile Rust WASM bindings and verify web packaging **once the WASM integration exists**; avoid requiring an empty or failing placeholder job |
| `python-quality` | lint and tests **only when Python code is introduced** |
| `security-audit` | dependency or secret checks if configured with supported free tools; never transmit private data in CI |

Use a synthetic, deterministic fixture for tests; include edge cases for timestamps and invalid OHLCV. No real Groww records, broker API secrets, login, or outbound financial API requests in CI. Keep full historical datasets off GitHub and out of action artifacts/logs. Do not execute untrusted public pull request code on a laptop or privileged/self-hosted runner.

Require `rust-quality` and `web-quality` status checks on PRs **after those names have run and are visible**; expand later when the corresponding jobs are real. Protect both `main` and `develop` against direct pushes, force-pushes and deletes using GitHub Free public-repository protection/rulesets where available. A sole maintainer should not require one approval that only another user can provide.

## 4. Continuous Delivery/Deployment — configure during P5, design for it in P0

A failed build/test must never update public production.

| Event | Result | Human gate? |
| --- | --- | --- |
| Push to `feature/*` + PR to `develop` | Automated CI + optional isolated Cloudflare Pages preview (synthetic data only) | Owner reviews PR |
| Accepted PR merged into `develop` | Integrated CI + optional staging preview, smoke testing | Owner confirms phase acceptance |
| Approved `release/*` PR merged into `main` | Automatic static frontend deployment to **production** from `main`, only if post-merge checks succeed; tag version and publish notes | **Yes, before merge/release** |
| Failed production smoke test | Block further releases; roll back through provider deployment rollback if supported, then reconcile source using a safe PR/revert; preserve Git history | Yes |

Use Cloudflare Pages with GitHub integration (free tier; check active quotas) or GitHub Pages as fallback. Production branch must explicitly be `main`; Cloudflare previews may build other branches. Avoid duplicate deployments through simultaneous Pages Git integration and a second GitHub Actions deploy job: choose one deployment mechanism and document it. A static-only frontend does not automatically host a Rust Axum API. Native local Rust CLI and private Zerodha bridge are separate artifacts, not public deployed services.

No public live Zerodha data redistribution; no credentials in frontend bundles, PR previews, GitHub Actions logs, or Cloudflare environment variables unless an approved server-side architecture and broker terms explicitly permit it. Public site defaults to synthetic or user-imported local data.

Deployment in P0 means **pipeline planning and successful test-builds**, not provisioning live hosting, adding credentials, or publishing an unfinished product. Establish actual site deploys and run browser smoke checks in P5.

## 5. Release governance and recovery

- Version tags: `v0.1.0`, `v0.2.0`, etc. Only tag built, tested, approved `main` commits. Do not move tags.
- Generate a changelog with highlights, changed contracts, migration guidance, known limitations, and tested targets (macOS, Windows/browser if verified).
- Keep PR, CI run, SHA, release URL and deployment URL (when it exists) in the progress/release report.
- Use `git revert` in a reviewed hotfix PR to correct source history; use provider rollback for immediate service restoration if supported. Do not force push, delete history, or assume a browser cache automatically rolls back.
- Pin Rust, Node and Python versions in reproducible toolchain or setup manifests; check in `Cargo.lock` and web lockfile. Confirm fresh-clone build on both macOS and Windows as part of a later cross-platform acceptance gate. Initial free GitHub-hosted Ubuntu builds are sufficient for P0.

## 6. Phase-0-specific acceptance

P0 can be accepted only if:
1. GitHub repo and `main`, `develop`, `feature/p0-foundation` branches actually exist remotely and are synchronized.
2. `AGENTS.md` links and enforces this policy and both core policies.
3. A workflow committed to `.github/workflows/` has **actually run** and CI status is reported accurately; failures are fixed or listed as blockers.
4. Branch protection is configured where permissions allow; unavailable permissions are reported, not invented.
5. CI makes no paid API calls and leaks no secrets/market data.
6. The phase ends with a pushed PR targeting `develop` and a user review gate; **do not auto-merge**.

P0 is complete without a public deployment. P5 owns preview/production environments and release automation, subject to owner approval.

## 7. Incremental verification and agent budget (v3)

- **Locally:** run the narrowest targeted tests during implementation. **In GitHub Actions:** run the required Rust/web CI suites on PR to `develop` or `main`. Broader local test suites are reserved for changed shared contracts or explicitly high-risk work.
- Tasks must arrive as scoped `docs/tasks/QOS-XXXX-*.md` tickets with locked interfaces, pass/fail acceptance criteria and one PR review gate. Unspecified features are not implicitly authorized.
- Default to one Codex coding agent. Additional read-only reviewers may be explicitly requested for independent security/financial-correctness investigation. Subagents consume extra tokens and do not substitute for CI.
- Keep CI outputs concise in handoffs: URLs, failing job names and a minimal relevant error excerpt, not full logs. Do not re-run green jobs just to generate reassuring transcripts.
- No automatic merges, speculative dependency upgrades or out-of-phase deployments.
