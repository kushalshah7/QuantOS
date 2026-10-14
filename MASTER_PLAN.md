# QuantOS — Master Plan (v3 · Git / DevOps / scoped-agent-governed)

**Date:** 14 October 2026  
**Process:** Agile Kanban + DevOps CI/CD; see `DEVOPS_CICD.md` for CI/release work and `CODEX_PROTOCOL.md` for scoped implementation.

**Mission:** Build an open-source, local-first quantitative finance terminal that is genuinely useful, technically deep, reproducible, and deployable on free tiers.

## 1. Non-negotiable product decisions

1. **Free-tier-only infrastructure.** Public product is a static website; all public backtests run client-side using Rust compiled to WebAssembly. No paid hosting, background servers, or mandatory APIs.
2. **Rust-first.** Performance-sensitive primitives (data validation, indicator calculations, strategy evaluation, fills, equity curves, risk metrics) live in an original deterministic Rust engine. Python is optional for exploratory offline analysis and independently checking calculations, not required to run the web product.
3. **One shared engine.** Native Rust CLI and browser WebAssembly use the same `quantos-core` logic. Avoid separate financial implementations with inconsistent results.
4. **Local-first data.** Users can import CSV datasets into their browser and run backtests without uploading them to our servers. Optional on-device persistence requires explicit consent. Nothing is sent to an analytics service.
5. **Separate public demo from private broker integration.** A future local-only Zerodha connector, if account permissions and provider terms allow, handles credentials on the user's Mac; never expose API keys, sessions, or broker data in public deployments.
6. **Do not redistribute unlicensed market data.** Existing Groww-derived files are personal/local inputs until redistribution rights are verified. A public repository and demo can contain *synthetic* sample OHLCV and public-domain/permitted datasets only.
7. **No live order placement in MVP.** No production credentials or real-money trades in CI, and no live brokerage execution by default. A brokerage adapter is an optional future phase with an explicit security and terms review.
8. **No decorative fake outputs.** Label sample/simulated data clearly. Financial metrics must trace to actual engine output.
9. **Phase gating.** A phase is complete only when automated tests, hands-on validation, and a written reviewer checklist pass.
10. **GitHub-first development on every laptop.** Treat `GIT_WORKFLOW.md` as mandatory: `main` is release-only, `develop` is integration, and each phase runs on a pushed `feature/*` branch; use PRs and green CI, immutable release tags, and no direct edits to protected branches. Commit and push every small, safe logical checkpoint. Do not continue if a push fails.

## 2. Product scope

### Public MVP
- Import OHLCV CSV and inspect data-health report.
- View interactive candlestick chart and timeframe metadata.
- Configure **one-symbol, long-only** backtests with preset strategies (SMA crossover, EMA crossover, breakout; add custom strategy DSL only once basics work).
- Configure initial cash, trade size, brokerage/fees, slippage, stop-loss and take-profit; show assumptions explicitly.
- Run deterministic Rust/WASM backtests entirely in-browser.
- View equity curve, executed trade ledger, total return, drawdown, volatility and Sharpe (with defined sampling/annualization conventions).
- Export trades, parameters and summary results in CSV/JSON.
- Free static public deployment with synthetic examples, documentation and screenshots.

### Next iteration (not MVP)
- More advanced strategy builder and multi-symbol research.
- Portfolio construction, correlation and portfolio optimization.
- Native CLI batch runs, Parquet/DuckDB optimization for large 2020–2026 5-minute datasets.
- Python notebooks for statistical research and cross-validation.
- Optional local-only Zerodha bridge for permitted personal data workflows. Keep it isolated from public website and from simulated/paper-trading products unless broker rules clearly permit use.
- Paper trading against independently permitted feeds; later security-reviewed live trading only with explicit user decision.

### Excluded from MVP
- Accounts, hosted dataset storage, social features, automated live trading, AI chatbot, options pricing, high-frequency execution, real-time data redistribution, and promises of trading profitability.

## 3. Architecture

```text
                    PUBLIC STATIC WEBSITE (free hosting)
           ┌───────────────────────────────────────────┐
           │ React + TypeScript + Vite + chart library  │
           │ Upload CSV, strategy builder, results UI   │
           ├───────────────────────────────────────────┤
           │ Web Worker -> quantos-wasm                 │
           │ Rust quantos-core (backtest + risk engine) │
           ├───────────────────────────────────────────┤
           │ Browser memory / optional IndexedDB        │
           │ No mandatory backend / no dataset upload   │
           └───────────────────────────────────────────┘
                             │ same engine
                    NATIVE MAC RESEARCH TOOL
           ┌───────────────────────────────────────────┐
           │ quantos-cli -> quantos-core                │
           │ local CSV/Parquet -> DuckDB (later)        │
           │ Python scripts (optional, offline)         │
           │ private Groww-derived datasets            │
           └───────────────────────────────────────────┘
                             │ isolated optional adapter
                     PRIVATE ZERODHA BRIDGE
           ┌───────────────────────────────────────────┐
           │ Rust local process; loopback only          │
           │ approved official API + token handling     │
           │ disabled until permissions/terms verified  │
           └───────────────────────────────────────────┘
```

## 4. Suggested repository

```text
quantos/
  AGENTS.md           # concise always-on task routing
  CODEX_PROTOCOL.md  # bounded, test-first execution rules
  MODULE_MAP.md      # route agent to relevant module
  MASTER_PLAN.md
  GIT_WORKFLOW.md
  PROGRESS.md
  START_HERE.md     # one-time starter instructions
  README.md
  LICENSE
  .gitignore
  Cargo.toml
  crates/
    quantos-core/       # Rust domain types, indicators, matching/fills, backtests, metrics
    quantos-wasm/       # WebAssembly bindings, browser-facing API
    quantos-cli/        # Optional local batch execution + benchmarks
  apps/
    web/                # React / TypeScript / Vite, Web Worker integration
  services/
    local-bridge/       # Optional, future, private broker connector
  scripts/
    analytics/          # Optional Python validation/research
  fixtures/
    synthetic/          # Clearly labeled synthetic data safe to publish
  docs/
    architecture.md
    api-contracts.md
    data-contract.md
    backtest-semantics.md
    decisions/
    tasks/                # ONE approved scoped task card at a time
  templates/
    TASK_TEMPLATE.md
  .agents/skills/
    quantos-implement/SKILL.md
    quantos-review/SKILL.md
  .github/
    workflows/            # free GitHub Actions CI
    pull_request_template.md
  data/                  # gitignored: never commit private/third-party data
```

## 5. Core financial contracts (lock before implementation)

### Candle schema
- `symbol`, `exchange`, `interval`, `ts_utc_ms`, `open`, `high`, `low`, `close`, `volume` and provenance metadata.
- CSV importer permits documented mappings from input column names; user explicitly chooses or confirms timezone if absent.
- Store internal timestamps in UTC milliseconds; Indian market session filters explicitly use `Asia/Kolkata`.
- Validate monotonic sorted times, duplicates, OHLC consistency, nonnegative volumes, and missing expected bars. Report missing bars but do not silently invent them.
- Flag corporate action/adjustment ambiguity; results must communicate whether splits/dividends were accounted for.

### Backtest semantics
- Signals see only **completed** candles; signals at bar close fill no earlier than the **next bar open**.
- Costs and slippage configurable and included in returns. No assumption of infinite liquidity.
- Deterministic stop-loss / take-profit precedence for same-bar touches (conservative default; document it).
- Track cash, integer share quantity, positions, fees and realized/unrealized P&L; avoid rounded display values in engine accounting. Use integer paise or decimal for money where practical.
- Formalize trading calendar, risk-free rate, metric annualization and treatment of non-trading days before publishing performance results.
- Identical input + config => identical results from native CLI and WASM (except documented numerical tolerance).

## 6. Milestones and approval gates

| Gate | Deliverable | Must pass before next phase |
|---|---|---|
| **P0 — Architecture** | GitHub repository bootstrap, `main`/`develop`/`feature/p0-foundation` branches, remote push, PR, protection settings, architecture decisions, tested Rust/web scaffold, CI, `DEVOPS_CICD.md` and Agile/DevOps governance | Rust and web skeleton compile; basic CI green; no unnecessary feature code; architecture reviewed |
| **P1 — Data** | CSV importer, normalized schema, data-quality report, synthetic fixtures | known bad files rejected/flagged; UTC/IST tests; local data never published |
| **P2 — Engine** | signals, execution model, indicators, results ledger, configurable fees | deterministic test cases; no same-candle lookahead; account reconciliation; independent reference check |
| **P3 — Browser** | polished terminal, Web Worker WASM integration, charts, progress/error states | usable offline backtest from user CSV; responsive interface; no financial values hardcoded |
| **P4 — Analytics** | performance/risk dashboard, trade ledger and exports | metrics checked against independent reference; explain all assumptions |
| **P5 — Deployment** | Cloudflare Pages or GitHub Pages static website; production auto-deploy from approved `main` merge, preview deployments, smoke tests, docs and demo guide | free-tier live site, branch-separated previews, tested release/rollback, browser compatibility, security/privacy check |
| **P6 — Private broker adapter** | optional local Zerodha integration | terms and account access verified; tokens stay private; no public data redistribution |

**Use independent self-review then human approval at every gate.** Do not move to the next gate autonomously. Every gate is implemented on a dedicated `feature/*` branch and delivered as a PR to `develop` with pushed commits and CI evidence; releasing to `main` happens separately after approval.

## 7. Mandatory GitHub governance and multi-device consistency

- `DEVOPS_CICD.md` is the authoritative Agile delivery, automated CI, preview, release and deployment policy; `AGENTS.md` must point to it.
- `GIT_WORKFLOW.md` is the authoritative step-by-step policy for branching, synchronization, protection rules, PRs, releases, tags, and rollbacks; `AGENTS.md` must point to it.
- Repo: `QuantOS` under the owner's authenticated GitHub account. Prefer **public** for portfolio visibility and branch protection under GitHub Free, but prevent publication of restricted data and secrets. Use existing remote instead of duplicating it.
- Long-lived branches: protected `main` (only approved releases) and protected `develop` (integration). Short-lived `feature/*`, `fix/*`, `release/*`, `hotfix/*` branches; no direct development on `main`/`develop` after the empty-repo seed.
- Agents commit and **push each coherent change**, run CI, and submit a PR to `develop`. The user reviews each phase before merging or starting another. Release branches eventually PR into `main` with annotated semantic-version tags.
- Cross-laptop rule: always fetch/pull the remote branch before changing devices or restarting work; use fast-forward-only pulls. A laptop's local directory is not the source of truth. Never work on the same feature branch simultaneously from two devices.
- No force pushes, destructive resets or bypass of branch policy without explicit owner approval. If push or CI fails, stop and report; never call the phase complete.
- GitHub Free protection is available for public repositories; for private repositories, some enforcement may be unavailable without a paid plan. Don't claim enforcement that isn't actually enabled.

## 8. Codex session hand-off protocol

- First read the approved task card and route through `MODULE_MAP.md`; `AGENTS.md` and `CODEX_PROTOCOL.md` define minimal-context execution. **Do not read the entire plan/workflow or full codebase on every task**; inspect only relevant contract(s), files and latest scoped diff. P0 bootstrap is the exception.
- Work on **one approved task card within one gate**; respect allowed file paths, locked interfaces and exact acceptance tests. Follow `CODEX_PROTOCOL.md` for escalation and bounded retries.
- Prefer small, testable commits and push each immediately to its tracked remote feature branch. No broad rewrites to solve a narrow defect; never push private data or credentials.
- Locally run affected tests first; GitHub CI runs full quality gates on PR. Run whole-project checks locally when the task or a cross-module change requires them.
- Produce a concise evidence-first handoff: changed paths, tests/results, blockers, remote-confirmed SHA(s), branch, CI status and PR link (no raw logs unless needed).
- Update `PROGRESS.md` and await human review; never claim success without execution evidence.

## 9. Market-data and API access constraints (background checked 14 Oct 2026)

- Zerodha access, pricing and entitlements can change. Before the private live-data phase, verify the account's **actual live-data entitlement** against current official terms. Design the MVP so it runs free without Zerodha. Never assume API availability or expose authenticated data to the public demo.
- Zerodha's published API terms prohibit publicly displaying live Kite Connect market data and use with virtual/mock trading apps. Support guidance warns against displaying its data on third-party platforms. Treat a private personal connector as pending a usage review; avoid assuming even local visualization is approved without clarifying terms.
- Groww-downloaded data should remain on the user's machine until redistribution permission is established. Owning a copy of data does not transfer distribution rights.
- All public demo datasets should be synthetic or otherwise clearly licensed for republication. Public web users can always supply their own files directly to their own browsers.

**Reference sources:**
- Zerodha pricing: https://zerodha.com/products/api/
- Zerodha free/paid access and October 2026 credits: https://support.zerodha.com/category/trading-and-markets/general-kite/kite-api/articles/how-do-i-sign-up-for-kite-connect
- Zerodha Kite Connect terms: https://kite.trade/terms/
- Zerodha display restrictions: https://support.zerodha.com/category/trading-and-markets/general-kite/kite-api/articles/can-i-use-historical-and-live-data-taken-from-kite-connect-api-on-other-platforms
- NSE data policy: https://www.nseindia.com/static/market-data/nse-data-policy

## 10. Definition of success

A user opens a free hosted QuantOS website, uploads an OHLCV CSV locally, selects a strategy and realistic assumptions, runs an original Rust/WASM backtest, inspects the trades and performance in a polished terminal, exports results, and can reproduce them through the open-source native engine. No hidden cloud compute cost and no dependence on proprietary public market-data redistribution.

## 11. Efficient coding-agent architecture (v3 addition)

- `AGENTS.md` remains compact (applies to every task).
- `CODEX_PROTOCOL.md` defines the implementer role, read boundaries, bounded test/debug cycles, reporting format and approval gates.
- `MODULE_MAP.md` routes work to its owning module and contract.
- `docs/tasks/QOS-XXXX-*.md` holds a single approved implementation ticket with file allowlist, locked interfaces and executable acceptance evidence.
- `.agents/skills/quantos-implement` and `quantos-review` are optional, narrowly targeted repository skills; no automatic multi-agent orchestration.
- Our architect/owner prepares the task contract; Codex implements it and stops with a PR. Different model versions are evaluated by reproducible behavior and tests, not assumed to output identical code.
