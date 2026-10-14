# QuantOS — Git & GitHub Operating Policy (v2.0)

**Authoritative when working on Git; ordinary code edits follow the short rules in `AGENTS.md` without re-reading this entire document each time.** Follow `DEVOPS_CICD.md` for the Agile Kanban and CI/CD delivery pipeline. GitHub is the canonical source of truth. Local directories are disposable working copies. The project must remain buildable from a fresh clone without private market data or secrets.

## 1. Repository and ownership

- Repository name: `QuantOS` (GitHub URLs are case-insensitive; local project directory can be `quantos`).
- Prefer a **public GitHub repository** for open-source portfolio visibility and GitHub Free branch-protection support; absolutely never publish Groww/NSE datasets, broker credentials, private configuration, raw live data logs, proprietary files, or user-generated results without authorization. If privacy cannot be guaranteed, stop before creating a public repo and ask the owner to make the visibility decision.
- Default branch: **`main`**. Integration branch: **`develop`**.
- Create the remote with the authenticated user's GitHub account (never invent a username or owner). Prefer existing repo if present; **do not create duplicate repositories**.
- Set `git config pull.ff only` for this repository. Avoid modifying global Git settings unless the owner explicitly requests it.
- No code lives exclusively on a laptop at an agent handoff. Push completed work before stopping.

## 2. Branch model

| Branch | Purpose | Source | Integration target |
|---|---|---|---|
| `main` | Tested, releasable versions only | initial bootstrap | tagged releases |
| `develop` | Verified integration of accepted phases | `main` | next `release/*` |
| `feature/p0-foundation`, `feature/p1-data` ... | Isolated implementation of one approved scope | current `develop` | PR -> `develop` |
| `fix/<issue>` | Ordinary bug fixes | `develop` | PR -> `develop` |
| `release/vX.Y.Z` | Stabilize a specific release candidate; no new features | `develop` | PR -> `main`, then synchronize changes back to `develop` |
| `hotfix/<issue>` | Critical fix to released version | `main` | PR -> `main`, then synchronize fix back to `develop` |

**Default:** create one short-lived feature branch per approved phase or coherent sub-feature. Do not create lots of idle branches. Delete short-lived remote branches after merge when safe; keep immutable version tags.

### Phase 0 bootstrap exception

For an *empty* repository only:
1. Build a tiny documentation-only seed (`README.md`, `MASTER_PLAN.md`, `GIT_WORKFLOW.md`, `AGENTS.md`, safe `.gitignore`) locally; initialize `main`, make an initial commit, push to GitHub.
2. Create and push `develop` from `main`.
3. Create `feature/p0-foundation` from `develop`; **all actual scaffolding and subsequent edits happen here**, not directly on `main` or `develop`.
4. Add CI and let it run at least once, then configure branch protection. Never require nonexistent CI checks that would deadlock merges.
5. Open a PR `feature/p0-foundation` -> `develop`, preferably draft until tests are green. **Stop before merging and request owner review.**

After bootstrap, **no direct pushes to `main` or `develop`**; use pull requests.

## 3. Commit -> push -> PR: required loop

For every small, coherent code/documentation change:
1. Confirm correct repository, working branch, and remote. Refuse edits on `main` or `develop` after bootstrap.
2. Implement change; run directly relevant tests.
3. Inspect `git status`, `git diff`, and the exact list of files to stage. Explicitly exclude secrets, credentials, data, logs, build output, and sensitive samples.
4. Stage only safe paths; inspect `git diff --cached --stat` and `git diff --cached`. Use descriptive commits, e.g. `feat(data): validate UTC timestamps`.
5. **Immediately push** the current short-lived branch to `origin` (`git push -u origin <branch>` first time, otherwise `git push`). Do not leave multiple unpublished commits waiting until session end.
6. Confirm push success and remote SHA. If push/auth/network fails, **stop and report**—don't proceed with unbacked local work.
7. Update/create a draft PR early, then mark ready for review when checks pass. Every meaningful delivery or phase ends with a PR URL, pushed commit SHA, test evidence, and a clean working tree.

**Interpretation of “every change should be on GitHub”:** Every completed *logical checkpoint* is committed and pushed; unsaved editor keystrokes and untested partial lines cannot be meaningfully tracked by Git. Agents must make checkpoints small and frequent. Never auto-commit secrets just to meet a push cadence. If a phase is interrupted with unfinished *safe* code, commit/push it as clearly marked `wip:` on the feature branch, with a note that tests may fail; never merge it.

### Forbidden unless owner explicitly approves

- `git push --force`, `--force-with-lease`, direct push to protected branches, `git reset --hard`, destructive cleans/deletes, unreviewed history rewrites, silently dropping commits, and disabling CI/protection to force a merge.
- Pushing API keys, OAuth tokens, cookies, Zerodha credentials, private market datasets or huge generated files.
- Claiming a push, PR, protection rule or test succeeded without checking the tool response.

## 4. Pull requests and branch protection (free tier)

Use GitHub's branch protection or repository rulesets for **both `main` and `develop`** once CI check names exist. Set:
- Require a PR to merge; require CI checks to pass; block force pushes and deletion; require resolved conversations if available.
- Require linear history if compatible with selected merge policy (prefer squash merges for phase feature PRs).
- Apply rules to administrators too **if supported and feasible**; ensure sole maintainer can still merge reviewed PRs.
- **Do not set mandatory approval count to 1** on a solo-maintainer repo unless a second eligible reviewer exists; self-approval cannot satisfy that requirement.
- Required checks: stable, unique names for Rust quality/test and Web lint/typecheck/build. Enable checks only after they have appeared on GitHub; do not lock the repository with missing checks.
- Avoid CI jobs that call paid services; don't expose repository secrets to untrusted PR code.

**Plan limitation:** GitHub Free's native branch protection is available for *public* repos; private repo protections may require a paid plan. If repo is private, document precisely which settings are not enforceable instead of pretending. Still follow workflow voluntarily.

## 5. CI, safety and auditability

**CI job names, conditions, rollout timing and deployment gates are defined in `DEVOPS_CICD.md`.** CI must run on PRs; release deployment is configured in P5, not P0. A merge to `develop` is never a production release. Approved merge to `main` is the sole production deployment trigger. Do not treat `CI green` as permission for Codex to auto-merge or auto-release.


- GitHub Actions should trigger on feature/fix/release branch pushes and PRs to `develop`/`main` (where supported), with minimal `contents: read` permissions.
- Initially run: Rust format, Clippy with warnings denied, Rust tests; web lint, TypeScript check, tests if configured, and production build. Pin or reasonably version dependencies; commit Rust `Cargo.lock` if appropriate for app workspace, web lockfile, and GitHub workflow files.
- At minimum, `.gitignore` covers: `data/`, `target/`, `node_modules/`, `dist/`, `.env*` except sample templates, token files, cache files, Python virtual environments, downloaded `.parquet`/`.db`/`.sqlite`, local broker logs. `fixtures/synthetic/` is permitted to hold *verified synthetic* samples.
- Add a secret / sensitive-data inspection before staging and before PR. Never blindly `git add .` where private data may be present.
- `PROGRESS.md` and `docs/decisions/` capture implementation evidence and key architecture decisions. PR description links to the relevant phase gate.
- Release only from green `main`; tag with annotated semantic version `vMAJOR.MINOR.PATCH` (or `v0.X.Y-alpha.N` for pre-release), and publish a changelog/release notes. Tags are never moved or reused.

## 6. Multi-laptop operating procedures

### First use on another laptop

```bash
gh auth status                         # or confirm SSH/HTTPS authentication
git clone <ACTUAL_GITHUB_REPO_URL> quantos
cd quantos
git config pull.ff only
git fetch origin --prune
git switch develop
git pull --ff-only origin develop
```

### Start an approved new task

```bash
git status --short                      # must be clean
git fetch origin --prune
git switch develop
git pull --ff-only origin develop
git switch -c feature/pN-short-description
git push -u origin feature/pN-short-description
```

### Continue an already pushed feature from a different laptop

```bash
git fetch origin --prune
git switch --track origin/feature/pN-short-description  # first time only
git pull --ff-only
git status -sb
```

If tracking branch already exists locally, `git switch feature/pN-short-description` then `git pull --ff-only`. Never work on the *same feature branch concurrently on two laptops*. Finish and push on laptop A, then fetch/pull on laptop B. On return to A, fetch/pull before editing.

### Restore a known good version without erasing history

```bash
git fetch --tags origin
git switch --detach v0.1.0                 # read-only inspection of a released tag
# For a safe production rollback, use a new hotfix branch and git revert,
# then open a PR. Never force-move main or rewrite published tags.
```

**Conflict policy:** If a branch diverges, stop, fetch, examine `git log --oneline --left-right --graph HEAD...@{u}` and resolve consciously; never run automated force-push/reset. If uncommitted changes exist on laptop B, commit/push them on their own branch or handle with explicit user direction before switching.

## 7. Definition of phase completion (mandatory)

Codex's end-of-session report must include:
- Active branch and GitHub repository URL.
- Commit SHA(s) that exist remotely; exact `git push` outcome.
- Pull-request URL and target (`develop` except releases/hotfixes).
- CI run/check status and locally executed commands (pass/fail/not run).
- `git status --porcelain` output; expected clean state.
- Files changed, unresolved issues, data/secrets inspection, and next recommended phase.
- **STOP** at the phase review gate. Do not merge PR or start next phase without owner approval.

## 8. If GitHub integration is unavailable

First try authenticated `gh` and `git` in the Codex environment. A GitHub plugin connected to ChatGPT **does not automatically guarantee** the separate Codex terminal has that plugin or an authenticated shell. Check actual access. If repo creation or pushes are blocked, finish *only safe local preparation*, describe the precise blocker and give the smallest manual action; never falsely mark a remote sync as completed.

## 9. Approved task cards and token-efficient Git operations (v3)

- One approved `docs/tasks/QOS-XXXX-*.md` per branch/task; the card declares allowed paths, acceptance tests, branch name and PR target. Link it from the PR.
- Avoid reading entire Git history. Use `git status -sb`, `git diff --stat`, `git diff -- <scoped paths>` and short recent logs only when necessary.
- Codex may make multiple small commits per task, each pushed immediately to its feature branch. Do not create a PR for every micro-commit; one PR represents one reviewed task.
- Repo task cards, PRs, CI and commits are the persistent cross-laptop state; do not depend on chat-session history. Keep handoffs short.
