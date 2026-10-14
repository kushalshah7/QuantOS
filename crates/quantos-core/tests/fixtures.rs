// Fixture QA only. This parser is restricted to our invented examples, not user CSV.
const HEADER: &str = "symbol,exchange,interval,ts_utc_ms,open,high,low,close,volume,source";

fn fixture_rows(csv: &str) -> Vec<Vec<&str>> {
    let mut lines = csv.lines();
    assert_eq!(lines.next(), Some(HEADER));
    lines
        .map(|line| {
            let row: Vec<_> = line.split(',').collect();
            assert_eq!(row.len(), 10);
            assert_eq!(&row[..3], &["SYNTH", "TEST", "5m"]);
            assert_eq!(row[9], "synthetic");
            row
        })
        .collect()
}

fn timestamp(row: &[&str]) -> i64 {
    row[3].parse().expect("fixture timestamp must be UTC ms")
}

fn ohlc_is_consistent(row: &[&str]) -> bool {
    let prices: Vec<f64> = row[4..8]
        .iter()
        .map(|value| value.parse().expect("fixture price must be numeric"))
        .collect();
    let [open, high, low, close] = prices[..] else {
        panic!("fixture must have four prices");
    };
    prices.iter().all(|price| price.is_finite() && *price > 0.0)
        && low <= open
        && low <= close
        && high >= open
        && high >= close
        && high >= low
}

#[test]
fn valid_fixture_has_sorted_consistent_synthetic_bars() {
    let rows = fixture_rows(include_str!(
        "../../../fixtures/synthetic/valid-candles.csv"
    ));
    assert_eq!(rows.len(), 3);
    assert_eq!(timestamp(&rows[0]), 0);
    for row in &rows {
        assert!(ohlc_is_consistent(row));
        let volume: u64 = row[8].parse().expect("fixture volume must be nonnegative");
        assert!(volume > 0);
    }
    for pair in rows.windows(2) {
        assert_eq!(timestamp(&pair[1]) - timestamp(&pair[0]), 300_000);
    }
}

#[test]
fn invalid_examples_retain_their_intended_contract_errors() {
    let invalid = fixture_rows(include_str!("../../../fixtures/synthetic/invalid-ohlc.csv"));
    assert_eq!(invalid.len(), 1);
    assert!(!ohlc_is_consistent(&invalid[0]));

    let duplicates = fixture_rows(include_str!(
        "../../../fixtures/synthetic/duplicate-timestamp.csv"
    ));
    assert_eq!(duplicates.len(), 2);
    assert_eq!(timestamp(&duplicates[0]), timestamp(&duplicates[1]));

    let ambiguous = include_str!("../../../fixtures/synthetic/missing-timezone.csv");
    let row: Vec<_> = ambiguous.lines().nth(1).unwrap().split(',').collect();
    assert_eq!(row[3], "1970-01-01T09:15:00");
    assert_eq!(row[9], "synthetic");
    assert!(!row[3].ends_with('Z'));
    assert!(!row[3].contains('+'));
}
