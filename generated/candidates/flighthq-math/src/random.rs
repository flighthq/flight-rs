// @generated from upstream/packages/math/src/random.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use flighthq_types::RandomSource;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/math/src/random.ts:21 (sha256:05c0098ff2bfa60260d390f6468662df57159ca6c1cbe6ebb7e1d6b0d27706f0)
pub fn create_random_source(seed: f64) -> RandomSource {
    let a: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(if (seed).is_finite() {
            (__flight_js_to_u32(seed) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64
        } else {
            0.0_f64
        }));
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut a = a.clone();
        move || -> f64 {
            (*a.lock().unwrap()) =
                (__flight_js_to_i32(((*a.lock().unwrap()).clone() + 1831565813.0_f64))
                    | __flight_js_to_i32(0.0_f64)) as f64;
            let mut t = __flight_js_to_i32(
                (__flight_js_to_i32((*a.lock().unwrap()).clone())
                    ^ __flight_js_to_i32(
                        (__flight_js_to_u32((*a.lock().unwrap()).clone())
                            >> (__flight_js_to_u32(15.0_f64) & 31)) as f64,
                    )) as f64,
            )
            .wrapping_mul(__flight_js_to_i32(
                (__flight_js_to_i32(1.0_f64) | __flight_js_to_i32((*a.lock().unwrap()).clone()))
                    as f64,
            )) as f64;
            t = (__flight_js_to_i32(
                (t + __flight_js_to_i32(
                    (__flight_js_to_i32(t)
                        ^ __flight_js_to_i32(
                            (__flight_js_to_u32(t) >> (__flight_js_to_u32(7.0_f64) & 31)) as f64,
                        )) as f64,
                )
                .wrapping_mul(__flight_js_to_i32(
                    (__flight_js_to_i32(61.0_f64) | __flight_js_to_i32(t)) as f64,
                )) as f64),
            ) ^ __flight_js_to_i32(t)) as f64;
            return ((__flight_js_to_u32(
                (__flight_js_to_i32(t)
                    ^ __flight_js_to_i32(
                        (__flight_js_to_u32(t) >> (__flight_js_to_u32(14.0_f64) & 31)) as f64,
                    )) as f64,
            ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64
                / 4294967296.0_f64);
        }
    })
        as Box<dyn FnMut() -> f64 + Send + 'static>));
}
