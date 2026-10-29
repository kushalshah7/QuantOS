# QuantOS

An open-source, local-first quantitative finance terminal built around a shared
Rust core, a separate WebAssembly adapter, and a React/TypeScript frontend.

P1's native data path is implemented: CSV parsing, explicit candle normalization
and JSON data-quality reports. The owner accepted P1 on 2026-10-29; the
[P1 acceptance packet](docs/p1-acceptance.md) records the decision and evidence.
[Personal-Mac validation](docs/p1-macos-validation.md)
ran on 2026-10-29: synthetic imports, all 68 Rust tests and web checks passed
after an owner-approved test portability fix. The owner subsequently delegated
approval to proceed within the scoped workflow; [D01–D12](docs/backtest-execution-v1-draft.md)
were accepted through PR #8. [QOS-0009](docs/tasks/QOS-0009-P2-EXECUTION-KERNEL.md)
implements a Rust-only provided-signal execution kernel, now under review in
[PR #9](https://github.com/kushalshah7/QuantOS/pull/9). Its
[typed API](docs/execution-api-v1.md) returns exact fills, accounting and atomic
errors; all 107 Mac Rust tests pass, including 39 execution tests.
The browser UI/WASM adapter still expose P0 surfaces. Strategy generation,
financial CLI/browser bindings, charts, broker integration and deployment remain
unimplemented; aggregate P2 acceptance is pending.
No market datasets or broker credentials belong here.

Native CSV import is now available through `quantos-cli import`; see the
[CLI interface and synthetic personal-machine example](docs/cli-import.md).
The JSON report intentionally contains the imported candles/provenance.

Start with [START_HERE.md](START_HERE.md). Coding agents follow [AGENTS.md](AGENTS.md),
[CODEX_PROTOCOL.md](CODEX_PROTOCOL.md), and the approved card in `docs/tasks/`.
Git and CI rules are in [GIT_WORKFLOW.md](GIT_WORKFLOW.md) and
[DEVOPS_CICD.md](DEVOPS_CICD.md).

## Build and check the foundation

Use Rust/rustup (the pinned toolchain installs rustfmt, Clippy and the WASM target)
and Node **24.19.0** with npm **11.17.0**. On Windows, use installed MSVC C++
build tools or rustup's GNU host for this pure-Rust skeleton. Commands below run
from the repository root on Windows, macOS, or Linux:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked
cargo run -p quantos-cli --locked
npm ci --ignore-scripts
npm run lint
npm run typecheck
npm test
npm run build
```

`npm run dev` starts the placeholder locally on loopback. Generated files stay
ignored. GitHub CI uses the same quality commands on Ubuntu; it does not deploy
the application. Browser WASM packaging and runtime integration are not present.

See [docs/architecture.md](docs/architecture.md) and the linked data/backtest/API
contracts for the approved boundaries. Synthetic contract examples live in
`fixtures/synthetic/`; private datasets stay outside Git under ignored `data/`.

Licensed under [Apache-2.0](LICENSE), selected by the owner for Phase 0.
