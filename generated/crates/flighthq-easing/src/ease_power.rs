// @generated from upstream/packages/easing/src/easePower.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/easing/src/easePower.ts:6 (sha256:bea4c7c6c1180598b2753000171fca1d6d6cdad1dfd3948103316549a3fa280f)
pub fn ease_in_out_power(exponent: f64) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 {
        if (t < 0.5_f64) {
            return ((t * 2.0_f64).powf(exponent) * 0.5_f64);
        }
        return (1.0_f64 - (((1.0_f64 - t) * 2.0_f64).powf(exponent) * 0.5_f64));
    });
}

// Source: upstream/packages/easing/src/easePower.ts:17 (sha256:e66af3c41951c6632e6e566264cb4b48b45a3cdd8cdfa7945926ff40e95a08f0)
pub fn ease_in_power(exponent: f64) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 { (t).powf(exponent) });
}

// Source: upstream/packages/easing/src/easePower.ts:24 (sha256:93c8d2959fbc647adda23200659133a7104d6f0eab4bbfd31867654b3510c6da)
pub fn ease_out_power(exponent: f64) -> EasingFunction {
    return std::sync::Arc::new(move |t: f64| -> f64 { (1.0_f64 - (1.0_f64 - t).powf(exponent)) });
}
