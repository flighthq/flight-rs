// @generated from upstream/packages/types/src/ColorGradeAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorTransformFunction;

// Source: upstream/packages/types/src/ColorGradeAdjustment.ts:7 (sha256:4d2fcb20eebdaf4b024c5f482ff5a085aa1fb61b67d6864f6de8e538d961e7ac)
#[derive(Clone)]
pub struct ColorGradeAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub transform: ColorTransformFunction,
    pub exposure: Option<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub saturation: Option<f64>,
    pub temperature: Option<f64>,
    pub tint: Option<f64>,
    pub lift: Option<f64>,
    pub gamma: Option<f64>,
    pub gain: Option<f64>,
}
impl PartialEq for ColorGradeAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
