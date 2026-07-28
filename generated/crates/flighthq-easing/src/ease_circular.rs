// @generated from upstream/packages/easing/src/easeCircular.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeCircular.ts:3 (sha256:d38aec60335134ba29d5377bdd378b85b4032a03d45895bc2e8b3104193d6bf0)
pub fn ease_in_circular(t: f64) -> f64 {
    return (1.0_f64 - (1.0_f64 - (t * t)).sqrt());
}

// Source: upstream/packages/easing/src/easeCircular.ts:5 (sha256:4f43dd5b3659ca6b663a7b58a6e55b57520d6bfbf6e255537d0fdf79a8adaee4)
pub fn ease_in_out_circular(t: f64) -> f64 {
    return if (t < 0.5_f64) {
        ((1.0_f64 - (1.0_f64 - ((4.0_f64 * t) * t)).sqrt()) / 2.0_f64)
    } else {
        (((1.0_f64 - (((-2.0_f64) * t) + 2.0_f64).powf(2.0_f64)).sqrt() + 1.0_f64) / 2.0_f64)
    };
}

// Source: upstream/packages/easing/src/easeCircular.ts:8 (sha256:006fa8ab220787f9b11dafac138072e21a514f70bc7de96458e1ed81193f59bb)
pub fn ease_out_circular(t: f64) -> f64 {
    return (1.0_f64 - ((t - 1.0_f64) * (t - 1.0_f64))).sqrt();
}
