# QOS-0005 — Native CLI CSV import and JSON report

- **Status:** APPROVED (owner submitted QOS-0005 prompt)
- **Phase:** P1 subtask, not P1-wide acceptance
- **Branch:** `feature/qos-0005-cli-import` from `origin/develop`
- **PR target:** `develop`
- **Risk:** high (bounded IO, argument/config precedence and data-bearing stdout)
- **Verified prerequisite:** accepted PR #4 squash merge/current remote develop
  `46679d2f6ce069513a7b0efd845d10770be27a04`.
- One agent; clean isolated worktree; preserve all untracked/ignored owner files.

## Goal
Read CSV/config files through a usable native command, invoke the unchanged core
parser/normalizer, and emit exact typed JSON reports with deterministic exits.

## Read first
- `MODULE_MAP.md`
- `docs/api-contracts.md`
- `docs/csv-contract.md`
- `docs/data-contract.md`
- CLI source/manifest/existing smoke test; core result exports only as needed

## Allowed edits
- `crates/quantos-cli/src/main.rs`, CLI-private helpers under `src/**`
- `crates/quantos-cli/tests/import.rs`, `tests/import_cases/**`
- `crates/quantos-cli/Cargo.toml`
- `Cargo.lock` (CLI dependency edges only; no package/version/checksum changes)
- `fixtures/synthetic/import-config.json`, fixture README reference only
- `docs/cli-import.md`, `docs/api-contracts.md` (CLI section/limits only)
- `README.md` (concise availability/example link only)
- `MODULE_MAP.md` (CLI routing/boundary note only)
- This card and `PROGRESS.md` (task/handoff and accepted QOS-0004 evidence)

Core/contracts/fixtures/existing tests including CLI smoke, workspace/toolchain/
workflow, WASM/web are read-only. No private data, broker/network feature or new
public library/combined-import API. Only new direct dependencies: serde exactly
1.0.229 with derive and serde_json exactly 1.0.154, reusing already locked packages.

## Locked contract / invariants
- Preserve no-argument foundation status/newline, exit 0, empty stderr. Sole
  --help is static plain usage, exit 0, empty stderr.
- Only import --csv <path> --config <path>, both flags once in either order.
  No positionals, aliases/defaults/stdin/extra flags. Usage errors and path '-'
  yield USAGE before file access. args_os/native paths; relative to process cwd.
- Read only. Config is UTF-8 JSON without BOM and exactly root keys schema_version,
  metadata, options. Preserve child values/types for core validation without defaults.
- Malformed/trailing/UTF-8/BOM JSON or duplicate keys at any depth -> CONFIG_JSON;
  retain default serde_json nesting limit. Bad root/envelope keys -> CONFIG_ENVELOPE.
- Precedence: args, config read/size, JSON, envelope, CSV read/size, core CSV parse,
  then normalization only on parse success. Do not duplicate domain validation.
- Limits: config 1048576, CSV 67108864 bytes, inclusive; read at most limit+1
  regardless of metadata. Exceeding produces TOO_LARGE, never truncation.
- CLI-private typed Serialize reports and exits exactly as docs/cli-import.md:
  normalization nested result/status unchanged; parser failure nested unchanged;
  operational failure has only closed code/field/fixed sanitized message.
- Exactly one compact UTF-8 JSON object/newline, empty normal stderr, except plain
  no-args/help. Serialize fully before writing. Serialization/write/flush failure:
  fixed sanitized stderr only, exit 3, no panic/retry; broken stdout may be partial.
- Requested stdout contains user candles/provenance: document this fact; no real
  data in GitHub logs. Never echo paths/contents/raw OS/JSON errors operationally.

## Exact implementation
1. Write/push this card and locked CLI document before implementation.
2. Add only approved CLI dependencies; resolve lock edges OFFLINE with existing
   pinned tools/cache, preserving all existing package/version/checksum entries.
3. Add private argument/config/read/report/output helpers using core APIs. Reject
   duplicate JSON keys with a small deserialization visitor, not a general framework.
4. Add the exact synthetic config/example. New tests spawn the actual built CLI
   via CARGO_BIN_EXE_quantos-cli with unique std-only owned temp directories.
5. Cover valid fixtures/flag orders/spaced/native paths; atomic domain/parser errors;
   ISO confirmation; multiline/quoted data; gap warnings; child validation; all
   usage/help/config/IO/duplicate/size/precedence cases; sanitized messages;
   repeatability/input preservation; injected writer failures. Independent full
   output/ordered-issue expectations, ignoring only descriptive message wording.
6. Format with existing pinned rustfmt, inspect scope/sensitive content/diffs,
   push checkpoints and open one PR. Inspect actual final-head new CLI tests plus
   unchanged CSV/normalization/P0 suites. Stop before merge.

## Verification and proof
- LOW-RESOURCE: local compilation/tests/builds, CLI execution, installations,
  servers, benchmarks and historical-data processing NOT RUN/forbidden.
- Exceptions only: existing pinned rustfmt, lightweight text/Git inspection,
  git diff --check and OFFLINE lock resolution. No registry refresh/downloads.
- If valid offline lock resolution fails, stop BLOCKED; never fabricate it.
- Existing unchanged CI: Rust fmt/Clippy/workspace tests/WASM compile and web
  quality. Inspect final-head new import-suite output and unchanged core/P0 tests;
  never assert test counts in tests or rely on badges/earlier heads.
- macOS/browser/native-WASM runtime parity NOT RUN. Synthetic temporary size-limit
  files are permitted in CI, including sparse files; no real dataset exercise.

## Review/stop gate
- Allowed paths only, final-head required CI green, actual suites inspected,
  remote SHA verified, one PR ready for owner review. Stop on push failure.
- Follow CODEX_PROTOCOL bounded fixes. Missing observable semantics/out-of-scope
  changes -> BLOCKED with one concrete amendment, no silent widening.
- No merge/release/deployment, broker calls, browser/WASM work, P1 acceptance or
  next task.

## Handoff
- Status: NOT STARTED
- Paths changed:
- Local tests/builds/CLI execution: NOT RUN (LOW-RESOURCE).
- Offline lock/formatter/diff evidence:
- Final-head CI and actual import-suite evidence:
- Remote SHA / PR:
- Risks / owner review:
