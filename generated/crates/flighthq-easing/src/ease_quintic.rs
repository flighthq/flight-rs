// @generated from upstream/packages/easing/src/easeQuintic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeQuintic.ts:3 (sha256:e888b480ae526a0c878ab944b99b34fa73345fcd47de041f46d222425191f162)
pub fn ease_in_out_quintic(t: f64) -> f64 {
    return if (t < 0.5_f64) {
        (((((16.0_f64 * t) * t) * t) * t) * t)
    } else {
        (1.0_f64 - ((((-2.0_f64) * t) + 2.0_f64).powf(5.0_f64) / 2.0_f64))
    };
}

// Source: upstream/packages/easing/src/easeQuintic.ts:6 (sha256:abb1afecf71bd32dc41c67a3dc476c26415d48e9adbfb8c91ddbd3493456de01)
pub fn ease_in_quintic(t: f64) -> f64 {
    return ((((t * t) * t) * t) * t);
}

// Source: upstream/packages/easing/src/easeQuintic.ts:8 (sha256:9b0cfa30bd2185c91d351d1e8475c16b23871c2acc79cdf722c2c474ebb5e685)
pub fn ease_out_quintic(t: f64) -> f64 {
    return (1.0_f64 - (1.0_f64 - t).powf(5.0_f64));
}
