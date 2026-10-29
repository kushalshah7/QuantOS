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

The [P1 acceptance packet](p1-acceptance.md) records current implementation-base
CI evidence, later Mac validation and the owner's P1 acceptance. QOS-0002 specified the
unchanged contract/goldens; QOS-0003 implemented normalization, QOS-0004 CSV and
QOS-0005 native CLI import. The owner accepted aggregate P1 on 2026-10-29;
the packet records reviewed head da33ea0 and protected PR #7 merge 21ac348.

`tests/normalization.rs` exercises all golden IDs/alternatives, structural and
numeric/time boundary cases, deterministic issue serialization/messages, and
isolated TZ=UTC/Asia/Kolkata subprocesses. Implementation-base Ubuntu CI executed
the 20 normalization tests successfully; see the packet for actual output. Rust
assertions transcribe hand-written goldens; CI does not execute Markdown or
independently validate its arithmetic. Original implementation-local runtime checks
were NOT RUN; the later [Mac receipt](p1-macos-validation.md) records the passing
personal-machine checklist and full Rust suite. WASM target evidence is compilation
only; browser/native-WASM normalization-result parity remains NOT RUN.

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
the composition in `tests/csv.rs` is test-only. QOS-0004 added no public combined-import,
JSON-request text parser, file IO, CLI command, WASM export or browser interface
of its own. Existing normalization behavior and all 25 domain codes are intact.

New CSV/pipeline integration tests use independent parser expectations and
read-only synthetic fixtures via include_bytes!, including preserved multiline
source and atomic parse/domain failures. Implementation-base Ubuntu CI executed
30 CSV/pipeline tests plus the unchanged normalization/P0 suites successfully;
the acceptance packet links the actual output. QOS-0004 local tests/builds were
NOT RUN; historical formatter/text-check evidence remains in PROGRESS.md.
No dependencies, lockfile, toolchain or workflow changed in QOS-0004. WASM remains
compilation-only evidence; browser/native-WASM runtime parity remains NOT RUN.
Later Mac validation passed and the owner accepted aggregate P1; see the packet.

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
test. Existing CLI smoke/core suites are unchanged. Implementation-base Ubuntu
CI executed 11 import tests and 2 output-writer unit tests successfully; see the
acceptance packet for actual output. Original QOS-0005 local tests/builds/CLI were
NOT RUN; QOS-0006 performed text/Git checks only. Later QOS-0007 Mac validation
passed, including 12 amended native-path import tests. Historical QOS-0005 formatter/
offline lock-resolution evidence remains in PROGRESS.md.
QOS-0005 added no real dataset validation, broker/network feature, WASM/browser
import, release or deployment and did not itself grant P1 acceptance. The owner
later accepted P1 through QOS-0007. Browser/native-WASM runtime parity remains NOT RUN.

## QOS-0009 Rust-only provided-signal execution API

```rust
pub fn execute_v1(
    input: &quantos_core::execution::ExecutionInputV1,
) -> quantos_core::execution::ExecutionResultV1;
```

The public path is `quantos_core::execution::execute_v1`; exact Serialize-only
types, field names, enum/error strings and deterministic ordering are locked in
[execution-api-v1.md](execution-api-v1.md). Input owns unchanged P1 normalized
candles/provenance, explicit complete-bar/config declarations and a typed close
decision for each bar. Revalidation precedes execution; no defaults, data repair,
JSON decoder, file IO or environment access is introduced.

The accepted [D01–D12 model](backtest-execution-v1-draft.md) defines next-OPEN
orders, checked integer paise, explicit costs, previous-volume shared capacity,
protective precedence and final CLOSE marking. Success contains ordered fills,
audit events, reconciled close snapshots, exact input declarations and summary.
Failure contains only version/status/error, even after earlier internal fills.

QOS-0009 is under review in [PR #9](https://github.com/kushalshah7/QuantOS/pull/9).
Mac evidence: 39 execution tests (all E01–E13 plus boundary/causality cases),
107 total workspace Rust tests, fmt, Clippy and WASM target compilation pass.
The P1 parser/normalizer integration test consumes the existing synthetic fixture
without changing P1. No new dependency, CLI command, WASM export or UI surface.
WASM evidence is compilation only; browser/native-WASM financial parity NOT RUN.
