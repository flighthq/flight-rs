// @generated from upstream/packages/easing/src/createEasingSamples.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::EasingFunction;

// Source: upstream/packages/easing/src/createEasingSamples.ts:17 (sha256:5327418e9e6e65ebe8bc0a8b9803c9551818c162b5a8c16495b2aaa7165ad1ee)
pub fn create_easing_samples(ease: EasingFunction, count: f64, out: Option<Vec<f32>>) -> Vec<f32> {
    if (!(count).is_finite()) || (count < 1.0_f64) {
        panic!(
            "{}",
            "createEasingSamples: count must be a finite integer >= 1"
        );
    }
    let n = (count).floor();
    let mut result = (out).clone().unwrap_or(vec![0.0_f32; (n) as usize]);
    if (n == 1.0_f64) {
        result[0.0_f64 as usize] = ({
            let __flight_callback = (ease).clone();
            let __flight_result = __flight_callback.lock().unwrap()(0.5_f64);
            __flight_result
        }) as f32;
        return result;
    }
    let step = (1.0_f64 / (n - 1.0_f64));
    {
        let mut i = 0.0_f64;
        while (i < n) {
            let t = (i * step);
            result[i as usize] = ({
                let __flight_callback = (ease).clone();
                let __flight_result = __flight_callback.lock().unwrap()(if (t < 0.0_f64) {
                    0.0_f64
                } else {
                    if (t > 1.0_f64) { 1.0_f64 } else { t }
                });
                __flight_result
            }) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    result[0.0_f64 as usize] = ({
        let __flight_callback = (ease).clone();
        let __flight_result = __flight_callback.lock().unwrap()(0.0_f64);
        __flight_result
    }) as f32;
    result[(n - 1.0_f64) as usize] = ({
        let __flight_callback = (ease).clone();
        let __flight_result = __flight_callback.lock().unwrap()(1.0_f64);
        __flight_result
    }) as f32;
    return result;
}
