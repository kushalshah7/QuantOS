# Candle normalization contract — schema 1

Status: documentation specified by owner-approved QOS-0002. No parser or normalizer
exists. The Rust/WASM/CLI interfaces in [api-contracts.md](api-contracts.md) remain
P0 skeletons. [Golden cases](../fixtures/synthetic/normalization-cases.md) are
hand-written specifications, not executed normalizer tests.

## Scope and units

One dataset contains one exact symbol/exchange pair and only `5m` bars. Every
timestamp identifies the bar **OPEN**, not its close. Other intervals are errors.
No session calendar, resampling, adjustment calculation or financial engine is
defined here. No additional timestamp alignment/cadence rule is inferred from
`5m`; expected openings come only from the caller's explicit grid.

Input price strings are decimal rupees. Output `open/high/low/close` are integer
**paise**: input `100` becomes `10000`. Output JSON is not input CSV; reusing its
paise numbers as rupee input would be a unit error. Volume is an integer count.
All normalized integers are exact JavaScript-safe integers, with absolute value
at most `9007199254740991`. Timestamps have the stricter range below.

## Typed document interfaces

These TypeScript-shaped definitions describe values only. They do not add a
TypeScript or Rust implementation, callable API, serialization library or browser
worker protocol. `number` below means an exact integer satisfying its constraints.
All object properties are required; unknown properties are errors. No defaults,
implicit coercion or environment-derived values apply.

```ts
type SafeInteger = number; // integer, abs(value) <= 9007199254740991
type UtcMs = SafeInteger; // 0 <= value < 4102444800000
type PositivePaise = SafeInteger; // 1 <= value <= 9007199254740991
type Volume = SafeInteger; // 0 <= value <= 9007199254740991
type ConfirmedTimezone = "UTC" | "Asia/Kolkata";
type ProvenanceV1 = {
  source: string; // nonempty after whitespace-only check; preserve exact spelling
  synthetic: boolean;
  adjustment_status:
    "unknown" | "unadjusted" | "split_adjusted" | "total_return_adjusted";
  redistribution_status: "unknown" | "private_only" | "permitted";
};
type NormalizationOptionsV1 = {
  timestamp_column: string; // exact header name; canonical name is ts_utc_ms
  timestamp_format: "utc_ms" | "iso8601";
  confirmed_timezone: ConfirmedTimezone | null;
  expected_timestamps_utc_ms: UtcMs[] | null;
};
type NormalizationInputV1 = {
  schema_version: "1";
  headers: string[]; // parsed CSV header, row 1
  rows: string[][]; // raw field strings, first data record is CSV row 2
  metadata: ProvenanceV1;
  options: NormalizationOptionsV1;
};
type NormalizedCandleV1 = {
  symbol: string;
  exchange: string;
  interval: "5m";
  ts_utc_ms: UtcMs;
  open: PositivePaise;
  high: PositivePaise;
  low: PositivePaise;
  close: PositivePaise;
  volume: Volume;
  source: string; // equals output metadata.source
};
type IssueV1 = {
  code: IssueCodeV1; // closed catalog below
  severity: "error" | "warning";
  row: SafeInteger | null; // when non-null, >= 1; 1 = header, 2 = first record
  field: string | null; // exact CSV name, qualified document path, or null
  message: string; // nonempty descriptive reason; not an assertion key
};
type GapAssessmentV1 =
  | { status: "not_performed"; missing_ts_utc_ms: [] }
  | { status: "performed"; missing_ts_utc_ms: UtcMs[] };
type NormalizationResultV1 = {
  schema_version: "1"; // also used to report an unsupported input version
  metadata: ProvenanceV1 | null; // exact declarations if valid; otherwise null
  issues: IssueV1[];
  gap_assessment: GapAssessmentV1;
} & (
  | { status: "success"; candles: NormalizedCandleV1[] }
  | { status: "failure"; candles: [] }
);
```

Input is an already parsed CSV table; CSV quoting/byte decoding is outside this
task. A quoted multiline field does not change record numbering: header is 1 and
data record index `i` is row `i + 2`. The later parser must retain this numbering.
CSV field strings are not trimmed or case-folded. For nonempty source/symbol/
exchange checks, whitespace means ASCII space, tab, CR and LF; a string containing
only those characters is empty. Otherwise preserve it exactly, including spaces.
Document property/type violations are reported, not coerced into the typed model.

Each candle's source plus result metadata supplies its provenance. Preserve all
four metadata declarations even on a failure if the metadata object is valid.
`synthetic: false` and any adjustment/redistribution label are declarations, not
evidence of actual market origin, adjustment accuracy or redistribution rights.
Invalid metadata returns `metadata: null`; do not fabricate a replacement.

## Headers, mapping and identity

Canonical input headers are exactly:

```text
symbol,exchange,interval,ts_utc_ms,open,high,low,close,volume,source
```

Header order may vary; names are exact and case-sensitive. There must be exactly
one of each required name, with no extras or duplicates. Only the timestamp name
may be replaced: explicitly set `options.timestamp_column` to the replacement.
That name must be nonempty and distinct from the other nine required names.
For canonical input explicitly set it to `ts_utc_ms`. A replacement and the old
timestamp column cannot coexist. No alias detection or other column mapping.
`timestamp_format` always selects the decoder, independently of the column name;
neither a header name nor a cell's appearance selects units or format.

Every row must match header width. Symbol and exchange must be nonempty and match
the first nonempty value in their respective columns exactly; compare them even
if another field in either row fails. `interval` must equal `5m`. Every row's
`source` must equal valid `metadata.source` exactly. Zero data rows is an error.

## Time parsing and timezone conflicts

- `utc_ms`: plain integer text matching `-?[0-9]+`; leading zeros are allowed.
  No decimal point, exponent, plus sign, spaces or unit inference. Its unit and
  timezone are already UTC milliseconds. `confirmed_timezone` may be null or UTC;
  Asia/Kolkata conflicts and produces one dataset-level `TIMEZONE_CONFLICT`.
- `iso8601`: accepted subset is `YYYY-MM-DDTHH:mm:ss[.fraction][zone]` with uppercase
  `T`, zone `Z` or `+HH:mm`/`-HH:mm`, or absent zone. Year is four digits, dates
  follow the proleptic Gregorian calendar, hours are 00–23, minutes/seconds 00–59.
  Offset hours are 00–23 and offset minutes 00–59. Leap seconds, 24:00, date-only
  strings, spaces and malformed dates are errors. A fraction has 1–3 digits,
  padded on the right to milliseconds. More digits are rejected, never rounded.
- A zone-free ISO value requires caller-confirmed UTC or Asia/Kolkata; null
  produces `TIMEZONE_REQUIRED`. UTC contributes offset zero; Asia/Kolkata uses
  +05:30 throughout the supported normalized range. Never read the host timezone.
- A zoned ISO value needs no confirmation. If a confirmation is supplied, its
  offset must equal the explicit offset: 00:00 for UTC, +05:30 for Asia/Kolkata.
  Otherwise issue `TIMEZONE_CONFLICT` at that timestamp cell. `-00:00` denotes
  zero offset in this accepted ISO subset, like `+00:00` and `Z`.
- Subtract the declared/confirmed offset to derive UTC, then enforce
  `0 <= ts_utc_ms < 4102444800000` (1970-01-01 inclusive to 2100-01-01 exclusive).
  Bounds apply to UTC, so a local date may cross the UTC date boundary.

For a timestamp cell: invalid syntax/date precedes missing/conflicting timezone,
which precedes UTC range validation; emit only the first applicable cell error.
The utc_ms options conflict is independent of cell parsing/range checks. Its
integer cells still have known UTC meaning and can be checked for ordering.
An ISO timezone error leaves that cell without a validated UTC value.

Reject every later repeated validated timestamp (`DUPLICATE_TIMESTAMP`), even if
nonadjacent. Also compare each validated timestamp with the nearest earlier
validated timestamp in input order; a smaller value is `DESCENDING_TIMESTAMP`.
Equal values are duplicate-only unless a separate descending comparison applies.
Malformed time cells do not hide ordering errors between surrounding valid times.
Other invalid fields do not disable these time checks. Never sort or deduplicate.

## Exact numeric parsing and OHLC

Prices use plain decimal text matching `-?[0-9]+(\.[0-9]+)?`; leading zeros are
allowed. No exponent, plus sign, whitespace, missing whole/fractional part,
NaN or infinity. Strip trailing fractional zeros for the precision check only.
At most two fractional digits may then remain; e.g. `100.0100` -> `10001` paise,
`0.0100` -> `1`, while `100.00100` is `PRICE_PRECISION`. Multiply the exact decimal
by 100, with no floating-point intermediate or rounding. The result must be
between `1` and `9007199254740991`, inclusive. Thus `90071992547409.91` is the
largest price; `90071992547409.92`, zero and negative prices are out of range.

Volume uses integer text matching `-?[0-9]+`; its exact value must be in
`[0,9007199254740991]`. `1.0`, exponents, spaces and nonfinite text are malformed.
Leading zeros and mathematical negative zero are allowed and normalize to their
integer values; they do not retain a textual sign in output.

Emit one primary issue per numeric cell: malformed/nonfinite first, then price
precision (prices only), then range. Compare OHLC only when all four prices have
validated integer values. Require `low <= open <= high` and `low <= close <= high`;
one `INVALID_OHLC` covers all failed inequalities for that row. Invalid volume
does not disable OHLC checking. Display formatting never changes these values.

## Expected grid, atomicity and validation dependencies

An explicit grid is an array of unique, strictly increasing UtcMs integers.
An empty grid is valid. It is a caller-supplied list, not a calendar: no implicit
alignment, grid expansion or exchange-session inference. Reject any invalid
grid with one `INVALID_EXPECTED_GRID`; independently check usable row fields.

If any error exists, return failure, `candles: []`, and gap assessment
`not_performed` with an empty missing list. Do not assess gaps against a partially
valid dataset or add gap warnings to a failed result. Collect all independently
detectable errors, not merely the first row's errors. This is atomic acceptance,
not a drop-bad-rows mode.

With no errors and a null grid, return success and `not_performed`. With a valid
explicit grid, return success and `performed`; missing timestamps are the sorted
set difference of grid minus observed timestamps. Emit one warning per missing
timestamp. Observations outside the grid are retained; the grid defines expected
presence only. Warnings never fabricate candles or turn success into failure.

Validation dependencies are explicit: validate available request/options/metadata
properties independently; one bad property does not suppress unrelated errors.
An invalid timestamp format suppresses time decoding, not price/identity checks.
Unavailable/invalid timestamp mapping or headers suppress mapped cell validation;
row shape/width and independent metadata/options/grid checks still run where
their inputs exist. A non-array or wrong-width row cannot supply mapped cells.
In a correctly sized row, a non-string cell is unavailable but siblings remain
usable. An invalid confirmation option does not become an absent confirmation:
skip conversion of naive ISO cells, while explicit UTC-ms/zoned ISO values can
still be parsed without inventing a timezone conflict. Invalid metadata.source
suppresses source matching only; other metadata errors do not. Do not manufacture
downstream OHLC, timezone, sequence or source errors from an unavailable value.

## Issue catalog and deterministic ordering

`IssueCodeV1` is exactly the ordered list below. All codes have severity `error`
except `MISSING_EXPECTED_BAR`, whose severity is `warning`. Each message must
describe the violated rule; wording is not versioned or compared as a test key.
Do not echo private row contents into messages or send them to external logs.

| Rank / code | Rule and location |
| --- | --- |
| 1 `INVALID_REQUEST` | Non-object root: row null, field null. Missing/wrong-typed/unknown root property: row null, field property path. Wrong-shaped data record: its CSV row, field null; non-string cell: its row and header name if known, otherwise null. |
| 2 `UNSUPPORTED_SCHEMA_VERSION` | A string input version other than `1`: row null, field `schema_version`. |
| 3 `INVALID_OPTIONS` | Missing/wrong-typed/unknown option or invalid timestamp mapping: row null, field `options.<name>`. |
| 4 `INVALID_METADATA` | Missing/wrong-typed/unknown metadata field, empty source or invalid enum: row null, field `metadata.<name>`. |
| 5 `INVALID_EXPECTED_GRID` | Non-null grid is not a valid grid: row null, field `options.expected_timestamps_utc_ms`. |
| 6 `EMPTY_DATASET` | No data records: row null, field `rows`. |
| 7 `INVALID_HEADER` | Missing, duplicate or extra header name: one aggregate issue at row 1, field null. Message describes all defects in ordinal name order. |
| 8 `ROW_WIDTH` | String-array record has the wrong number of fields: its row, field null. |
| 9 `INVALID_IDENTITY` | Empty symbol/exchange: its row, corresponding CSV field. |
| 10 `MIXED_DATASET` | Symbol/exchange differs from its column's first nonempty value: its row, corresponding field. |
| 11 `UNSUPPORTED_INTERVAL` | Interval is not exactly `5m`: its row, field `interval`. |
| 12 `SOURCE_MISMATCH` | Row source differs from valid declared source: its row, field `source`. |
| 13 `INVALID_TIMESTAMP` | Malformed integer/ISO or invalid date/time/precision: its row, mapped timestamp field. |
| 14 `TIMEZONE_REQUIRED` | Naive ISO lacks confirmation: its row, mapped timestamp field. |
| 15 `TIMEZONE_CONFLICT` | UTC-ms/Kolkata options conflict: row null, field `options.confirmed_timezone`; ISO offset conflict: its row, mapped timestamp field. |
| 16 `TIMESTAMP_OUT_OF_RANGE` | Valid UTC value is outside the supported range: its row, mapped timestamp field. |
| 17 `DUPLICATE_TIMESTAMP` | A validated time repeats any earlier validated time: later row, mapped timestamp field. |
| 18 `DESCENDING_TIMESTAMP` | Validated time is smaller than nearest earlier validated time: current row, mapped timestamp field. |
| 19 `INVALID_PRICE` | Malformed/nonfinite decimal price: its row, price field. |
| 20 `PRICE_PRECISION` | More than two meaningful fractional digits: its row, price field. |
| 21 `PRICE_OUT_OF_RANGE` | Exact paise value is not positive and safe: its row, price field. |
| 22 `INVALID_VOLUME` | Malformed/nonfinite integer volume: its row, field `volume`. |
| 23 `VOLUME_OUT_OF_RANGE` | Negative or unsafe integer volume: its row, field `volume`. |
| 24 `INVALID_OHLC` | Validated prices violate OHLC inequalities: its row, field null. |
| 25 `MISSING_EXPECTED_BAR` | One missing expected opening: row null, field null. Message identifies its timestamp; the structured missing list is authoritative. |

For a missing or wrong-typed options/metadata container, use one `INVALID_REQUEST`
at `options`/`metadata`; do not invent its child issues. Within a present object,
emit one property issue per invalid/missing/unknown property. Grid validation uses
`INVALID_EXPECTED_GRID` even for wrong grid type; a missing grid property is
`INVALID_OPTIONS`. An invalid version does not suppress other available checks.
A non-object root yields only `INVALID_REQUEST` at row/field null, with output
metadata null, no candles and gap assessment not performed.

Order issues without locale, hash iteration, message text or concurrency effects:
1. Dataset issues (row null, excluding gap warnings): catalog rank, then field
   by Unicode code-point order, with null before strings.
2. Header and data issues: ascending CSV row; within each row use logical field
   order `symbol, exchange, interval, timestamp, open, high, low, close, volume,
   source, null`. The timestamp key uses its mapped name. Break ties by catalog
   rank. Unmapped fields, if any, follow known fields in Unicode code-point order
   before null. Keep one issue per applicable code/row/field; aggregate duplicate
   descriptions rather than create indistinguishable errors.
3. Gap warnings: ascending missing UTC timestamp, matching the missing list.

Successful candles retain input order and exact metadata declarations. On success
there are no error-severity issues; on failure there is at least one and no candles.
A fixed implementation/version must return identical values and messages for
identical input/options, independently of host timezone. Acceptance assertions
compare structure and ordered issue keys rather than descriptive message wording;
wording may improve between implementations/versions. No native/WASM parity claim
exists until implementation and tests.

## Safety and verification limits

Public examples are invented and explicitly synthetic. Private inputs stay under
ignored `data/`; no broker credentials, licensed data or local backtest results
belong in Git/CI logs/artifacts. Provenance does not grant redistribution rights.
Existing P0 tests only check their fixture examples and skeleton interfaces.
CI regression success does not validate this document's arithmetic, execute these
normalization rules or prove that an importer/normalizer exists. Runtime behavior,
dependencies and public bindings require another approved task.
