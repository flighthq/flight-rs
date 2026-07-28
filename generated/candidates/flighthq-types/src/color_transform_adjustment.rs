// @generated from upstream/packages/types/src/ColorTransformAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorTransform;

// Source: upstream/packages/types/src/ColorTransformAdjustment.ts:10 (sha256:c31fc4c470375aaa6b620ec0028148ba1a6cbcfc3eca340bff502ed3fa6a0899)
#[derive(Clone)]
pub struct ColorTransformAdjustment {
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub color_transform: ColorTransform,
}
