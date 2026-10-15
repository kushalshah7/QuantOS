# Native and WASM boundaries

The existing P0 build-stage interfaces remain unchanged:

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

## QOS-0003 Rust-only normalization API

```rust
pub fn quantos_core::data::normalize_v1(
    input: &serde_json::Value,
) -> quantos_core::data::NormalizationResultV1;
```

This is the sole new callable domain entry point. Its input is an in-memory JSON
value containing an already-parsed CSV table, explicit options and provenance.
It reports malformed/missing/unknown properties without strict deserialization
discarding independently detectable errors. No CSV/JSON-text parsing, file IO,
network access, host-timezone lookup, worker protocol or cancellation API is added.

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
