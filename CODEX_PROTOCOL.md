# QuantOS — Bounded Codex Execution Protocol (v3)

**Purpose:** Make the work reproducible across Mac/Windows and across model versions by specifying the task, inputs, acceptance tests, and allowed edits *before* asking Codex to write code. This is an engineering protocol, not a guarantee that different models produce identical code.

## A. Ownership and authority

- **Architect / owner (human + planning review):** prioritize issues, design interfaces, write/approve task cards and acceptance criteria, approve contract or architecture changes, merge PRs, authorize releases.
- **Codex implementer:** carry out exactly one approved task card, reason locally about implementation details, produce working code and tests, commit/push, open a PR, report evidence, and stop.
- **CI:** independent arbiter of lint, build, tests, reproducibility and security checks. A passing check cannot replace human review.
- Codex is an implementer with *bounded discretion*, not a literal unthinking compiler: it still must choose correct algorithms and handle implementation-level edge cases. It may not expand product scope on its own.

## B. Context routing — progressive disclosure

Typical new task instructions: `Implement QOS-XXXX using CODEX_PROTOCOL.md. Stop at its PR review gate.`

1. Get the named task card from `docs/tasks/` and confirm the expected remote branch + clean git state. The small root `AGENTS.md` is automatically discovered in supported Codex clients; do not redundantly paste it.
2. Use `MODULE_MAP.md` to locate the responsible module. Read *only* the card's `Read first` files, the referenced interface/contract and source/tests in the declared paths.
3. Read `MASTER_PLAN.md` for product-level decisions, `docs/architecture.md` for cross-module design, `docs/backtest-semantics.md` for trading semantics, `GIT_WORKFLOW.md` for exceptional Git changes and `DEVOPS_CICD.md` for CI/releases **only when relevant**. P0 bootstrap is the one broad-context exception.
4. Discover references with `rg -n '<symbol>' <allowed-directories>` / `rg --files <allowed-directories>`; avoid recursive dumps of repo contents, unnecessary full CI logs, generated folders, lockfiles or data.
5. If a needed dependency is outside the allowlist: inspect its *published interface* or relevant small excerpt. Escalate before editing it.

Document index (not mandatory reading):
- `MASTER_PLAN.md`: product scope, milestones, financial invariants.
- `MODULE_MAP.md`: owners, entry points, interface/test locations.
- `docs/data-contract.md`: candle normalization, source/timezone rules.
- `docs/backtest-semantics.md`: signals, fills, fees, risk, determinism.
- `docs/api-contracts.md`: Rust -> WASM -> UI API boundary.
- `GIT_WORKFLOW.md`: branches, PRs, multi-laptop sync, recovery.
- `DEVOPS_CICD.md`: checks, releases and hosting.
- `PROGRESS.md`: latest accepted milestone and blocker summaries; consult if the task refers to earlier work.

## C. Task-card contract

Every `docs/tasks/QOS-XXXX-*.md` specifies:

| Field | Required meaning |
| --- | --- |
| ID + status | Unique GitHub Issue or task ID; approved / draft / complete |
| Goal | One testable outcome, not a list of unrelated features |
| Branch / base | Exact feature branch and merge target |
| Read first | Small list of relevant files; avoid a repo-wide context dump |
| Allowed edits | Glob/path boundaries including tests/docs |
| Interfaces / invariants | Locked signatures, schemas, behavior, security constraints |
| Implementation steps | Ordered, bounded tasks; no optional scope creep |
| Verification | Exact tests/commands and expected observable behavior |
| Definition of Done | CI pass, Git remote SHA and PR, brief evidence |
| Stop / escalation | What requires owner approval rather than invention |

Change a task card's goal/allowlist/contracts only with owner approval. A missing required contract is a **BLOCKED** result, not permission to design arbitrary behavior.

## D. Implementation loop (bounded)

**SYNC -> READ -> IMPLEMENT -> TARGETED TEST -> REVIEW DIFF -> COMMIT/PUSH -> PR -> STOP**

1. `git fetch origin --prune` and confirm branch is up to date (`git pull --ff-only` when continuing). Do not modify protected branches. P0 follows its special repo bootstrap instructions.
2. Write minimal code implementing the prescribed interface and tests. Prefer incremental changes to wholesale rewrites.
3. Implement prescribed acceptance examples as deterministic assertions, including boundary/error cases. Prefer golden fixtures for financial calculations. Run formatting, compile, and **only the affected** tests locally first. On failures, inspect the failing messages/small output sections and fix them. CI runs the full Rust/web suites on PR; local full-suite runs only when a cross-module interface changes or the card demands it.
4. If still blocked after **two materially different diagnosis/fix attempts**, stop with the exact failing command, relevant stderr excerpt and next diagnostic action. Do not run indefinite speculative loops. For immediate obvious mistakes, simple correction/retest is allowed without escalating.
5. Review changed-file list and staged diff for unrelated changes, secrets, and large/private datasets. Commit a coherent checkpoint and immediately push. When the push fails, stop and report it.
6. Update the small handoff section (task card or `PROGRESS.md`) with pass/fail evidence, exact SHA and PR URL. Open PR to `develop`; do not merge. A task ending early reports **BLOCKED**, never **DONE**.
7. Final response must fit: `Status | Branch/PR/SHA | Changed paths | Tests PASSED/FAILED/NOT RUN | Blockers | Review required`. Do not include entire logs or lengthy narration.

The working tree should be clean at a completed checkpoint; do not commit `data/`, logs, caches or secrets even if that prevents a push.

## E. Tooling and token-efficiency policy

- **One task per Codex chat/thread** when feasible; don't drag unrelated historic conversation into the next task. Repository task cards + Git commit/PR history carry state between laptops and sessions.
- **No entire-codebase read** as a ritual. A narrow search for an unknown symbol is allowed if the task cannot otherwise proceed; scope it by module and capture only relevant results.
- **No repeated self-analysis essays**, multi-page final summaries, or unnecessary tool outputs. Favor compiler errors, assertions and PR diffs over verbal confidence.
- **Default single agent**. Subagents are optional for independent read-only risk reviews, separate subsystem investigations or truly disjoint tasks. They spend additional tokens; never launch a team just to perform ordinary implementation.
- When explicitly enabled: at most 1–2 subagents for a task; give each a precise scope, **read-only by default**, minimal files, and ask for a short finding list with file references. Main agent alone edits, commits and pushes unless owner approves isolated worktrees.
- **Model routing:** use a lower-cost/lower-effort model for fully specified, local edits. Use stronger reasoning for architecture trade-offs, concurrency, finance correctness, security, complex bugs and milestone review. A stronger model is not a replacement for tests.
- Periodically compare models on *the same task cards* using the same test suite and review checklist. Measure pass rate, regressions, CI retries, elapsed time and tokens **when the client exposes usage**. Never claim GPT-5.5 and GPT-6.1 will be equivalent without evidence.
- Avoid creating a custom agent framework, vector database, repo index service or many overlapping skills at P0. Add automation only when repeated friction is measured.

## F. Escalation / fail-closed cases

Stop with `BLOCKED` if the task is ambiguous about a money calculation, timeseries timezone, fill/cost semantics, licensed data, public/private exposure, authentication, new dependency or interface change. Also stop if push access, CI permissions or a required tool is unavailable. Give one concrete proposed resolution without editing outside authorization.

## G. What is *not* required at every turn

Do not read all architecture documents, run all test suites, reconfigure GitHub, create new ADRs, regenerate diagrams or invoke subagents for small work. These are event-triggered actions (architectural change, cross-cutting failure, release preparation, or specifically approved task).

### Source alignment

This design follows publicly documented principles of minimal `AGENTS.md`, progressive disclosure, targeted skills, bounded task goals, and selective subagent use. It is **our QuantOS-specific workflow**, not a claim of access to OpenAI's internal private engineering playbook.
