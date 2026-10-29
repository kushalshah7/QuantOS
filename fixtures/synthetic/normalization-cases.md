# Schema-1 normalization golden specification

All values below are invented. `SYNTH`/`TEST` are fictional and all examples,
including deliberately false provenance declarations, are synthetic test designs.
They contain no market observations. These QOS-0002 hand-written expected results
specify [the data contract](../../docs/data-contract.md); the Markdown is not an
executable test. QOS-0003 implemented normalization, QOS-0004 added CSV parsing,
and QOS-0005 exposes the native CLI. Rust tests transcribe these expectations;
see the [P1 acceptance packet](../../docs/p1-acceptance.md) for executed CI evidence.

## Exact base input and notation

`R` is the following complete parsed input. All CSV cells are strings; options
are explicit. Row 1 is the header; the three records are CSV rows 2, 3 and 4.

```json
{
  "schema_version": "1",
  "headers": ["symbol","exchange","interval","ts_utc_ms","open","high","low","close","volume","source"],
  "rows": [
    ["SYNTH","TEST","5m","0","100","103","99","102","10","synthetic"],
    ["SYNTH","TEST","5m","300000","102","104","101","103","12","synthetic"],
    ["SYNTH","TEST","5m","600000","103","105","100","101","8","synthetic"]
  ],
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

- `M` is R's exact metadata. `Q` is R retaining only its first data record.
- `T(value, confirmation)` is Q with the header `ts_utc_ms` renamed `timestamp`,
  `timestamp_column: "timestamp"`, `timestamp_format: "iso8601"`, the specified
  `confirmed_timezone`, and its timestamp cell set to the exact string `value`.
- An input change such as `row 2 open = "100.0100"` changes only that raw cell.
  Unmentioned fields/options remain exactly as in the named base input.
- `C0`, `C1`, `C2` are the three exact normalized objects below. `C0{open:10001}`
  means copy C0 with only that named property replaced; this is document notation.

```json
[
  {"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":0,"open":10000,"high":10300,"low":9900,"close":10200,"volume":10,"source":"synthetic"},
  {"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":300000,"open":10200,"high":10400,"low":10100,"close":10300,"volume":12,"source":"synthetic"},
  {"symbol":"SYNTH","exchange":"TEST","interval":"5m","ts_utc_ms":600000,"open":10300,"high":10500,"low":10000,"close":10100,"volume":8,"source":"synthetic"}
]
```

Expected issue notation `E(CODE,row,field)` means exactly these assertion keys:
`{code:CODE,severity:"error",row:row,field:field}`. `W` is the same with severity
`"warning"`. A conforming IssueV1 also has a nonempty descriptive `message`;
messages are deliberately not assertion keys. For example an INVALID_OHLC message
may say `Open and close must lie between low and high.` Do not compare that text.

`S(candles)` expands to this exact result, with its argument substituted:

```text
schema_version: "1"
status: "success"
metadata: M
candles: <argument>
issues: []
gap_assessment: {status:"not_performed",missing_ts_utc_ms:[]}
```

`F(issues)` expands to this exact result, with its ordered issue list substituted:

```text
schema_version: "1"
status: "failure"
metadata: M
candles: []
issues: <argument>
gap_assessment: {status:"not_performed",missing_ts_utc_ms:[]}
```

An explicitly stated metadata/gap/issues replacement overrides only that output
property. Each alternative in an `each of` list is a separate case with the same
stated expectation. These conventions specify every result field without relying
on an implementer's defaults, partial acceptance or message wording.

## Canonical, UTC and timezone cases

| ID | Exact input | Exact expected result |
| --- | --- | --- |
| N01 | R, unchanged; matches existing `valid-candles.csv` plus explicit options/M. | S([C0,C1,C2]). |
| N02 | Q with `confirmed_timezone: "UTC"`. | S([C0]); explicit UTC is consistent with UTC-ms. |
| T01 | T("1970-01-01T03:45:00Z", null). | S([C0{ts_utc_ms:13500000}]). |
| T02 | T("1970-01-01T09:15:00+05:30", null). | S([C0{ts_utc_ms:13500000}]). |
| T03 | T("1970-01-01T09:15:00", "Asia/Kolkata"). | S([C0{ts_utc_ms:13500000}]). |
| T04 | T("1970-01-01T03:45:00", "UTC"). | S([C0{ts_utc_ms:13500000}]). |
| T05 | T("1970-01-01T09:15:00", null), equivalent to the P0 missing-timezone example with its mapping supplied. | F([E(TIMEZONE_REQUIRED,2,"timestamp")]); no assumed zone or candle. |
| T06 | T("1970-01-02T00:15:00+05:30", "Asia/Kolkata"). | S([C0{ts_utc_ms:67500000}]); UTC is 1970-01-01 18:45:00. |
| T07 | T("1970-01-01T03:45:00Z", "Asia/Kolkata"). | F([E(TIMEZONE_CONFLICT,2,"timestamp")]). |
| T08 | T("1970-01-01T09:15:00+05:30", "UTC"). | F([E(TIMEZONE_CONFLICT,2,"timestamp")]). |
| T09 | Q with `confirmed_timezone: "Asia/Kolkata"`. | F([E(TIMEZONE_CONFLICT,null,"options.confirmed_timezone")]). |
| T10 | T("1970-01-01T00:00:00.1Z", null). | S([C0{ts_utc_ms:100}]); fraction is exactly 100 ms. |
| T11 | Each of T("1970-02-30T00:00:00Z",null), T("1970-01-01T24:00:00Z",null), T("1970-01-01T00:00:60Z",null), T("1970-01-01T00:00:00.0000Z",null), T("1970-01-01",null). | F([E(INVALID_TIMESTAMP,2,"timestamp")]). |
| T12 | Each of T("1970-01-01T00:00:00+00:00","UTC"), T("1970-01-01T00:00:00-00:00",null). | S([C0]). |
| T13 | Q with raw timestamp "1970-01-01T00:00:00Z", keeping `timestamp_format: "utc_ms"`. | F([E(INVALID_TIMESTAMP,2,"ts_utc_ms")]); no format guessing. |

Arithmetic: `(9*60+15 - 330)*60000 = 13500000`;
`(24*60+15 - 330)*60000 = 67500000`. Equivalence assertions compare timestamps,
not local date strings. No machine timezone participates.

## Prices, volume and boundaries

| ID | Exact input | Exact expected result |
| --- | --- | --- |
| P01 | Q, row 2 open = "100.0100". | S([C0{open:10001}]). |
| P02 | Q, row 2 open = "000100.0000". | S([C0]); trailing fractional zeros/leading whole zeros do not change value. |
| P03 | Q, row 2 low = "0.0100". | S([C0{low:1}]); one paise is allowed. |
| P04 | Each of Q with row 2 open = "100.001", or "100.00100". | F([E(PRICE_PRECISION,2,"open")]); neither rounds to 10000 paise. |
| P05 | Each of Q with row 2 open = "0", "-1", or "-0.00". | F([E(PRICE_OUT_OF_RANGE,2,"open")]). |
| P06 | Each of Q with row 2 open = "NaN", "Infinity", "-Infinity", "1e2", "+100", "100.", ".50", or " 100". | F([E(INVALID_PRICE,2,"open")]). |
| V01 | Each of Q with row 2 volume = "0", "000", or "-0". | S([C0{volume:0}]). |
| V02 | Q, row 2 volume = "-1". | F([E(VOLUME_OUT_OF_RANGE,2,"volume")]). |
| V03 | Each of Q with row 2 volume = "1.0", "1e1", "NaN", "Infinity", or " 10". | F([E(INVALID_VOLUME,2,"volume")]). |
| B01 | Q, timestamp = "4102444799999". | S([C0{ts_utc_ms:4102444799999}]); upper inclusive representable millisecond. |
| B02 | Each of Q with timestamp = "-1" or "4102444800000". | F([E(TIMESTAMP_OUT_OF_RANGE,2,"ts_utc_ms")]). |
| B03 | Q, timestamp = "0.0". | F([E(INVALID_TIMESTAMP,2,"ts_utc_ms")]). |
| B04 | Each of T("1969-12-31T23:59:59.999Z",null), T("2100-01-01T00:00:00Z",null). | F([E(TIMESTAMP_OUT_OF_RANGE,2,"timestamp")]). |
| B05 | Q, all four raw prices = "90071992547409.91", volume = "9007199254740991". | S([C0{open:9007199254740991,high:9007199254740991,low:9007199254740991,close:9007199254740991,volume:9007199254740991}]). |
| B06 | Q, timestamp = "4102444800000", open = "90071992547409.92", volume = "9007199254740992". | F([E(TIMESTAMP_OUT_OF_RANGE,2,"ts_utc_ms"), E(PRICE_OUT_OF_RANGE,2,"open"), E(VOLUME_OUT_OF_RANGE,2,"volume")]). |

Price arithmetic: `100.01*100 = 10001`; `0.01*100 = 1`;
`90071992547409.91*100 = 9007199254740991`, exactly. No floating-point
approximation or rounding tolerance applies. `100.001` cannot become integer
paise without changing its value and is rejected.

## OHLC, identity, ordering and atomic failure

| ID | Exact input | Exact expected result |
| --- | --- | --- |
| D01 | Q, row 2 high = "98" (same prices as `invalid-ohlc.csv`). | F([E(INVALID_OHLC,2,null)]). |
| D02 | R, row 3 timestamp = "0" (first pair matches `duplicate-timestamp.csv`). | F([E(DUPLICATE_TIMESTAMP,3,"ts_utc_ms")]); no row is silently dropped. |
| D03 | R, row 3 timestamp = "600000", row 4 timestamp = "300000". | F([E(DESCENDING_TIMESTAMP,4,"ts_utc_ms")]); no sort. |
| D04 | R, row 4 timestamp = "0". | F([E(DUPLICATE_TIMESTAMP,4,"ts_utc_ms"), E(DESCENDING_TIMESTAMP,4,"ts_utc_ms")]); both independent conditions apply. |
| D05 | R, row 3 symbol = "OTHER", row 4 exchange = "OTHER". | F([E(MIXED_DATASET,3,"symbol"), E(MIXED_DATASET,4,"exchange")]). |
| D06 | Q, row 2 symbol = " ", exchange = "". | F([E(INVALID_IDENTITY,2,"symbol"), E(INVALID_IDENTITY,2,"exchange")]). |
| D07 | Q, row 2 interval = "1m". | F([E(UNSUPPORTED_INTERVAL,2,"interval")]). |
| D08 | Q, row 2 source = "other". | F([E(SOURCE_MISMATCH,2,"source")]). |
| A01 | R, row 2 open = "NaN" and volume = "-1"; row 3 timestamp = "0" and high = "Infinity"; row 4 unchanged. | F([E(INVALID_PRICE,2,"open"), E(VOLUME_OUT_OF_RANGE,2,"volume"), E(DUPLICATE_TIMESTAMP,3,"ts_utc_ms"), E(INVALID_PRICE,3,"high")]); even the valid final row is not returned. |
| A02 | Q, high = "98", volume = "-1". | F([E(VOLUME_OUT_OF_RANGE,2,"volume"), E(INVALID_OHLC,2,null)]); bad volume does not suppress OHLC comparison. |
| A03 | Q, metadata.adjustment_status = "bad", grid = [0,0], open = "100.001". | F([E(INVALID_METADATA,null,"metadata.adjustment_status"), E(INVALID_EXPECTED_GRID,null,"options.expected_timestamps_utc_ms"), E(PRICE_PRECISION,2,"open")]), replacing output metadata with null. |

All F results above have exactly zero candles. Independently detectable fields
and later rows are inspected; dependent OHLC errors are not invented when a price
cannot be validated. For A01, no OHLC issue is applicable to rows 2 or 3.

## Explicit-grid gaps

| ID | Exact input | Exact expected result |
| --- | --- | --- |
| G01 | R with its middle data record removed; grid remains null. | S([C0,C2]); assessment not performed, no gap warning. |
| G02 | Same observed records as G01, grid = [0,300000,600000]. | S([C0,C2]), replacing gap_assessment with `{status:"performed",missing_ts_utc_ms:[300000]}` and issues with [W(MISSING_EXPECTED_BAR,null,null)]. |
| G03 | Q, grid = [0,300000,600000]. | S([C0]), gap_assessment `{status:"performed",missing_ts_utc_ms:[300000,600000]}`, issues [W(MISSING_EXPECTED_BAR,null,null), W(MISSING_EXPECTED_BAR,null,null)] in that missing-time order. |
| G04 | Q, grid = []. | S([C0]), gap_assessment `{status:"performed",missing_ts_utc_ms:[]}`; observation outside the empty grid is retained. |
| G05 | Each of Q with grid = [0,0], [300000,0], [-1], [4102444800000], [0.5], or the string "0". | F([E(INVALID_EXPECTED_GRID,null,"options.expected_timestamps_utc_ms")]). |
| G06 | Q, grid = [0,300000], open = "NaN". | F([E(INVALID_PRICE,2,"open")]); assessment not performed and no gap warning on failure. |

The grid is an explicit expected list. These cases say nothing about real NSE
sessions or missing market observations. The two G03 warnings correspond to the
two sorted timestamps; message text is not used to order or assert them.

## Metadata, schema and structural errors

| ID | Exact input | Exact expected result |
| --- | --- | --- |
| S01 | Q, input schema_version = "2". | F([E(UNSUPPORTED_SCHEMA_VERSION,null,"schema_version")]); output schema_version remains "1". |
| S02 | Q, metadata.source = "". | F([E(INVALID_METADATA,null,"metadata.source")]), replacing output metadata with null; no derived SOURCE_MISMATCH. |
| S03 | Q, metadata = {"source":"synthetic","synthetic":false,"adjustment_status":"split_adjusted","redistribution_status":"private_only"}. | S([C0]), replacing output metadata with exactly that supplied metadata object. These are deliberately supplied declarations in an invented case, not verified facts. |
| S04 | Q, delete raw header `source` and the corresponding row cell. | F([E(INVALID_HEADER,1,null)]); no derived cell errors. |
| S05 | Q, append header "extra" and row cell "invented". | F([E(INVALID_HEADER,1,null)]). |
| S06 | R, remove only row 4's final source cell, leaving headers intact. | F([E(ROW_WIDTH,4,null)]). |
| S07 | R, replace rows with []. | F([E(EMPTY_DATASET,null,"rows")]). |
| S08 | Q, options.timestamp_column = "open". | F([E(INVALID_OPTIONS,null,"options.timestamp_column")]); mapping is ambiguous with a required price column. |
| S09 | Q, options.timestamp_format = "guess". | F([E(INVALID_OPTIONS,null,"options.timestamp_format")]); no timestamp guessing. |
| S10 | Q, add root property `extra: true`. | F([E(INVALID_REQUEST,null,"extra")]). |
| S11 | Q, options.confirmed_timezone = "host". | F([E(INVALID_OPTIONS,null,"options.confirmed_timezone")]); no host-zone fallback. |
| S12 | Q, rename only header `source` to `open`, retaining ten cells. | F([E(INVALID_HEADER,1,null)]); duplicate open/missing source are aggregated, with no guessed mapping. |

## Determinism and proof limits

For every case, rerunning the same typed input/options must preserve status,
metadata, candle values/order, gap status/missing list and ordered issue keys.
Changing the process timezone between UTC and Asia/Kolkata must not change them.
Cases N01 and T01–T04 specifically expose unit/offset mistakes; A01–A03 expose
fail-fast or partial-acceptance mistakes. Executable Rust assertions now cover
these expectations; their observed CI results are linked in the acceptance packet.

QOS-0002 used manual document/arithmetic review and lightweight Git diff checks.
Current Ubuntu CI executes the Rust normalization, CSV/pipeline and CLI tests,
not this Markdown, and does not independently validate its arithmetic. Office-
laptop runtime checks were **NOT RUN**. The later personal-Mac checklist and full
Rust suite passed, and the owner accepted aggregate P1 on 2026-10-29; see the packet
and its dated receipt. Browser/native-WASM runtime parity remains **NOT RUN**;
WASM compilation alone is not runtime parity evidence.
