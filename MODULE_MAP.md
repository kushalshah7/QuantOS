# QuantOS — module / contract router (keep short and current)

Open this before code exploration. Route to the **smallest** source area. Update on changes to module boundaries, not for every commit.

| Work category | Owning paths | Contract(s) | Focused tests/checks |
| --- | --- | --- | --- |
| Candles, schema, import validation | `crates/quantos-core/` (data module), `fixtures/synthetic/` | `docs/data-contract.md` | `cargo test -p quantos-core` |
| Strategy signals, fills, portfolio ledger | `crates/quantos-core/` (engine module) | `docs/backtest-semantics.md` | `cargo test -p quantos-core` |
| Browser Rust bindings | `crates/quantos-wasm/` | `docs/api-contracts.md` | `cargo check -p quantos-wasm` + later WASM build |
| Native/local research CLI | `crates/quantos-cli/` | `docs/api-contracts.md` | `cargo test -p quantos-cli` |
| UI and browser workers | `apps/web/` | `docs/api-contracts.md`, UI task card | `npm run typecheck`, `npm test` (as configured) |
| Independent math/data verification | `scripts/analytics/` | data/backtest contracts | Python test specified by task |
| Future local private broker bridge | `services/local-bridge/` | explicit *future* broker spec | no live calls in default CI |
| CI, release, hosting | `.github/workflows/`, hosting config | `DEVOPS_CICD.md` | GitHub Actions jobs / smoke tests |
| Project governance | `AGENTS.md`, `CODEX_PROTOCOL.md`, `GIT_WORKFLOW.md`, `docs/tasks/` | task card + governance | PR review |

**P0 note:** The target modules may not exist yet; use P0's task card to create only skeletons. This file is a router, not an instruction to inspect all paths. Avoid building a code-embedding/RAG indexing service before the repo is large enough to justify it.
