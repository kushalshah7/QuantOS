# QuantOS — working rules for coding agents

Use the approved task card in `docs/tasks/`. Treat its outcome, file boundary, interfaces, tests and stop conditions as the work order. **Do not autonomously select a new phase.** `CODEX_PROTOCOL.md` defines the execution sequence; use `MODULE_MAP.md` to locate relevant files.

- **Minimal context:** inspect only the task card, named contracts and affected files. Do not load the whole repository or re-read `MASTER_PLAN.md` on every turn. Read project-wide documents only for architectural, governance or release changes.
- **Scope:** never edit outside the task's allowed paths including tests and handoff files. If a change outside the allowlist is required, stop for scope amendment. If a public interface, financial semantics, security boundary or dependencies must change unexpectedly, stop and request an amended task card.
- **Correctness:** financial results must be derived, deterministic and verified; never introduce look-ahead bias. Never invent test results or pretend a tool ran.
- **Safety:** do not commit market datasets, Zerodha credentials, `.env` files, secrets or local backtest results. Follow `GIT_WORKFLOW.md` when touching Git and `DEVOPS_CICD.md` when touching CI/deploy.
- **Git:** one task = one remote feature branch and reviewable PR. Commit and push each verified checkpoint; never auto-merge, force-push or release. Stop if remote push fails.
- **Verification:** run affected tests locally; CI performs full checks. Report compact evidence (paths, commands/results, SHA, PR, blockers). **Default to one agent; subagents only if the task explicitly authorizes bounded, independent work.**

For P0, follow `CODEX_PHASE_0_PROMPT.md` and `docs/tasks/QOS-0001-P0.md`; initializing the repository legitimately requires reading the governance and architecture documents once.
