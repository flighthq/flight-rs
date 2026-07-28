// @generated from upstream/packages/easing/src/easeCubic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeCubic.ts:3 (sha256:7973d79913a0a59c432dded763b2007b6e663a23b1da1b9d53a6532a45b0582c)
pub fn ease_in_cubic(t: f64) -> f64 {
    return ((t * t) * t);
}

// Source: upstream/packages/easing/src/easeCubic.ts:5 (sha256:82b344a64e40460bd3e6d1ae1acfe6fb6a43223058fd8efeaa1a5d2c64b6338b)
pub fn ease_in_out_cubic(t: f64) -> f64 {
    return if (t < 0.5_f64) {
        (((4.0_f64 * t) * t) * t)
    } else {
        (1.0_f64 - ((((-2.0_f64) * t) + 2.0_f64).powf(3.0_f64) / 2.0_f64))
    };
}

// Source: upstream/packages/easing/src/easeCubic.ts:7 (sha256:5aba73b3452f308c510d291aa9a7d9d759f8c40bdc0957791d52b425df28d29e)
pub fn ease_out_cubic(t: f64) -> f64 {
    return (1.0_f64 - (1.0_f64 - t).powf(3.0_f64));
}
