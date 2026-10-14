---
name: quantos-implement
description: Implement one owner-approved QuantOS task card with limited file edits and verified GitHub PR handoff. Use only when asked to execute a QOS task ID.
---

1. Use Codex-provided `AGENTS.md` (read it only if not already in context), `CODEX_PROTOCOL.md` and the named `docs/tasks/QOS-*.md` task card; use `MODULE_MAP.md` to find relevant code and contracts. Do not read all other project documents by default.
2. Verify exact branch, allowed edits, locked interfaces and acceptance tests; stop if missing or unapproved.
3. Implement only that task. Test relevant affected modules first; fix only task-caused regressions.
4. Review diff and secrets; commit and immediately push approved checkpoints to the remote feature branch. Open a PR targeting `develop` and do not merge.
5. Return a brief status, tests, changed paths, remote SHA/PR and blockers. Stop at review gate.

Never spawn subagents unless the task explicitly authorizes them.
