// @generated from upstream/packages/easing/src/easeSine.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeSine.ts:3 (sha256:f7fba9c04b6ad1f8a63ade98d6b928df0620638c4b039fa3310fee78205cfa4c)
pub fn ease_in_out_sine(t: f64) -> f64 {
    return ((-((std::f64::consts::PI * t).cos() - 1.0_f64)) / 2.0_f64);
}

// Source: upstream/packages/easing/src/easeSine.ts:5 (sha256:4e1aa7a2f5fe318089d2ce8da351aecb9b6bb19520964b61cd1e1849257e3251)
pub fn ease_in_sine(t: f64) -> f64 {
    return (1.0_f64 - ((t * std::f64::consts::PI) / 2.0_f64).cos());
}

// Source: upstream/packages/easing/src/easeSine.ts:7 (sha256:0307941b5976072f29f37d6015688135583b4f04a19fa92b80d1f21870e551e0)
pub fn ease_out_sine(t: f64) -> f64 {
    return ((t * std::f64::consts::PI) / 2.0_f64).sin();
}
