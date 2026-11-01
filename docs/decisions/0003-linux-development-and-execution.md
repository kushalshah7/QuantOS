# ADR 0003 — Linux development and native execution

Date: 2026-11-01. Direction authorized by the owner's Linux Integration Directive.
Implementation is reviewed through QOS-LINUX-001; no merge or release is implied.

## Decision

Linux is a first-class development and native execution platform alongside
supported macOS use. The same quantos-core remains shared by native CLI and the
separate WASM adapter. The public product stays local-first and browser-executed;
Linux does not introduce a mandatory server or move user uploads to a backend.

Use a small, reproducible GitHub Codespaces container for optional interactive
Linux work, and standard public GitHub Actions runners for native x86-64/ARM64
execution evidence. A local VM/Docker installation is unnecessary on the owner's
8 GB M2 Mac. Codespaces usage is conditional on available included quota and
confirmed no-paid-overage controls; its account settings are not changed here.

QOS-LINUX-001 retains Rust/Node/npm pins and dependency locks. An immutable
multi-architecture Python image pins Python and its C/C++ build tools. No native
component is rewritten, and no C++ financial implementation is introduced.

## Consequences and boundaries

- Existing Linux x86-64 tests are acknowledged, not duplicated as a new engine.
- ARM64 runs the Rust tests natively; cross-compilation alone is insufficient.
- WASM compilation remains required. Browser financial parity is a later gate.
- Private Groww datasets, broker credentials and local results stay off Git,
  build contexts, Codespaces, CI and public logs. This milestone uses synthetic
  inputs only; it does not authorize transferring local data to cloud storage.
- Native backtest CLI integration, DuckDB/Parquet, analytics and Zerodha remain
  separate work. A future private worker requires a bounded job API, resource
  measurements, shutdown/error/security contracts and its own approval.
- No systemd service, cgroups policy, queue server, orchestration platform,
  container registry publication, deployment or paid resource is introduced.
- Image/toolchain upgrades are reviewed changes with fresh runtime evidence;
  immutable pins trade automatic updates for deliberate maintenance.

See [Linux plan and evidence](../linux.md) and the
[bounded task](../tasks/QOS-LINUX-001.md).
