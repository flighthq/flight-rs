// @generated from upstream/packages/easing/src/easeCubicBezier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/easing/src/easeCubicBezier.ts:9 (sha256:d7f3f750548dac640f288e297287aabe71bc61372f08e47181b143eb7fdfe17b)
pub fn ease_cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64) -> EasingFunction {
    let cx = (3.0_f64 * x1);
    let bx = ((3.0_f64 * (x2 - x1)) - cx);
    let ax = ((1.0_f64 - cx) - bx);
    let cy = (3.0_f64 * y1);
    let by = ((3.0_f64 * (y2 - y1)) - cy);
    let ay = ((1.0_f64 - cy) - by);
    let sample_x = move |s: f64| -> f64 { (((((ax * s) + bx) * s) + cx) * s) };
    let sample_y = move |s: f64| -> f64 { (((((ay * s) + by) * s) + cy) * s) };
    let sample_derivative_x =
        move |s: f64| -> f64 { (((((3.0_f64 * ax) * s) + (2.0_f64 * bx)) * s) + cx) };
    let solve_parameter_for_x = move |x: f64, epsilon: f64| -> f64 {
        let mut s = x;
        {
            let mut i = 0.0_f64;
            while (i < 8.0_f64) {
                let x_error = (sample_x(s) - x);
                if ((x_error).abs() < epsilon) {
                    return s;
                }
                let derivative = sample_derivative_x(s);
                if ((derivative).abs() < 0.000001_f64) {
                    break;
                }
                {
                    s -= (x_error / derivative);
                    s
                };
                {
                    i += 1.0;
                    i
                };
            }
        }
        let mut low = 0.0_f64;
        let mut high = 1.0_f64;
        {
            s = x;
            s
        };
        if (s < low) {
            return low;
        }
        if (s > high) {
            return high;
        }
        while (low < high) {
            let sampled = sample_x(s);
            if ((sampled - x).abs() < epsilon) {
                return s;
            }
            if (x > sampled) {
                {
                    low = s;
                    low
                };
            } else {
                {
                    high = s;
                    high
                };
            }
            {
                s = (((high - low) * 0.5_f64) + low);
                s
            };
        }
        return s;
    };
    return std::sync::Arc::new(move |t: f64| -> f64 {
        if (t <= 0.0_f64) {
            return 0.0_f64;
        }
        if (t >= 1.0_f64) {
            return 1.0_f64;
        }
        return sample_y(solve_parameter_for_x(t, 1e-7_f64));
    });
}
