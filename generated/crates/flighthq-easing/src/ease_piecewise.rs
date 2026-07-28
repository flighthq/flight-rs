// @generated from upstream/packages/easing/src/easePiecewise.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use flighthq_types::{EasingFunction, EasingSegment};

// Source: upstream/packages/easing/src/easePiecewise.ts:13 (sha256:110b70587ac6c686c40eb576d5081aeacb20835c46efadc9fa8ca1595f3d6d30)
#[derive(Clone)]
struct EasePiecewiseRecord1 {
    ease: EasingFunction,
    end: f64,
    start: f64,
}

pub fn ease_piecewise(segments: Vec<EasingSegment>) -> EasingFunction {
    if ((segments.len() as f64) == 0.0_f64) {
        panic!("easePiecewise: segments array must not be empty");
    }
    let total_weight = (segments)
        .iter()
        .cloned()
        .fold(0.0_f64, move |sum: f64, seg: EasingSegment| -> f64 {
            (sum + (seg.weight).unwrap_or(1.0_f64))
        });
    if (total_weight <= 0.0_f64) {
        panic!("easePiecewise: total segment weight must be greater than zero");
    }
    let mut breakpoints: Vec<EasePiecewiseRecord1> = vec![];
    let mut accumulated = 0.0_f64;
    for seg in (segments).iter().cloned() {
        let weight = (seg.weight).unwrap_or(1.0_f64);
        let start = (accumulated / total_weight);
        accumulated += weight;
        let end = (accumulated / total_weight);
        breakpoints.push(EasePiecewiseRecord1 {
            ease: seg.ease,
            end: end,
            start: start,
        });
    }
    return std::sync::Arc::new(move |t: f64| -> f64 {
        {
            let mut i = 0.0_f64;
            while (i < (breakpoints.len() as f64)) {
                let bp = breakpoints[i as usize].clone();
                if ((t <= bp.end) || (i == ((breakpoints.len() as f64) - 1.0_f64))) {
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
                    return (bp.ease)(clamped_t);
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
        return (segments[((segments.len() as f64) - 1.0_f64) as usize]
            .clone()
            .ease)(1.0_f64);
    });
}
