# QuantOS v3 — Git-first, CI/CD, token-efficient Codex starter

## Intended workflow

You own requirements, architecture and release decisions. Codex implements *one approved task card at a time* and returns a GitHub PR with test evidence. An `AGENTS.md` routing file, task cards, `MODULE_MAP.md` and optional minimal local skills keep future sessions from re-reading the full repository.

## How to start

1. Extract **this entire ZIP** into an otherwise empty `QuantOS/` folder. Keep all folders, including the hidden `.agents/skills/` folder.
2. Open that directory in Codex. Select a capable model for the one-time P0 Git/architecture bootstrap; local authentication to GitHub may still need to be configured.
3. Paste **only** this instruction:

   `Execute CODEX_PHASE_0_PROMPT.md. Follow QOS-0001. Set up the real GitHub repo and CI, open the P0 PR, then STOP without merging.`

4. Review Codex's reported repository URL, branch, GitHub Actions status, tests, PR and files. Do not start P1 until the PR has been reviewed and accepted.
5. On each subsequent task, create an *approved* one-outcome card using `templates/TASK_TEMPLATE.md`, choose appropriate model/reasoning effort, and prompt: `Implement QOS-XXXX using CODEX_PROTOCOL.md. Stop at the PR review gate.`
6. On another laptop, clone from GitHub and `git fetch` / `git pull --ff-only` before editing; never transfer private candles through the public repo.

## Important boundaries

- This ZIP is a **starter specification**, not an initialized GitHub repository or finished application. GitHub repo/branch/CI setup must actually occur in the Codex environment; the connected ChatGPT plugin alone does not ensure Codex GitHub auth.
- Never commit Groww market datasets, Zerodha tokens or private logs. Use synthetic fixtures in the public repository, import your own data locally. Check source and redistribution permissions before public publication.
- Any claim that different models produce the same results is an objective to *test*, not a guarantee. Scoped tasks plus executable assertions are designed to narrow differences.
- Subagents are intentionally **disabled by default** in the workflow because they usually consume more tokens. Ask for separate read-only reviews when complexity warrants them.
