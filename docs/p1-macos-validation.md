# P1 personal-Mac validation — 2026-10-29

**Result: verification executed; one macOS test portability blocker remains.**
All four synthetic acceptance imports and the web quality suite passed. The full
Rust suite failed in a fixture-creation step, before calling the CLI. P1 owner
acceptance remains PENDING; no phase advancement, merge or deployment occurred.

## Checkout and environment

- Fresh clone: `https://github.com/kushalshah7/QuantOS`.
- Integration base: `5b6a5005851b63224b4841a3d9dcdac4cf5cb7fa`, merged PR #6.
- Validation branch: `feature/qos-0007-macos-validation`; [PR #7](https://github.com/kushalshah7/QuantOS/pull/7).
- Exact acceptance-import checkout: detached implementation commit
  `0568b5ec5b77e23e484baee012156557a81d8236`; returned to the validation branch.
- Executable sources, tests, CSV/JSON fixtures, manifests, lockfiles, toolchain
  and workflow compare unchanged between that implementation SHA and the branch.
- macOS 27.0, build 26A428; Apple Silicon, `aarch64-apple-darwin`.
- Rust `1.99.0 (b940084d7 2026-09-28)`, Cargo `1.99.0 (5f94df478 2026-08-27)`;
  rustfmt, Clippy and `wasm32-unknown-unknown` installed through rustup.
- Node `24.19.0`, npm `11.17.0`, installed in the user's local QuantOS tool directory.
  The official Node archive's SHA256 matched its published checksum:
  `8294b7aa9b03997481c06babf1e8b270c859358f27da57a11509afe537ac381d`.

Use the installed tools in a new shell:

```sh
export PATH="$HOME/.local/share/quantos/toolchains/node-v24.19.0-darwin-arm64/bin:$HOME/.cargo/bin:$PATH"
```

npm scripts initially found an ancestor `$HOME/node_modules/.bin/node` at Node
21.2.0. This caused a Vitest startup failure (`node:util` lacked `styleText`).
An ignored repository-local `node_modules/.bin/node` symlink to the pinned Node
fixed script resolution. `npm exec --offline --call 'command -v node && node --version'`
then resolved the local shim at v24.19.0; all web checks were rerun successfully.
No ancestor installation or tracked dependency/configuration changed. After a
future `npm ci`, recreate this local shim if that ancestor executable still exists:

```sh
ln -s "$HOME/.local/share/quantos/toolchains/node-v24.19.0-darwin-arm64/bin/node" node_modules/.bin/node
```

## Executed personal-machine checklist

The acceptance packet's four `cargo run -p quantos-cli --locked -- import`
commands ran at the exact implementation SHA. Temporary timezone configs used
the packet's declared synthetic metadata; fixture/config files were not edited.
JSON output was parsed and compared with independent expected candle values,
issue keys, schemas, stage, dispositions and provenance.

| Case | Observed exit | Actual observations | Result |
| --- | --- | --- | --- |
| Valid synthetic CSV | 0 | Three candles at UTC ms 0/300000/600000; OHLC paise and volumes exactly match the packet; no issues; gaps not performed. | PASS |
| Invalid OHLC | 1 | Outer/inner failure, no candles, `INVALID_OHLC` / error / row 2 / null field; no partial results. | PASS |
| Naive timestamp without confirmation | 1 | No candles; `TIMEZONE_REQUIRED` / error / row 2 / timestamp field. | PASS |
| Explicit Asia/Kolkata confirmation | 0 | One candle at UTC ms 13500000; OHLC 10000/10300/9900/10200, volume 10, no issues. | PASS |
| Provenance on every report | As above | Source synthetic, synthetic true, adjustment/redistribution unknown; candle sources synthetic. These are declarations, not verified licensing/adjustment claims. | PASS |
| Host timezone and direct process output | As above | Each built-CLI case matched byte-for-byte under both TZ=UTC and TZ=Asia/Kolkata, with the same exit and empty stderr. Cargo's own build/status diagnostics were recorded separately. | PASS |
| Input preservation | n/a | SHA256 of every existing synthetic CSV/JSON fixture and both lockfiles unchanged before/after imports. | PASS |

Raw synthetic reports, temporary configs and command logs remain outside Git;
no real market dataset, credential or user backtest result was processed.

## Local quality evidence

All commands used the pinned tools. The deterministic dependency install ran at
the implementation checkout; remaining quality checks ran after returning to the
task branch. The manifests, lockfiles and all executable inputs were identical.

| Command | Actual result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | FAIL: import suite 10 passed / 1 failed; Cargo stopped before later suites. |
| `cargo test --workspace --locked --no-fail-fast` | FAIL: 66 passed / 1 failed overall; no ignored/filtered tests. Only the same filename fixture failed. |
| `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked` | PASS; target compilation only. |
| `cargo run -p quantos-cli --locked` | PASS; preserved P0 foundation status, exit 0. |
| `npm ci --ignore-scripts` | PASS; 178 packages added, 180 audited; zero vulnerabilities reported at execution time. |
| `npm run lint` | PASS after fixing local Node resolution. |
| `npm run typecheck` | PASS with pinned Node. |
| `npm test` | PASS; one test in one test file with pinned Node. |
| `npm run build` | PASS; production assets built locally, no hosting/deployment. |

Actual Rust groups in the complete diagnostic run: CLI output units 2 passed;
CLI import 10 passed/1 failed; CLI smoke 1 passed; CSV/pipeline 30 passed;
fixtures 2 passed; normalization 20 passed; native adapter 1 passed.

## Reproduced blocker and proposed scope amendment

`crates/quantos-cli/tests/import_cases/cases.rs:400` marks the raw-byte filename
fixture `#[cfg(unix)]`, which includes macOS. At line 408, creating
`prices\xff.csv` fails with OS error 92, `Illegal byte sequence`; the CLI has not
yet started. Independent file-creation probes reproduced this on both the system
temporary directory and the project filesystem. This is an invalid assumption
about supported test-fixture filenames; the observed imports do not show a
normalizer/CLI calculation defect. The full macOS Rust suite remains FAILED.

Proposed bounded amendment for owner approval: allow edits to this one test file;
keep the successful raw-byte filename case on other Unix targets, add a portable
Unicode CSV/config filename success case, and assert sanitized read failures for
macOS-rejected raw-byte paths. Keep production code, contracts, dependencies and
Linux raw-byte success assertions unchanged. Run focused import tests, then the
full Rust acceptance checks and existing Ubuntu CI. No test waiver is inferred.

## Review gate

Current personal-Mac observations are recorded; the test portability fix needs a
scope amendment under `AGENTS.md`. Required CI will be recorded at the final PR
head. Green Ubuntu CI cannot override the local macOS failure. Owner written P1
acceptance remains pending. Browser/native-WASM runtime parity is NOT RUN because
browser import bindings remain unimplemented. No P2 work is authorized.
