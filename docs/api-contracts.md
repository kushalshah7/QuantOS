# Native and WASM boundaries

The existing P0 build-stage interfaces remain unchanged:

| Surface | Interface | Behavior |
| --- | --- | --- |
| Core Rust | `foundation_status() -> &'static str` | Static P0 status; no IO |
| WASM adapter | `foundation_status() -> String` | Copies the core status; wasm-bindgen export |
| Native CLI | `quantos-cli` (no arguments) | Prints the core status and a newline; exits successfully |
| Web | Static React page | Clearly labels P0 and unavailable market functionality |

Status text: `QuantOS P0 foundation; market functionality unavailable`.
The adapter and CLI are tested against the same core value. The adapter compiles
for `wasm32-unknown-unknown`; browser glue generation and worker calls do not yet
exist. There is no JavaScript/native cross-runtime financial equivalence claim.

Future domain APIs require an approved task specifying input/output schemas,
versioning, error types, validation, cancellation/progress and ownership across
workers. React must use the WASM adapter for domain work; it must not reimplement
Rust financial calculations. Do not invent a backtest request/result API in P0.

## QOS-0003 Rust-only normalization API

```rust
pub fn normalize_v1(
    input: &serde_json::Value,
) -> quantos_core::data::NormalizationResultV1;
```

This QOS-0003 domain entry point takes an in-memory JSON
value containing an already-parsed CSV table, explicit options and provenance.
Its public path is `quantos_core::data::normalize_v1`.
It reports malformed/missing/unknown properties without strict deserialization
discarding independently detectable errors. This normalization entry point does
not parse CSV/JSON text or add file IO, network access, host-timezone lookup,
worker protocol or cancellation behavior.

The accepted [schema-1 contract](data-contract.md) defines the exact fields,
lexical formats, issue codes and ordering. Supporting public result types live
under `quantos_core::data`: `NormalizationResultV1`, `NormalizedCandleV1`,
`ProvenanceV1`, `IssueV1`, `NormalizationStatusV1`, `GapAssessmentV1`, `IntervalV1`,
`IssueCodeV1`, `IssueSeverityV1`, `AdjustmentStatusV1`, `RedistributionStatusV1`.
They implement Serde Serialize; no Deserialize API is introduced. Serialization
uses exact schema strings/nulls/arrays and a flat result object; gap variants use
the schema's internal `status` tag without additional enum wrappers.

Input prices are decimal rupees; normalized OHLC are positive integer paise.
Integers are bounded before output. Any error returns no candles and gap
assessment not performed; valid explicit grids alone produce gap warnings.
Messages describe rules without echoing raw private cell values. P0 foundation
status remains a build-stage string, not a backtest/normalizer result.

The accepted contract/golden documents retain their QOS-0002 specification-stage
status text. Current implementation and verification evidence belongs here and
in `PROGRESS.md`; their accepted semantics have not been edited.

`tests/normalization.rs` exercises all golden IDs/alternatives, structural and
numeric/time boundary cases, deterministic issue serialization/messages, and
isolated TZ=UTC/Asia/Kolkata subprocesses. Local tests/builds are NOT RUN in
LOW-RESOURCE mode. Existing pinned rustfmt and Cargo lockfile resolution are the
only local tool exceptions. Acceptance requires final-head CI output proving the
new suite actually ran. The unchanged WASM target check proves compilation only,
not browser execution or native/WASM normalization-result parity. macOS NOT RUN.

## QOS-0004 Rust-only CSV byte API

Public path: `quantos_core::data::parse_csv_v1`.

```rust
pub fn parse_csv_v1(input: &[u8]) -> quantos_core::data::CsvParseResultV1;
```

Re-exported types are `CsvParseResultV1` (tagged success/failure enum),
`CsvParseIssueV1` and closed `CsvParseCodeV1`. They implement Serialize only using
existing dependencies. Exact fields, offsets, UTF-8/BOM/quote/separator rules and
six parser-only codes are fixed in [csv-contract.md](csv-contract.md). Parser
failure never exposes a partial table, and messages do not echo raw CSV contents.

Success preserves exact decoded strings; it does not validate header names,
widths, identity, numeric/time semantics or valid market data. Callers must
explicitly construct the existing normalize_v1 request with metadata/options;
the composition in `tests/csv.rs` is test-only. No public combined-import,
JSON-request text parser, file IO, CLI command, WASM export or browser interface
is introduced. Existing normalization behavior and all 25 domain codes are intact.

New CSV/pipeline integration tests use independent parser expectations and
read-only synthetic fixtures via include_bytes!, including preserved multiline
source and atomic parse/domain failures. Final-head CI must prove this suite and
the unchanged normalization/P0 suites ran. QOS-0004 local tests/builds NOT RUN;
existing pinned rustfmt and lightweight text/Git checks only. No dependencies,
lockfile, toolchain or workflow changed. WASM remains compilation-only evidence;
macOS and browser/native-WASM runtime parity NOT RUN. This is not P1 acceptance.

## QOS-0005 native CLI import

`quantos-cli import --csv <path> --config <path>` reads bounded native-path inputs,
then calls the unchanged `parse_csv_v1` and, on parse success, `normalize_v1`.
There is no new public library or core combined-import API. CLI-private Serialize
reports wrap the exact core result; the complete command/config/report/exit and
precedence contract is [cli-import.md](cli-import.md). No arguments retains the
foundation status; sole --help prints static usage.

Config root is exactly schema_version/metadata/options; child values/types pass
unchanged to core. The private duplicate-rejecting JSON visitor retains default
nesting limits. Reads are capped at config 1048576 and CSV 67108864 bytes,
inclusive, using limit+1 rather than trusting metadata. All operational issues
are sanitized fixed messages; requested normalized data/provenance remain in
stdout. Reports serialize fully before stdout writes; output failures exit 3
with a fixed stderr diagnostic and may leave partial stdout across a broken pipe.

Only CLI serde 1.0.229 (derive) and serde_json 1.0.154 edges are added, reusing
already locked packages. New subprocess tests use the actual built CLI with
synthetic owned temporary files; output-write errors use an injected writer unit
test. Existing CLI smoke/core suites are unchanged. Local tests/builds/CLI
execution NOT RUN; only existing pinned rustfmt, OFFLINE lock resolution and
text/Git checks are allowed. Final-head CI must prove new import tests ran.
No real dataset validation, broker/network feature, WASM/browser import, release,
deployment or P1-wide acceptance. macOS/browser/native-WASM runtime parity NOT RUN.
