---
name: quantos-review
description: Review an existing QuantOS feature PR for correctness and acceptance-test coverage, without implementing changes. Use only when asked to review a specific PR or task.
---

1. Read the linked task card, changed-file list, diff and relevant contract. Inspect adjacent code only for the changed interface.
2. Look for financial correctness, look-ahead/timezone errors, security/data leaks, performance regressions and missing tests. Prioritize concrete reproducible bugs.
3. Return a concise severity-ordered report: file:line, issue, evidence, proposed test/fix. State when no supported issue was found.
4. Read-only: never merge, release, edit files or launch subagents unless the owner explicitly permits it.
