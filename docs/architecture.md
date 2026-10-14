# Phase 0 architecture

Status: foundation contract for QOS-0001; domain behavior is not implemented.

```text
Native CLI ---------------------> quantos-core
React (future worker boundary) -> quantos-wasm -> quantos-core
```

- `quantos-core` owns domain types, validation, indicators, fills, accounting,
  and risk metrics when approved later. It must remain usable without browser,
  network, broker, or UI dependencies. P0 exposes build-stage text only.
- `quantos-wasm` is a separate adapter with no duplicate financial logic. P0
  compiles a wasm-bindgen export; generated JavaScript packaging and worker
  integration are deferred to an approved browser task.
- `quantos-cli` depends on the same core. P0 prints foundation status and accepts
  no financial parameters or datasets.
- `apps/web` is a static React/TypeScript/Vite placeholder. It currently does
  not load WASM, import data, display market charts, or produce financial values.

Public execution is local-first: future imported data stays in browser memory.
Persistence needs explicit user consent. No mandatory backend, telemetry,
hosted data, paid service, broker credentials, or deployment is introduced here.
A future broker bridge requires a separate local-only security and terms review.

Financial boundaries live in [data-contract.md](data-contract.md) and
[backtest-semantics.md](backtest-semantics.md). Native/browser interfaces live in
[api-contracts.md](api-contracts.md). Decisions are recorded in `decisions/`.
Use the task cards and `MODULE_MAP.md` for subsequent implementation scope.
