// @generated from upstream/packages/types/src/PointLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{LightUnit, Vector3Like};

// Source: upstream/packages/types/src/PointLightOptions.ts:6 (sha256:aebaa70672a84fe2effda2090ec1dca58a764003c994fdb81a9fac276cc29c79)
#[derive(Clone, Default)]
pub struct PointLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub layer_mask: Option<f64>,
    pub priority: Option<f64>,
    pub color: Option<f64>,
    pub decay: Option<f64>,
    pub enabled: Option<bool>,
    pub intensity: Option<f64>,
    pub intensity_unit: Option<LightUnit>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub shadow_bias: Option<f64>,
    pub shadow_far: Option<f64>,
    pub shadow_map_size: Option<f64>,
    pub shadow_near: Option<f64>,
    pub shadow_strength: Option<f64>,
}
impl PartialEq for PointLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
