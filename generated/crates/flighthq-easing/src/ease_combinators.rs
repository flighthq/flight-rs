// @generated from upstream/packages/easing/src/easeCombinators.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/easing/src/easeCombinators.ts:16 (sha256:6320e25c24e62c87988aad0e6fa64915417d9bd5b9eb2ce6abcaccf5d73d3497)
pub fn ease_clamp(ease: EasingFunction) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 {
        ease(if (t < 0.0_f64) {
            0.0_f64
        } else {
            if (t > 1.0_f64) { 1.0_f64 } else { t }
        })
    });
}

// Source: upstream/packages/easing/src/easeCombinators.ts:23 (sha256:bd1a0bef3840e939b815abcdf0c6c2e7ea25e427e892e12ce0613eb443b23d15)
pub fn ease_clamp_output(ease: EasingFunction, min: f64, max: f64) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 {
        let v = ease(t);
        return if (v < min) {
            min
        } else {
            if (v > max) { max } else { v }
        };
    });
}

// Source: upstream/packages/easing/src/easeCombinators.ts:33 (sha256:4c231b0071a36f2845f4bacc910312225d1e7be68ebeddf87cb3df83794004e0)
pub fn ease_invert(ease: EasingFunction) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 { (1.0_f64 - ease(t)) });
}

// Source: upstream/packages/easing/src/easeCombinators.ts:41 (sha256:7a72a5a8ef5b8bbde20087f361b5f517a9e0151b137657ba48b360b4bd3be2d6)
pub fn ease_mirror(ease_in: EasingFunction) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 {
        if (t < 0.5_f64) {
            return (ease_in((t * 2.0_f64)) * 0.5_f64);
        }
        return (1.0_f64 - (ease_in(((1.0_f64 - t) * 2.0_f64)) * 0.5_f64));
    });
}

// Source: upstream/packages/easing/src/easeCombinators.ts:50 (sha256:b64786214ad38808ae43a453a72cba4f1f7bf6b22635ddaec40fc41689146780)
pub fn ease_reverse(ease_in: EasingFunction) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 { (1.0_f64 - ease_in((1.0_f64 - t))) });
}

// Source: upstream/packages/easing/src/easeCombinators.ts:57 (sha256:4632554ac3b23bf27ebe2ce47aa7ed5d65f57202988304442812d02d29443281)
pub fn ease_scale_output(ease: EasingFunction, from_value: f64, to_value: f64) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 {
        (from_value + (ease(t) * (to_value - from_value)))
    });
}
