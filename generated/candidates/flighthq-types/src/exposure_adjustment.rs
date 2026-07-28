// @generated from upstream/packages/types/src/ExposureAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ExposureAdjustment.ts:7 (sha256:6a3ba44800f4e773b8ffc9b4398d9fe0abfe4614bac29eb81a0f941426893814)
#[derive(Clone)]
pub struct ExposureAdjustment {
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub exposure: Option<f64>,
}
