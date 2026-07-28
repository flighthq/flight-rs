// @generated from upstream/packages/types/src/MotionBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MotionBlurEffect.ts:3 (sha256:987dba8dc376afdab57b3c97754e91e81a7ebde80eeb691a33e8c65372ea55af)
#[derive(Clone)]
pub struct MotionBlurEffect {
    pub kind: String,
    pub intensity: Option<f64>,
    pub samples: Option<f64>,
}
