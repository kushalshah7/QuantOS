# ADR 0002: Reproducible Phase 0 quality gates

Status: implemented pipeline from the approved P0 work order.

Pin Rust and Node in toolchain/workspace manifests; commit Cargo and npm
lockfiles. Use GitHub-hosted Ubuntu CI with independent `rust-quality` and
`web-quality` jobs and read-only repository permissions. Rust quality also checks
the real adapter's WASM target; no empty `wasm-build` job or deployment is added.

Required branch checks must be enabled only after those jobs actually run.
Protect `main` and `develop`, require PRs and resolved conversations, block
force pushes/deletion, and allow a solo maintainer to review and merge without
an impossible second-user approval count. Only squash merges are enabled.

All tests use synthetic fixtures. P0 stops at a PR into `develop`; release,
hosting, complete WASM browser packaging, and additional CI jobs need later
approved tasks. Current evidence belongs in `PROGRESS.md` and the PR.
