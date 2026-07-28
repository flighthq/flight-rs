use std::sync::{Arc, Mutex};

use flighthq_easing::{
    create_easing_samples, ease_clamp, ease_cubic_bezier, ease_in_back, ease_in_cubic,
    ease_in_out_quadratic, ease_in_power, ease_linear, ease_out_back, ease_piecewise,
    ease_smoothstep_range, ease_steps, get_easing_derivative,
};
use flighthq_types::{EasingFunction, EasingSegment};

fn callback(function: impl FnMut(f64) -> f64 + Send + 'static) -> EasingFunction {
    Arc::new(Mutex::new(Box::new(function)))
}

fn call(function: &EasingFunction, value: f64) -> f64 {
    function.lock().unwrap()(value)
}

fn close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "actual={actual}, expected={expected}, epsilon={epsilon}",
    );
}

#[test]
fn fixed_curves_preserve_endpoints_and_midpoints() {
    close(ease_linear(0.0), 0.0, 0.0);
    close(ease_linear(1.0), 1.0, 0.0);
    close(ease_in_out_quadratic(0.5), 0.5, 1e-12);
    close(ease_in_back(0.0), 0.0, 1e-12);
    close(ease_out_back(1.0), 1.0, 1e-12);
}

#[test]
fn generated_closures_capture_inputs() {
    let cubic = ease_in_power(3.0);
    close(call(&cubic, 0.5), 0.125, 1e-12);

    let clamped = ease_clamp(callback(|value| value * 2.0));
    close(call(&clamped, -1.0), 0.0, 1e-12);
    close(call(&clamped, 2.0), 2.0, 1e-12);

    let remap = ease_smoothstep_range(10.0, 20.0);
    close(call(&remap, 10.0), 0.0, 1e-12);
    close(call(&remap, 15.0), 0.5, 1e-12);
    close(call(&remap, 20.0), 1.0, 1e-12);
}

#[test]
fn optional_defaults_and_derivatives_match_upstream_contracts() {
    let steps = ease_steps(4.0, None);
    close(call(&steps, 0.0), 0.0, 0.0);
    close(call(&steps, 0.26), 0.25, 0.0);
    close(call(&steps, 1.0), 1.0, 0.0);

    close(
        get_easing_derivative(callback(ease_linear), 0.5, None),
        1.0,
        1e-9,
    );
}

#[test]
fn cubic_bezier_solver_handles_linear_curve() {
    let linear_bezier = ease_cubic_bezier(0.0, 0.0, 1.0, 1.0);
    for sample in [0.0, 0.1, 0.25, 0.5, 0.9, 1.0] {
        close(call(&linear_bezier, sample), sample, 1e-6);
    }
}

#[test]
fn typed_array_sampling_preserves_owned_output_and_f32_values() {
    let out = vec![0.0_f32; 10];
    let allocation = out.as_ptr();
    let samples = create_easing_samples(callback(ease_linear), 5.9, Some(out));
    assert_eq!(samples.as_ptr(), allocation);
    assert_eq!(samples.len(), 10);
    assert_eq!(&samples[..6], &[0.0, 0.25, 0.5, 0.75, 1.0, 0.0]);

    let midpoint = create_easing_samples(callback(ease_in_cubic), 1.0, None);
    close(f64::from(midpoint[0]), 0.125, 1e-7);
}

#[test]
fn structural_segments_lower_to_weighted_owned_records() {
    let piecewise = ease_piecewise(vec![
        EasingSegment {
            __flight_identity: Arc::new(()),
            ease: callback(ease_linear),
            weight: Some(1.0),
        },
        EasingSegment {
            __flight_identity: Arc::new(()),
            ease: callback(|_| 1.0),
            weight: Some(2.0),
        },
    ]);
    close(call(&piecewise, 1.0 / 6.0), 0.5, 1e-12);
    close(call(&piecewise, 2.0 / 3.0), 1.0, 1e-12);
}
