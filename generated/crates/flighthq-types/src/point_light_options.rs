// @generated from upstream/packages/types/src/PointLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3Like;

// Source: upstream/packages/types/src/PointLightOptions.ts:5 (sha256:35ec8d4b09d8b5e4ec4a3bb1b4cc4df6cae2b1df2ee5ff01c43296dc64029d2c)
#[derive(Clone, Default)]
pub struct PointLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub shadow_bias: Option<f64>,
}
impl PartialEq for PointLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
