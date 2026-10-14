# Codex — Execute QuantOS P0, then stop

**Approved task:** `docs/tasks/QOS-0001-P0.md` only. **Protocol:** `CODEX_PROTOCOL.md`; **persistent rules:** `AGENTS.md`. This is a *one-time* foundation/bootstrap task, so read the architecture and Git/DevOps governance files listed on the P0 task card. Do not recursively read the complete repository. No subagents for P0.

## Step 1 — Remote GitHub first

1. Inspect `git status`, `git remote -v`, and available GitHub authentication (`gh auth status` or supported GitHub connector). An integration in ChatGPT does not prove this Codex runtime has the same access. Determine the authenticated owner; do not guess.
2. If an existing `QuantOS` repository belongs to the authenticated owner, examine/reuse it instead of creating a duplicate. Otherwise, create a public GitHub `QuantOS` repository **only after inspecting files for private Groww data, secrets and proprietary material**. If privacy cannot be established, stop.
3. For an empty repo, seed `main` with reviewed documentation-only files (the supplied Markdown policy/protocol files, brief README, `.gitignore`, template and skills), commit and push. This is the sole initial direct commit to `main`.
4. Establish and push `develop` from remote `main`. Create/push `feature/p0-foundation` from `develop`, set upstream. Do not develop on `main` or `develop` thereafter.
5. Commit and push each coherent tested checkpoint. If a push fails, stop; no unpushed completed work may be represented as synchronized.

## Step 2 — Implement task QOS-0001 only

Follow the P0 card exactly. Add architecture contracts, minimal Rust workspace / WASM adapter / CLI skeleton, minimal React+TypeScript+Vite app, synthetic fixtures, tests, scripts needed for reproducible builds and real GitHub Actions quality jobs. Keep the provided `AGENTS.md` **short**, and route future agents through `MODULE_MAP.md`, task cards and relevant contracts. Do not build P1 data import, backtesting, charts, Zerodha bridge, or AI features.

Use targeted tests while editing; run the specified P0 complete acceptance suite once prior to PR. Inspect the staged diff for secrets and private datasets before every push. Do not install an elaborate agent orchestration framework or enable default parallel subagents.

## Step 3 — CI, protection, review gate

Once the jobs have run, enable available protection for `main` and `develop` requiring PRs, viable existing CI check names, and preventing force push/deletion. For a solo maintainer, do not configure an impossible second-user approval rule. Never claim unavailable branch rules were successfully applied.

Open a PR **from `feature/p0-foundation` into `develop`**, attach test/CI evidence, remote SHA and brief summary. Record the outcome in `PROGRESS.md`; verify remote branch and clean local state. **Do not merge, release, publish market data or start P1.**

## Final response — five compact lines

`STATUS: REVIEW | BLOCKED`
`GIT: <actual repo> | <feature branch> | <remote SHA> | <PR URL>`
`CHANGED: <main paths>`
`CHECKS: <actual local commands + pass/fail>; <actual CI URL/status>`
`BLOCKERS/DECISIONS: <only material unresolved issues>`

If a tool cannot run, say `NOT RUN`, not `PASSED`. Report exactly what permission or prerequisite is missing and stop.
