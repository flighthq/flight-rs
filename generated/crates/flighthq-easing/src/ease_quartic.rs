// @generated from upstream/packages/easing/src/easeQuartic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeQuartic.ts:3 (sha256:2f5076840ae1205843b6e03f1ff8a3f82ff4053555b661656556eb8f17d2a100)
pub fn ease_in_out_quartic(t: f64) -> f64 {
    return if (t < 0.5_f64) {
        ((((8.0_f64 * t) * t) * t) * t)
    } else {
        (1.0_f64 - ((((-2.0_f64) * t) + 2.0_f64).powf(4.0_f64) / 2.0_f64))
    };
}

// Source: upstream/packages/easing/src/easeQuartic.ts:5 (sha256:406a9e9ac56b1f8fbe4fba323e480953f4e3ad232d36c02769d73d72748a7fc5)
pub fn ease_in_quartic(t: f64) -> f64 {
    return (((t * t) * t) * t);
}

// Source: upstream/packages/easing/src/easeQuartic.ts:7 (sha256:34f70c9ce57bdfda094f377c9399f55c0df516c99d73529a4c31cb2097bc3624)
pub fn ease_out_quartic(t: f64) -> f64 {
    return (1.0_f64 - (1.0_f64 - t).powf(4.0_f64));
}
