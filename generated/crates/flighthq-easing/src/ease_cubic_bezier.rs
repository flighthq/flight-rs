// @generated from upstream/packages/easing/src/easeCubicBezier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::EasingFunction;

// Source: upstream/packages/easing/src/easeCubicBezier.ts:9 (sha256:d7f3f750548dac640f288e297287aabe71bc61372f08e47181b143eb7fdfe17b)
pub fn ease_cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64) -> EasingFunction {
    let cx = (3.0_f64 * x1);
    let bx = ((3.0_f64 * (x2 - x1)) - cx);
    let ax = ((1.0_f64 - cx) - bx);
    let cy = (3.0_f64 * y1);
    let by = ((3.0_f64 * (y2 - y1)) - cy);
    let ay = ((1.0_f64 - cy) - by);
    let mut sample_x: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |s: f64| -> f64 {
        (((((ax * s) + bx) * s) + cx) * s)
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
    let mut sample_y: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |s: f64| -> f64 {
        (((((ay * s) + by) * s) + cy) * s)
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
    let mut sample_derivative_x: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |s: f64| -> f64 {
        (((((3.0_f64 * ax) * s) + (2.0_f64 * bx)) * s) + cx)
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
    let mut solve_parameter_for_x: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let sample_derivative_x = sample_derivative_x.clone();
        let sample_x = sample_x.clone();
        move |x: f64, epsilon: f64| -> f64 {
            let mut s = x;
            {
                let mut i = 0.0_f64;
                while (i < 8.0_f64) {
                    let x_error = ({
                        let __flight_callback = (sample_x).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(s);
                        __flight_result
                    } - x);
                    if ((x_error).abs() < epsilon) {
                        return s;
                    }
                    let derivative = {
                        let __flight_callback = (sample_derivative_x).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(s);
                        __flight_result
                    };
                    if ((derivative).abs() < 0.000001_f64) {
                        break;
                    }
                    s -= (x_error / derivative);
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            let mut low = 0.0_f64;
            let mut high = 1.0_f64;
            s = x;
            if (s < low) {
                return low;
            }
            if (s > high) {
                return high;
            }
            while (low < high) {
                let sampled = {
                    let __flight_callback = (sample_x).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(s);
                    __flight_result
                };
                if ((sampled - x).abs() < epsilon) {
                    return s;
                }
                if (x > sampled) {
                    low = s;
                } else {
                    high = s;
                }
                s = (((high - low) * 0.5_f64) + low);
            }
            return s;
        }
    })
        as Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>));
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let sample_y = sample_y.clone();
        let solve_parameter_for_x = solve_parameter_for_x.clone();
        move |t: f64| -> f64 {
            if (t <= 0.0_f64) {
                return 0.0_f64;
            }
            if (t >= 1.0_f64) {
                return 1.0_f64;
            }
            return {
                let __flight_callback = (sample_y).clone();
                let __flight_result = __flight_callback.lock().unwrap()({
                    let __flight_callback = (solve_parameter_for_x).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(t, 1e-7_f64);
                    __flight_result
                });
                __flight_result
            };
        }
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
}
