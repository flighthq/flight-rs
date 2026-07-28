// @generated from upstream/packages/easing/src/easeExponential.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeExponential.ts:3 (sha256:740b32ccdc4c365bc53aeabf178b1854792e66bca8d29de125f4b50ee7a72258)
pub fn ease_in_exponential(t: f64) -> f64 {
    return if (t == 0.0_f64) {
        0.0_f64
    } else {
        (2.0_f64).powf(((10.0_f64 * t) - 10.0_f64))
    };
}

// Source: upstream/packages/easing/src/easeExponential.ts:5 (sha256:7fe95e8aec832adf0cafe1d747e30ee786403115946c43dde0e8eb9268ada36d)
pub fn ease_in_out_exponential(t: f64) -> f64 {
    if ((t == 0.0_f64) || (t == 1.0_f64)) {
        return t;
    }
    return if (t < 0.5_f64) {
        ((2.0_f64).powf(((20.0_f64 * t) - 10.0_f64)) / 2.0_f64)
    } else {
        ((2.0_f64 - (2.0_f64).powf((((-20.0_f64) * t) + 10.0_f64))) / 2.0_f64)
    };
}

// Source: upstream/packages/easing/src/easeExponential.ts:10 (sha256:7f2883c5895272688f111bdaa62f3826997559bf4d5faaaf93c445113fa4d0f0)
pub fn ease_out_exponential(t: f64) -> f64 {
    return if (t == 1.0_f64) {
        1.0_f64
    } else {
        (1.0_f64 - (2.0_f64).powf(((-10.0_f64) * t)))
    };
}
