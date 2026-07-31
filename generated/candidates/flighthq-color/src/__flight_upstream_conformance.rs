// @generated from upstream @flighthq/color tests; do not edit.

fn flight_close(actual: f64, expected: f64, precision: i32) {
    let tolerance = 0.5_f64 * 10.0_f64.powi(-precision);
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual}, expected={expected}, precision={precision}"
    );
}

// srgbTransfer.test.ts:4 — linearChannelToSrgb > maps 0 → 0
#[test]
fn upstream_srgb_transfer_1_linear_channel_to_srgb_maps_0_0() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::linear_channel_to_srgb(0.0_f64), 0.0_f64);
}
// srgbTransfer.test.ts:7 — linearChannelToSrgb > maps 1 → 1
#[test]
fn upstream_srgb_transfer_2_linear_channel_to_srgb_maps_1_1() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(crate::linear_channel_to_srgb(1.0_f64), 1.0_f64, 8_i32);
}
// srgbTransfer.test.ts:10 — linearChannelToSrgb > is the inverse of the sRGB decode for a round-trip
#[test]
fn upstream_srgb_transfer_3_linear_channel_to_srgb_is_the_inverse_of_the_s_rgb_decode_for_a_round_trip()
 {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    let linear = 0.5_f64;
    let srgb = crate::linear_channel_to_srgb(linear);
    flight_close(crate::srgb_channel_to_linear(srgb), linear, 8_i32);
}
// srgbTransfer.test.ts:18 — srgbChannelToLinear > maps 0 → 0 and 1 → 1
#[test]
fn upstream_srgb_transfer_4_srgb_channel_to_linear_maps_0_0_and_1_1() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    assert_eq!(crate::srgb_channel_to_linear(0.0_f64), 0.0_f64);
    flight_close(crate::srgb_channel_to_linear(1.0_f64), 1.0_f64, 8_i32);
}
// srgbTransfer.test.ts:22 — srgbChannelToLinear > decodes mid sRGB below the linear midpoint
#[test]
fn upstream_srgb_transfer_5_srgb_channel_to_linear_decodes_mid_s_rgb_below_the_linear_midpoint() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    flight_close(
        crate::srgb_channel_to_linear((128.0_f64 / 255.0_f64)),
        0.21586_f64,
        5_i32,
    );
}
// srgbTransfer.test.ts:26 — srgbChannelToLinear > round-trips with linearChannelToSrgb
#[test]
fn upstream_srgb_transfer_6_srgb_channel_to_linear_round_trips_with_linear_channel_to_srgb() {
    let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();
    let srgb = 0.25_f64;
    flight_close(
        crate::linear_channel_to_srgb(crate::srgb_channel_to_linear(srgb)),
        srgb,
        8_i32,
    );
}
