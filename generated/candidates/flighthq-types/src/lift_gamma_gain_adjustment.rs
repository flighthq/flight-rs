// @generated from upstream/packages/types/src/LiftGammaGainAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorTransformFunction;

// Source: upstream/packages/types/src/LiftGammaGainAdjustment.ts:3 (sha256:4aea66b1032af3c2204ba582da1734c48ae4493f9109db490d69391f9bb66ccf)
#[derive(Clone)]
pub struct LiftGammaGainAdjustment {
    pub kind: String,
    pub transform: ColorTransformFunction,
    pub lift: Option<f64>,
    pub gamma: Option<f64>,
    pub gain: Option<f64>,
}
