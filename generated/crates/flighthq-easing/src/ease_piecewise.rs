// @generated from upstream/packages/easing/src/easePiecewise.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{EasingFunction, EasingSegment};

// Source: upstream/packages/easing/src/easePiecewise.ts:13 (sha256:110b70587ac6c686c40eb576d5081aeacb20835c46efadc9fa8ca1595f3d6d30)
#[derive(Clone)]
struct EasePiecewiseRecord1 {
    __flight_identity: std::sync::Arc<()>,
    ease: EasingFunction,
    end: f64,
    start: f64,
}
impl PartialEq for EasePiecewiseRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ease_piecewise(segments: Vec<EasingSegment>) -> EasingFunction {
    if ((segments.len() as f64) == 0.0_f64) {
        panic!("{}", "easePiecewise: segments array must not be empty");
    }
    let total_weight = (segments)
        .iter()
        .cloned()
        .fold(0.0_f64, |sum: f64, seg: EasingSegment| -> f64 {
            (sum + (seg.weight).unwrap_or(1.0_f64))
        });
    if (total_weight <= 0.0_f64) {
        panic!(
            "{}",
            "easePiecewise: total segment weight must be greater than zero"
        );
    }
    let breakpoints: std::sync::Arc<std::sync::Mutex<Vec<EasePiecewiseRecord1>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut accumulated = 0.0_f64;
    for seg in (segments).iter().cloned() {
        let weight = (seg.weight).unwrap_or(1.0_f64);
        let start = (accumulated / total_weight);
        accumulated += weight;
        let end = (accumulated / total_weight);
        (*breakpoints.lock().unwrap()).push(EasePiecewiseRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            ease: (seg.ease).clone(),
            end: end,
            start: start,
        });
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut breakpoints = breakpoints.clone();
        let segments = segments.clone();
        move |t: f64| -> f64 {
            {
                let mut i = 0.0_f64;
                while (i < ((*breakpoints.lock().unwrap()).len() as f64)) {
                    let bp = (*breakpoints.lock().unwrap())[i as usize].clone();
                    if (t <= bp.end)
                        || (i == (((*breakpoints.lock().unwrap()).len() as f64) - 1.0_f64))
                    {
                        let span = (bp.end - bp.start);
                        let local_t = if (span > 0.0_f64) {
                            ((t - bp.start) / span)
                        } else {
                            1.0_f64
                        };
                        let clamped_t = if (local_t < 0.0_f64) {
                            0.0_f64
                        } else {
                            if (local_t > 1.0_f64) {
                                1.0_f64
                            } else {
                                local_t
                            }
                        };
                        return {
                            let __flight_callback = (bp.ease).clone();
                            let __flight_result = __flight_callback.lock().unwrap()(clamped_t);
                            __flight_result
                        };
                    }
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            return {
                let __flight_callback =
                    (segments[((segments.len() as f64) - 1.0_f64) as usize].ease).clone();
                let __flight_result = __flight_callback.lock().unwrap()(1.0_f64);
                __flight_result
            };
        }
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
}
