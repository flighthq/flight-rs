// @generated from upstream/packages/types/src/HueSaturationAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorTransformFunction;

// Source: upstream/packages/types/src/HueSaturationAdjustment.ts:3 (sha256:3640104d1343b3a9cc0acac40751d37bdc920fd0885d3c2f490fc75339160109)
#[derive(Clone)]
pub struct HueSaturationAdjustment {
    pub kind: String,
    pub transform: ColorTransformFunction,
    pub hue: Option<f64>,
    pub saturation: Option<f64>,
    pub lightness: Option<f64>,
}
