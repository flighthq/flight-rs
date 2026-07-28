use std::sync::Arc;

use flighthq_easing::{
    ease_clamp, ease_cubic_bezier, ease_in_back, ease_in_out_quadratic, ease_in_power, ease_linear,
    ease_out_back, ease_smoothstep_range, ease_steps, get_easing_derivative,
};

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
    close(cubic(0.5), 0.125, 1e-12);

    let clamped = ease_clamp(Arc::new(|value| value * 2.0));
    close(clamped(-1.0), 0.0, 1e-12);
    close(clamped(2.0), 2.0, 1e-12);

    let remap = ease_smoothstep_range(10.0, 20.0);
    close(remap(10.0), 0.0, 1e-12);
    close(remap(15.0), 0.5, 1e-12);
    close(remap(20.0), 1.0, 1e-12);
}

#[test]
fn optional_defaults_and_derivatives_match_upstream_contracts() {
    let steps = ease_steps(4.0, None);
    close(steps(0.0), 0.0, 0.0);
    close(steps(0.26), 0.25, 0.0);
    close(steps(1.0), 1.0, 0.0);

    close(
        get_easing_derivative(Arc::new(ease_linear), 0.5, None),
        1.0,
        1e-9,
    );
}

#[test]
fn cubic_bezier_solver_handles_linear_curve() {
    let linear_bezier = ease_cubic_bezier(0.0, 0.0, 1.0, 1.0);
    for sample in [0.0, 0.1, 0.25, 0.5, 0.9, 1.0] {
        close(linear_bezier(sample), sample, 1e-6);
    }
}

