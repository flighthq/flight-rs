// @generated from upstream/packages/types/src/InvertAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InvertAdjustment.ts:3 (sha256:21b60731b36b15786ea3aef5f0a4f47736131c14b0ecedd0e2e742ca4c14b308)
#[derive(Clone)]
pub struct InvertAdjustment {
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub intensity: Option<f64>,
}
