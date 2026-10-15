# ADR 0001: Shared local-first Rust core

Status: implemented boundary from the approved P0 work order.

Native CLI and browser WASM share `quantos-core`. The browser adapter is a
separate crate; React is responsible for interaction/presentation, not financial
arithmetic. Public execution needs no hosted backend. Private data and any
future broker bridge remain isolated from the public application and CI.

This avoids divergent financial implementations. Browser bindings and worker
integration need a later task; P0 proves crate boundaries and target compilation
using foundation status alone. No backtest or CSV implementation is implied.
