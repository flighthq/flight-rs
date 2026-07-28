// @generated from upstream/packages/types/src/LookupTableGradeAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ColorLut, ColorTransformFunction};

// Source: upstream/packages/types/src/LookupTableGradeAdjustment.ts:8 (sha256:f1a84a04d9cec19663083fdc5a76a946848551131d27504f51299542a1265898)
#[derive(Clone)]
pub struct LookupTableGradeAdjustment {
    pub kind: String,
    pub transform: ColorTransformFunction,
    pub lut: Option<ColorLut>,
    pub strength: Option<f64>,
}
