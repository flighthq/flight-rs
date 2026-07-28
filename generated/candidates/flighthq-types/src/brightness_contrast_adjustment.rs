// @generated from upstream/packages/types/src/BrightnessContrastAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BrightnessContrastAdjustment.ts:3 (sha256:caf6f67d86b94c68417c4e4fb33eb7d14bc9a5242a1428d63c6b528c4e037b27)
#[derive(Clone)]
pub struct BrightnessContrastAdjustment {
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
