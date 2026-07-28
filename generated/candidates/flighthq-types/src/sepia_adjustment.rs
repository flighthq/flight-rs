// @generated from upstream/packages/types/src/SepiaAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SepiaAdjustment.ts:3 (sha256:a83e972503e43391f11ed9b702ec92cb0573028669784f134f2dcfcc50090790)
#[derive(Clone)]
pub struct SepiaAdjustment {
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub intensity: Option<f64>,
}
