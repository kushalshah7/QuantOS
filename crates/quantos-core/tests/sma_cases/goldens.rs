use super::support::*;
use quantos_core::sma::*;
use serde_json::json;

#[test]
fn sc01_warmup_first_joint_and_equality_are_hold() {
    assert_eq!(
        report(&input(2, 3, &[100, 100, 100])).decisions,
        vec![H, H, H]
    );
}

#[test]
fn sc02_short_inputs_and_already_above_or_below_have_no_initial_signal() {
    for closes in [&[101][..], &[100, 200][..], &[200, 100][..]] {
        assert_eq!(
            report(&input(1, 2, closes)).decisions,
            vec![H; closes.len()]
        );
    }
    assert_eq!(report(&input(2, 3, &[100])).decisions, vec![H]);
}

#[test]
fn sc03_both_crossings_only_after_completed_close() {
    let i = input(2, 3, &[300, 200, 100, 200, 300, 200, 100, 100]);
    assert_eq!(report(&i).decisions, vec![H, H, H, H, E, H, X, H]);
}

#[test]
fn sc04_equal_resets_retain_repeated_enter_and_exit_decisions() {
    let i = input(1, 2, &[100, 100, 200, 200, 300, 300, 200, 200, 100]);
    assert_eq!(report(&i).decisions, vec![H, H, E, H, E, H, X, H, X]);
}

#[test]
fn sc05_earliest_final_signal_and_exact_public_success_shape() {
    let i = input(1, 2, &[200, 100, 200]);
    assert_eq!(report(&i).decisions, vec![H, H, E]);
    assert_eq!(
        serde_json::to_value(generate_sma_v1(&i)).unwrap(),
        json!({
            "status":"success", "schema_version":"1", "report":{
                "metadata":{"source":"synthetic","synthetic":true,"adjustment_status":"unadjusted","redistribution_status":"unknown"},
                "bar_completion":"all_complete","fast_period":1,"slow_period":2,
                "decisions":["hold","hold","enter_long"]
            }
        })
    );
}

#[test]
fn sc06_fractional_averages_are_not_truncated_before_comparison() {
    // At 2: 199/2 < 299/3; at 3: 200/2 = 300/3;
    // at 4: 201/2 > 300/3, so rounding both to 100 would lose the crossing.
    assert_eq!(
        report(&input(2, 3, &[100, 100, 99, 101, 100])).decisions,
        vec![H, H, H, H, E]
    );
}

#[test]
fn sc07_near_m_products_differ_by_one_and_private_sums_may_exceed_m() {
    assert_eq!(
        report(&input(1, 2, &[M - 1, M, M - 1])).decisions,
        vec![H, H, X]
    );
}

#[test]
fn sc08_max_periods_short_input_and_full_wide_windows() {
    assert_eq!(report(&input(9999, 10000, &[M])).decisions, vec![H]);
    let i = input(9999, 10000, &vec![M; 10001]);
    // Sum 10000*M and cross-products exceed u64. No averages are exposed;
    // exact decisions are the executable observation of this valid boundary.
    assert_eq!(report(&i).decisions, vec![H; 10001]);
}

#[test]
fn maximum_period_subtract_add_updates_preserve_one_unit_comparisons() {
    for (baseline, last, signal) in [(M, M - 1, X), (M - 1, M, E)] {
        let mut closes = vec![baseline; 10001];
        closes[10000] = last;
        let r = report(&input(9999, 10000, &closes));
        assert_eq!(&r.decisions[..10000], &[H; 10000]);
        assert_eq!(r.decisions[10000], signal);
    }
}
