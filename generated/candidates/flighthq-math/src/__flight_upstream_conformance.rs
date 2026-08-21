// @generated from upstream @flighthq/math tests; do not edit.

fn flight_close(actual: f64, expected: f64, precision: i32) {
    let tolerance = 0.5_f64 * 10.0_f64.powi(-precision);
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual}, expected={expected}, precision={precision}"
    );
}

// clamp.test.ts:4 — clamp > clamps below minimum
#[test]
fn upstream_clamp_1_clamp_clamps_below_minimum() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::clamp(-(5.0_f64), 0.0_f64, 10.0_f64), 0.0_f64);
}
// clamp.test.ts:7 — clamp > clamps above maximum
#[test]
fn upstream_clamp_2_clamp_clamps_above_maximum() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::clamp(15.0_f64, 0.0_f64, 10.0_f64), 10.0_f64);
}
// clamp.test.ts:10 — clamp > leaves value within range unchanged
#[test]
fn upstream_clamp_3_clamp_leaves_value_within_range_unchanged() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::clamp(5.0_f64, 0.0_f64, 10.0_f64), 5.0_f64);
}
// clamp.test.ts:13 — clamp > leaves value equal to min unchanged
#[test]
fn upstream_clamp_4_clamp_leaves_value_equal_to_min_unchanged() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::clamp(0.0_f64, 0.0_f64, 10.0_f64), 0.0_f64);
}
// clamp.test.ts:16 — clamp > leaves value equal to max unchanged
#[test]
fn upstream_clamp_5_clamp_leaves_value_equal_to_max_unchanged() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::clamp(10.0_f64, 0.0_f64, 10.0_f64), 10.0_f64);
}
// clamp.test.ts:19 — clamp > propagates NaN
#[test]
fn upstream_clamp_6_clamp_propagates_na_n() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert!((crate::clamp(f64::NAN, 0.0_f64, 10.0_f64)).is_nan());
}
// clamp.test.ts:25 — inRange > returns true for a value within range
#[test]
fn upstream_clamp_7_in_range_returns_true_for_a_value_within_range() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::in_range(5.0_f64, 0.0_f64, 10.0_f64), true);
}
// clamp.test.ts:28 — inRange > returns true at the minimum boundary
#[test]
fn upstream_clamp_8_in_range_returns_true_at_the_minimum_boundary() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::in_range(0.0_f64, 0.0_f64, 10.0_f64), true);
}
// clamp.test.ts:31 — inRange > returns true at the maximum boundary
#[test]
fn upstream_clamp_9_in_range_returns_true_at_the_maximum_boundary() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::in_range(10.0_f64, 0.0_f64, 10.0_f64), true);
}
// clamp.test.ts:34 — inRange > returns false below minimum
#[test]
fn upstream_clamp_10_in_range_returns_false_below_minimum() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::in_range(-(1.0_f64), 0.0_f64, 10.0_f64), false);
}
// clamp.test.ts:37 — inRange > returns false above maximum
#[test]
fn upstream_clamp_11_in_range_returns_false_above_maximum() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::in_range(11.0_f64, 0.0_f64, 10.0_f64), false);
}
// clamp.test.ts:43 — saturate > clamps below 0 to 0
#[test]
fn upstream_clamp_12_saturate_clamps_below_0_to_0() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::saturate(-(1.0_f64)), 0.0_f64);
}
// clamp.test.ts:46 — saturate > clamps above 1 to 1
#[test]
fn upstream_clamp_13_saturate_clamps_above_1_to_1() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::saturate(2.0_f64), 1.0_f64);
}
// clamp.test.ts:49 — saturate > leaves value within [0, 1] unchanged
#[test]
fn upstream_clamp_14_saturate_leaves_value_within_0_1_unchanged() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::saturate(0.5_f64), 0.5_f64);
}
// clamp.test.ts:52 — saturate > leaves 0 unchanged
#[test]
fn upstream_clamp_15_saturate_leaves_0_unchanged() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::saturate(0.0_f64), 0.0_f64);
}
// clamp.test.ts:55 — saturate > leaves 1 unchanged
#[test]
fn upstream_clamp_16_saturate_leaves_1_unchanged() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::saturate(1.0_f64), 1.0_f64);
}
// clamp.test.ts:58 — saturate > returns 0 for NaN (GPU semantics)
#[test]
fn upstream_clamp_17_saturate_returns_0_for_na_n_gpu_semantics() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::saturate(f64::NAN), 0.0_f64);
}
// comparison.test.ts:4 — approxEqual > returns true for identical values
#[test]
fn upstream_comparison_1_approx_equal_returns_true_for_identical_values() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_equal(1.0_f64, 1.0_f64, None), true);
}
// comparison.test.ts:7 — approxEqual > returns true when values are within the default epsilon
#[test]
fn upstream_comparison_2_approx_equal_returns_true_when_values_are_within_the_default_epsilon() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(
        crate::approx_equal(1.0_f64, (1.0_f64 + 1e-7_f64), None),
        true
    );
}
// comparison.test.ts:10 — approxEqual > returns false when values differ by more than the default epsilon
#[test]
fn upstream_comparison_3_approx_equal_returns_false_when_values_differ_by_more_than_the_default_epsilon()
 {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(
        crate::approx_equal(1.0_f64, (1.0_f64 + 0.00001_f64), None),
        false
    );
}
// comparison.test.ts:13 — approxEqual > accepts a custom epsilon
#[test]
fn upstream_comparison_4_approx_equal_accepts_a_custom_epsilon() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_equal(1.0_f64, 1.1_f64, Some(0.2_f64)), true);
    assert_eq!(crate::approx_equal(1.0_f64, 1.3_f64, Some(0.2_f64)), false);
}
// comparison.test.ts:17 — approxEqual > returns true for zero and near-zero
#[test]
fn upstream_comparison_5_approx_equal_returns_true_for_zero_and_near_zero() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_equal(0.0_f64, 1e-7_f64, None), true);
}
// comparison.test.ts:20 — approxEqual > works with negative values
#[test]
fn upstream_comparison_6_approx_equal_works_with_negative_values() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(
        crate::approx_equal(-(1.0_f64), (-(1.0_f64) - 1e-7_f64), None),
        true
    );
}
// comparison.test.ts:26 — approxEqualRelative > returns true for identical values
#[test]
fn upstream_comparison_7_approx_equal_relative_returns_true_for_identical_values() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(
        crate::approx_equal_relative(1000.0_f64, 1000.0_f64, None),
        true
    );
}
// comparison.test.ts:29 — approxEqualRelative > returns true for large values within relative epsilon
#[test]
fn upstream_comparison_8_approx_equal_relative_returns_true_for_large_values_within_relative_epsilon()
 {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(
        crate::approx_equal_relative(100000000.0_f64, (100000000.0_f64 + 1.0_f64), None),
        true
    );
}
// comparison.test.ts:32 — approxEqualRelative > returns false when large values differ beyond relative epsilon
#[test]
fn upstream_comparison_9_approx_equal_relative_returns_false_when_large_values_differ_beyond_relative_epsilon()
 {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(
        crate::approx_equal_relative(100000000.0_f64, (100000000.0_f64 * 2.0_f64), None),
        false
    );
}
// comparison.test.ts:35 — approxEqualRelative > returns true for near-zero values within absolute epsilon
#[test]
fn upstream_comparison_10_approx_equal_relative_returns_true_for_near_zero_values_within_absolute_epsilon()
 {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_equal_relative(0.0_f64, 1e-7_f64, None), true);
}
// comparison.test.ts:41 — approxZero > returns true for exact zero
#[test]
fn upstream_comparison_11_approx_zero_returns_true_for_exact_zero() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_zero(0.0_f64, None), true);
}
// comparison.test.ts:44 — approxZero > returns true for values within epsilon
#[test]
fn upstream_comparison_12_approx_zero_returns_true_for_values_within_epsilon() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_zero(1e-7_f64, None), true);
}
// comparison.test.ts:47 — approxZero > returns false for values outside epsilon
#[test]
fn upstream_comparison_13_approx_zero_returns_false_for_values_outside_epsilon() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_zero(0.0001_f64, None), false);
}
// comparison.test.ts:50 — approxZero > accepts a custom epsilon
#[test]
fn upstream_comparison_14_approx_zero_accepts_a_custom_epsilon() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::approx_zero(0.01_f64, Some(0.1_f64)), true);
    assert_eq!(crate::approx_zero(0.2_f64, Some(0.1_f64)), false);
}
// constants.test.ts:36 — constants > DEG_TO_RAD > converts 180 degrees to π
#[test]
fn upstream_constants_3_constants_deg_to_rad_converts_180_degrees_to() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(
        (180.0_f64 * crate::DEG_TO_RAD),
        std::f64::consts::PI,
        10_i32,
    );
}
// constants.test.ts:39 — constants > DEG_TO_RAD > converts 360 degrees to 2π
#[test]
fn upstream_constants_4_constants_deg_to_rad_converts_360_degrees_to_2() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(
        (360.0_f64 * crate::DEG_TO_RAD),
        (std::f64::consts::PI * 2.0_f64),
        10_i32,
    );
}
// constants.test.ts:44 — constants > EPSILON > is a positive number
#[test]
fn upstream_constants_5_constants_epsilon_is_a_positive_number() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert!(crate::EPSILON > 0.0_f64);
}
// constants.test.ts:47 — constants > EPSILON > is smaller than 1e-5
#[test]
fn upstream_constants_6_constants_epsilon_is_smaller_than_1e_5() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert!(crate::EPSILON < 0.00001_f64);
}
// constants.test.ts:52 — constants > HALF_PI > equals π / 2
#[test]
fn upstream_constants_7_constants_half_pi_equals_2() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(crate::HALF_PI, (std::f64::consts::PI / 2.0_f64), 10_i32);
}
// constants.test.ts:57 — constants > RAD_TO_DEG > converts π radians to 180 degrees
#[test]
fn upstream_constants_8_constants_rad_to_deg_converts_radians_to_180_degrees() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(
        (std::f64::consts::PI * crate::RAD_TO_DEG),
        180.0_f64,
        10_i32,
    );
}
// constants.test.ts:60 — constants > RAD_TO_DEG > is the reciprocal of DEG_TO_RAD
#[test]
fn upstream_constants_9_constants_rad_to_deg_is_the_reciprocal_of_deg_to_rad() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close((crate::RAD_TO_DEG * crate::DEG_TO_RAD), 1.0_f64, 10_i32);
}
// constants.test.ts:65 — constants > TAU > equals 2π
#[test]
fn upstream_constants_10_constants_tau_equals_2() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(crate::TAU, (std::f64::consts::PI * 2.0_f64), 10_i32);
}
