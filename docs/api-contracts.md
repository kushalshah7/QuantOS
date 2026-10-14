# Phase 0 native and WASM boundary

Only these build-stage interfaces exist:

| Surface | Interface | Behavior |
| --- | --- | --- |
| Core Rust | `foundation_status() -> &'static str` | Static P0 status; no IO |
| WASM adapter | `foundation_status() -> String` | Copies the core status; wasm-bindgen export |
| Native CLI | `quantos-cli` | Prints the core status and a newline; exits successfully |
| Web | Static React page | Clearly labels P0 and unavailable market functionality |

Status text: `QuantOS P0 foundation; market functionality unavailable`.
The adapter and CLI are tested against the same core value. The adapter compiles
for `wasm32-unknown-unknown`; browser glue generation and worker calls do not yet
exist. There is no JavaScript/native cross-runtime financial equivalence claim.

Future domain APIs require an approved task specifying input/output schemas,
versioning, error types, validation, cancellation/progress and ownership across
workers. React must use the WASM adapter for domain work; it must not reimplement
Rust financial calculations. Do not invent a backtest request/result API in P0.
