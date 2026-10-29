# P1 personal-Mac validation — 2026-10-29

**Result: Mac validation and Ubuntu CI PASSED; P1 ACCEPTED on 2026-10-29.**
All four synthetic acceptance imports, all 68 Rust workspace tests and the web
quality suite passed. An initial fixture-creation failure was resolved through
the owner's approved test-only amendment. The owner later accepted P1 and
authorized PR #7's squash merge; see the closing record below. The validation
implementation itself performed no phase advancement, merge or deployment.

## Checkout and environment

- Fresh clone: `https://github.com/kushalshah7/QuantOS`.
- Integration base: `5b6a5005851b63224b4841a3d9dcdac4cf5cb7fa`, merged PR #6.
- Validation branch: `feature/qos-0007-macos-validation`; [PR #7](https://github.com/kushalshah7/QuantOS/pull/7).
- Exact acceptance-import checkout: detached implementation commit
  `0568b5ec5b77e23e484baee012156557a81d8236`; returned to the validation branch.
- Before the approved amendment, executable sources/tests and all build inputs
  compared unchanged with that implementation SHA. The only subsequent executable
  edit is the approved native-path test file; production sources, CSV/JSON
  fixtures, manifests, lockfiles, toolchain and workflow remain unchanged.
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
task branch. Production/build inputs remain identical; only the approved native-
path tests changed. After that change, focused import tests and the full Rust
fmt/Clippy/test/WASM checks were rerun. Prior web checks retain unchanged inputs.

| Command | Actual result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS |
| `cargo test -p quantos-cli --test import --locked` | PASS after amendment: 12 passed / 0 failed / 0 ignored / 0 filtered. |
| `cargo test --workspace --locked` | PASS after amendment: 68 passed / 0 failed / 0 ignored / 0 filtered. Initial pre-amendment run failed during filename fixture creation. |
| `cargo test --workspace --locked --no-fail-fast` | Historical pre-amendment diagnostic: 66 passed / 1 failed; no ignored/filtered tests. Only the filename fixture failed; this is superseded by the full passing run above. |
| `cargo check -p quantos-wasm --target wasm32-unknown-unknown --locked` | PASS; target compilation only. |
| `cargo run -p quantos-cli --locked` | PASS; preserved P0 foundation status, exit 0. |
| `npm ci --ignore-scripts` | PASS; 178 packages added, 180 audited; zero vulnerabilities reported at execution time. |
| `npm run lint` | PASS after fixing local Node resolution. |
| `npm run typecheck` | PASS with pinned Node. |
| `npm test` | PASS; one test in one test file with pinned Node. |
| `npm run build` | PASS; production assets built locally, no hosting/deployment. |

Actual Rust groups in the amended full run: CLI output units 2 passed;
CLI import 12 passed; CLI smoke 1 passed; CSV/pipeline 30 passed;
fixtures 2 passed; normalization 20 passed; native adapter 1 passed.

## Reproduced portability failure and approved fix

The [pre-amendment test](https://github.com/kushalshah7/QuantOS/blob/d739287e1a3f4cfeaf0a39e762e5a56c481a10b1/crates/quantos-cli/tests/import_cases/cases.rs#L400)
marked the raw-byte filename fixture `#[cfg(unix)]`, which includes macOS.
Creating `prices\xff.csv` failed at old line 408 with OS error 92, `Illegal byte
sequence`; the CLI had not yet started. Independent file-creation probes
reproduced this on both the system
temporary directory and the project filesystem. This is an invalid assumption
about supported test-fixture filenames; the observed imports do not show a
normalizer/CLI calculation defect. The initial full macOS Rust suite FAILED.

The owner selected "Fix the test and finish validation" on 2026-10-29. Amendment
`d8d926d0f6c70dc6cae3528713363abed42324be` was pushed before editing the approved
test file. Fix checkpoint `f325279a4485db7fdc4c098f642e8904e869a4a3`:

- Retains unchanged raw-byte filename success assertions on non-macOS Unix.
- Adds successful Unicode CSV/config filenames on all platforms.
- On macOS, passes raw-byte native paths directly to the CLI and asserts exact
  sanitized CSV_READ / CONFIG_READ reports, exit 3 and empty stderr, without
  attempting to create filenames the filesystem rejects.
- Changes no production code, dependency, contract or other test file. Focused
  and full local Rust checks pass; no ignored tests, runtime skips or waiver.

## Review gate

Local validation is complete under the owner's amendment. Fix checkpoint
`f325279a4485db7fdc4c098f642e8904e869a4a3` passed both required quality jobs in
[PR CI](https://github.com/kushalshah7/QuantOS/actions/runs/40845132505) and
[push CI](https://github.com/kushalshah7/QuantOS/actions/runs/40845123060).
Actual Ubuntu Rust output confirms 12 import tests passed, including both the
retained raw-byte success case and the new Unicode success case. The full Mac
run instead executes the macOS read-error case plus Unicode success, both PASS.
The final documentation-handoff head/checks are recorded in PR #7.
The owner explicitly accepted P1 and authorized the reviewed head
`da33ea0e199afe4d732afe6f45dc874a2dc8981e` to merge on 2026-10-29. Protected
PR #7 squash merge: `21ac34874550e89873f677cb395140e0ebbd957f`; no bypass.
Browser/native-WASM runtime parity remains NOT RUN because import bindings are
unimplemented. QOS-0008 contract drafting is separately authorized; no P2 engine,
release or deployment is authorized by this acceptance.
