use super::support::*;
use quantos_core::sma::*;

#[test]
fn sc09_representable_invalid_periods_and_first_config_error() {
    for (fast, slow, field) in [
        (0, 3, "fast_period"),
        (10000, 10000, "fast_period"),
        (u64::MAX, 0, "fast_period"),
        (1, 0, "slow_period"),
        (1, 1, "slow_period"),
        (1, 10001, "slow_period"),
        (1, u64::MAX, "slow_period"),
        (3, 3, "periods"),
        (4, 3, "periods"),
    ] {
        failure(
            &input(fast, slow, &[100]),
            SmaErrorCodeV1::InvalidConfig,
            None,
            field,
        );
    }
    // Missing/negative/fractional/bool/string fields and invalid enum variants
    // cannot be constructed in this Rust-only API; no decoder tests are claimed.
}

#[test]
fn sc10_gaps_duplicates_descending_and_late_invalid_are_atomic() {
    for timestamps in [[0, 600000], [0, 0], [300000, 0]] {
        let mut i = input(1, 2, &[100, 100]);
        for (c, t) in i.candles.iter_mut().zip(timestamps) {
            c.ts_utc_ms = t;
        }
        failure(
            &i,
            SmaErrorCodeV1::InvalidCandle,
            Some(1),
            "candles[1].ts_utc_ms",
        );
    }
    let mut i = input(1, 2, &[200, 100, 200, 100]);
    i.candles[3].ts_utc_ms = 1200000;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(3),
        "candles[3].ts_utc_ms",
    );
}

#[test]
fn sc11_input_config_metadata_and_price_precedence() {
    failure(
        &input(1, 2, &[]),
        SmaErrorCodeV1::InvalidInput,
        None,
        "candles",
    );
    let mut i = input(0, 3, &[100]);
    i.schema_version = "2".into();
    failure(&i, SmaErrorCodeV1::InvalidInput, None, "schema_version");
    i.schema_version = "1".into();
    i.metadata.source.clear();
    failure(&i, SmaErrorCodeV1::InvalidConfig, None, "fast_period");
    i.fast_period = 1;
    i.metadata.source = " \t\r\n".into();
    failure(&i, SmaErrorCodeV1::InvalidMetadata, None, "metadata.source");
    let mut i = input(1, 2, &[100]);
    i.candles[0].close = 0;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].close",
    );
    let mut i = input(1, 2, &[M]);
    i.candles[0].close = M + 1;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].close",
    );
    let mut i = input(1, 2, &[100]);
    i.candles[0].close = 101;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].ohlc",
    );
    i.candles[0].source = "raw-private-sentinel".into();
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].source",
    );
    assert!(
        !serde_json::to_string(&generate_sma_v1(&i))
            .unwrap()
            .contains("raw-private-sentinel")
    );
}

#[test]
fn every_representable_candle_field_is_revalidated_in_order() {
    for field in [
        "symbol",
        "exchange",
        "source",
        "ts_utc_ms",
        "open",
        "high",
        "low",
        "close",
        "ohlc",
        "volume",
    ] {
        let mut i = input(1, 2, &[100, 100]);
        let c = &mut i.candles[1];
        match field {
            "symbol" => c.symbol = "OTHER".into(),
            "exchange" => c.exchange = "OTHER".into(),
            "source" => c.source = "OTHER".into(),
            "ts_utc_ms" => c.ts_utc_ms = 4_102_444_800_000,
            "open" => c.open = 0,
            "high" => c.high = M + 1,
            "low" => c.low = u64::MAX,
            "close" => c.close = 0,
            "ohlc" => c.low = 101,
            "volume" => c.volume = M + 1,
            _ => unreachable!(),
        }
        failure(
            &i,
            SmaErrorCodeV1::InvalidCandle,
            Some(1),
            &format!("candles[1].{field}"),
        );
    }
    for field in ["symbol", "exchange"] {
        let mut i = input(1, 2, &[100]);
        if field == "symbol" {
            i.candles[0].symbol = " \t\n\r".into();
        } else {
            i.candles[0].exchange.clear();
        }
        failure(
            &i,
            SmaErrorCodeV1::InvalidCandle,
            Some(0),
            &format!("candles[0].{field}"),
        );
    }
}

#[test]
fn price_validation_precedes_ohlc_and_volume_and_first_bar_wins() {
    let mut i = input(1, 2, &[100, 100]);
    i.candles[0].open = 0;
    i.candles[0].high = 0;
    i.candles[0].volume = M + 1;
    i.candles[1].symbol.clear();
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].open",
    );
    i.candles[0].open = 100;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].high",
    );
    i.candles[0].high = 99;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].ohlc",
    );
    i.candles[0].high = 100;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].volume",
    );
}

#[test]
fn timestamp_range_endpoints_and_arbitrary_valid_segment_origin() {
    let mut i = input(1, 2, &[100]);
    i.candles[0].ts_utc_ms = 4_102_444_799_999;
    assert_eq!(report(&i).decisions, vec![H]);
    i.candles[0].ts_utc_ms += 1;
    failure(
        &i,
        SmaErrorCodeV1::InvalidCandle,
        Some(0),
        "candles[0].ts_utc_ms",
    );
    let mut i = input(1, 2, &[200, 100, 200]);
    for c in &mut i.candles {
        c.ts_utc_ms += 17;
    }
    assert_eq!(report(&i).decisions, vec![H, H, E]);
}
