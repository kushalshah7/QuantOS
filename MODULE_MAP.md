# QuantOS — module / contract router (keep short and current)

Open this before code exploration. Route to the **smallest** source area. Update on changes to module boundaries, not for every commit.

| Work category | Owning paths | Contract(s) | Focused tests/checks |
| --- | --- | --- | --- |
| Candles, schema, import validation | `crates/quantos-core/src/data.rs`, `src/data/` within that crate; `tests/normalization.rs`, `tests/normalization_cases/`; `fixtures/synthetic/` | `docs/data-contract.md`, `docs/api-contracts.md` | `cargo test -p quantos-core --test normalization --locked` (existing workspace CI discovers it) |
| Strategy signals, fills, portfolio ledger | `crates/quantos-core/` (engine module) | `docs/backtest-semantics.md` | `cargo test -p quantos-core` |
| Browser Rust bindings | `crates/quantos-wasm/` | `docs/api-contracts.md` | `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked`; browser packaging later |
| Native/local research CLI | `crates/quantos-cli/` | `docs/api-contracts.md` | `cargo test -p quantos-cli` |
| UI and browser workers | `apps/web/` | `docs/api-contracts.md`, UI task card | root `npm run lint`, `npm run typecheck`, `npm test`, `npm run build` |
| Independent math/data verification | `scripts/analytics/` | data/backtest contracts | Python test specified by task |
| Future local private broker bridge | `services/local-bridge/` | explicit *future* broker spec | no live calls in default CI |
| CI, release, hosting | `.github/workflows/`, hosting config | `DEVOPS_CICD.md` | GitHub Actions jobs / smoke tests |
| Project governance | `AGENTS.md`, `CODEX_PROTOCOL.md`, `GIT_WORKFLOW.md`, `docs/tasks/` | task card + governance | PR review |

**Boundary note:** Core now provides the Rust-only schema-1 `data::normalize_v1`
for already-parsed in-memory tables. P0 fixture tests still verify examples only;
the new normalization integration suite tests the domain API. CSV parsing, native
CLI commands and WASM/browser normalization bindings are not implemented. CLI,
WASM adapter and web retain their P0 surfaces. This is a router, not an instruction
to inspect all paths.
