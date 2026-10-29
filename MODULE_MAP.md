# QuantOS — module / contract router (keep short and current)

Open this before code exploration. Route to the **smallest** source area. Update on changes to module boundaries, not for every commit.

| Work category | Owning paths | Contract(s) | Focused tests/checks |
| --- | --- | --- | --- |
| Candles, schema, import validation | `crates/quantos-core/src/data.rs`, `src/data/` within that crate; `tests/normalization.rs`, `tests/normalization_cases/`; `fixtures/synthetic/` | `docs/data-contract.md`, `docs/api-contracts.md` | `cargo test -p quantos-core --test normalization --locked` (existing workspace CI discovers it) |
| UTF-8 CSV bytes to exact field strings | `crates/quantos-core/src/data/csv.rs`; `tests/csv.rs`, `tests/csv_cases/` in that crate | `docs/csv-contract.md`, `docs/api-contracts.md` | `cargo test -p quantos-core --test csv --locked` (existing workspace CI discovers it) |
| Provided-signal fills and position ledger | `crates/quantos-core/src/execution.rs`, `src/execution/`; `tests/execution.rs`, `tests/execution_cases/` in that crate | `docs/execution-api-v1.md`, `docs/backtest-execution-v1-draft.md`, `docs/backtest-semantics.md` | `cargo test -p quantos-core --test execution --locked` |
| Browser Rust bindings | `crates/quantos-wasm/` | `docs/api-contracts.md` | `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked`; browser packaging later |
| Native/local research CLI | `crates/quantos-cli/src/`, `tests/import.rs`, `tests/import_cases/` in that crate | `docs/cli-import.md`, `docs/api-contracts.md` | `cargo test -p quantos-cli --locked` |
| UI and browser workers | `apps/web/` | `docs/api-contracts.md`, UI task card | root `npm run lint`, `npm run typecheck`, `npm test`, `npm run build` |
| Independent math/data verification | `scripts/analytics/` | data/backtest contracts | Python test specified by task |
| Future local private broker bridge | `services/local-bridge/` | explicit *future* broker spec | no live calls in default CI |
| CI, release, hosting | `.github/workflows/`, hosting config | `DEVOPS_CICD.md` | GitHub Actions jobs / smoke tests |
| Project governance | `AGENTS.md`, `CODEX_PROTOCOL.md`, `GIT_WORKFLOW.md`, `docs/tasks/` | task card + governance | PR review |

**Boundary note:** Core now provides the Rust-only schema-1 `data::normalize_v1`
for already-parsed in-memory tables. P0 fixture tests still verify examples only;
the normalization integration suite tests the domain API. Rust-only
`data::parse_csv_v1` separately parses CSV bytes, with test-only composition into
normalization. Parser success does not assert valid market data. Native CLI
`import` composes these core APIs with explicit caller configuration and bounded
read-only IO; no-argument status is preserved. WASM/browser import bindings remain
unimplemented. QOS-0009 adds Rust-only `execution::execute_v1` for supplied
close decisions, exact execution/accounting and atomic errors. It reuses and
revalidates P1 types without changing P1; strategy generation and financial
CLI/WASM/UI surfaces remain separate. This router does not authorize inspecting
all paths.
