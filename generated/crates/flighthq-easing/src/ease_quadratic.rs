// @generated from upstream/packages/easing/src/easeQuadratic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeQuadratic.ts:3 (sha256:89cf6fbf576047813ed27aac439952d95470a58e901e99fcbaa657b722710ca1)
pub fn ease_in_out_quadratic(t: f64) -> f64 {
    return if (t < 0.5_f64) {
        ((2.0_f64 * t) * t)
    } else {
        (1.0_f64 - ((((-2.0_f64) * t) + 2.0_f64).powf(2.0_f64) / 2.0_f64))
    };
}

// Source: upstream/packages/easing/src/easeQuadratic.ts:5 (sha256:082f5fe836d7d067377d4e139c8db422f6a05d53697c298ad4c27e522818814f)
pub fn ease_in_quadratic(t: f64) -> f64 {
    return (t * t);
}

// Source: upstream/packages/easing/src/easeQuadratic.ts:7 (sha256:8b3aa6c2ae649abdc632e85ef90b706807ba0c5c45a84eedb58461aedc3bbd38)
pub fn ease_out_quadratic(t: f64) -> f64 {
    return (t * (2.0_f64 - t));
}
