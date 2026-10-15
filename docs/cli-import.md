# Native CLI import — schema 1

Owner-approved QOS-0005 interface. The CLI reads files, calls the existing core
CSV parser/normalizer and reports JSON. No new financial math, core combined API,
browser integration or complete-P1 acceptance. Input files are never modified.

## Commands and native paths

```text
quantos-cli
quantos-cli --help
quantos-cli import --csv <path> --config <path>
```

No args preserves the foundation status plus newline, exit 0 and empty stderr.
Sole --help prints static plain usage with exit 0/empty stderr. Import requires
both flags exactly once, either order. No positional extras, flag=value aliases,
defaults, stdin or extra flags. Unknown command/flag, duplicate/missing flag,
missing value or path '-' -> USAGE before any file access. Use args_os/native
paths, including non-UTF-8 paths; relative paths use the process working directory.
No telemetry, network, persistence, guessing or interactive prompts.

## Configuration and precedence

Config is UTF-8 JSON without BOM. It must be an object with exactly the three
required root keys schema_version, metadata and options; bad envelope/root keys
produce CONFIG_ENVELOPE. Children retain their exact values/types, without CLI
defaults/coercion or duplicate domain validation. Core owns their validation.

Invalid UTF-8/JSON, trailing non-whitespace or duplicate keys at ANY object depth
produce CONFIG_JSON. Retain serde_json's default nesting limit; use a small
CLI-private duplicate-rejecting deserialization helper. Never accept last-key-wins.

Precedence is arguments -> config read/size -> config JSON -> envelope -> CSV
read/size -> parse_csv_v1 -> normalize_v1 on parser success only. Build the core
request from the three exact config values plus parsed headers/rows. Invalid
children become ordered core issues, even if wrong-typed or missing nested fields.

Bounded reads allow config up to 1,048,576 bytes and CSV up to 67,108,864 bytes,
inclusive. Read at most limit+1, even with absent/stale metadata. Exceeding the
limit produces the matching TOO_LARGE code, never silent truncation. Reads only;
no output files or input modifications.

## Exact reports and exits

Except no-args/help, normal output is one compact UTF-8 JSON object plus one
newline to stdout, with empty stderr. CLI-private Serialize types produce these
exact shapes; all shown properties required, no extra wrappers/partial candles.

Normalization completion (outer status equals nested status, all values forwarded):

```json
{"schema_version":"1","status":"success","stage":"normalization","result":{"schema_version":"1","status":"success","metadata":{},"issues":[],"gap_assessment":{"status":"not_performed","missing_ts_utc_ms":[]},"candles":[]}}
```

The nested value is the exact NormalizationResultV1, including failure results;
the object above illustrates the outer shape, not valid example domain metadata.
CSV failure does not invoke normalization:

```json
{"schema_version":"1","status":"failure","stage":"csv","result":{"schema_version":"1","status":"failure","issue":{"code":"EMPTY_INPUT","row":null,"byte_offset":0,"message":"CSV input must contain a record."}}}
```

CLI/config/IO failure has no result or partial candles:

```json
{"schema_version":"1","status":"failure","stage":"cli","issue":{"code":"USAGE","field":"args","message":"Use import --csv <path> --config <path>."}}
```

| Code/result | Field | Exit |
| --- | --- | --- |
| USAGE | args | 2 |
| CONFIG_JSON, CONFIG_ENVELOPE, CONFIG_TOO_LARGE | config | 2 |
| CONFIG_READ | config | 3 |
| CSV_READ, CSV_TOO_LARGE | csv | 3 |
| CSV parser failure or normalization failure | nested core result | 1 |
| Normalization success, including gap warnings | nested core result | 0 |

These seven CLI codes are closed and separate from core parser/domain codes.
Operational messages are fixed descriptive text: no paths, config/CSV contents or
raw OS/JSON parser errors. Requested normalized candles/provenance ARE written to
stdout, so the report contains the user's data. Do not log real reports to GitHub.

Serialize the full report before writing. On serialization/write/flush failure,
emit only fixed `quantos-cli: output failure` plus newline to stderr and exit 3;
do not panic, retry or claim success. A broken pipe can leave partial stdout;
there is no promise of atomicity across output failure.

## Synthetic configuration and personal-machine example

`fixtures/synthetic/import-config.json` contains exactly:

```json
{
  "schema_version": "1",
  "metadata": {
    "source": "synthetic",
    "synthetic": true,
    "adjustment_status": "unknown",
    "redistribution_status": "unknown"
  },
  "options": {
    "timestamp_column": "ts_utc_ms",
    "timestamp_format": "utc_ms",
    "confirmed_timezone": null,
    "expected_timestamps_utc_ms": null
  }
}
```

For later use on a personal machine; NOT executed locally in this task:

```sh
cargo run -p quantos-cli --locked -- import --csv fixtures/synthetic/valid-candles.csv --config fixtures/synthetic/import-config.json
```

The input fixtures/config are invented; no real historical dataset is needed.
New CI integration tests spawn the actual built CLI with unique std-only owned
temporary directories. They cover exact reports/exits/stderr, full values and
ordered issue keys, malformed config/arguments/CSV, duplicates, size limits,
precedence, data-bearing stdout, deterministic messages, unchanged input bytes and
injected output failures. Existing no-argument smoke and core suites remain intact.

Local tests/builds/CLI execution NOT RUN under LOW-RESOURCE. Only existing pinned
rustfmt, text/Git checks and OFFLINE lock resolution are permitted. Final-head
existing CI must prove new import tests and unchanged core/P0 tests ran. WASM is
compilation-only evidence; macOS/browser/native-WASM runtime parity NOT RUN.
