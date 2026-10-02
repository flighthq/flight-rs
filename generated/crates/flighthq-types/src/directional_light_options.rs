// @generated from upstream/packages/types/src/DirectionalLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{LightUnit, Vector3Like};

// Source: upstream/packages/types/src/DirectionalLightOptions.ts:4 (sha256:c0a6bf4a005dd31930379a8aa13dc57df33773f78507444760dd135d2914cc2a)
#[derive(Clone, Default)]
pub struct DirectionalLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cascade_count: Option<f64>,
    pub cascade_splits: Option<Vec<f64>>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub enabled: Option<bool>,
    pub intensity: Option<f64>,
    pub intensity_unit: Option<LightUnit>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub shadow_bias: Option<f64>,
    pub shadow_far: Option<f64>,
    pub shadow_map_size: Option<f64>,
    pub shadow_near: Option<f64>,
    pub shadow_strength: Option<f64>,
}
impl PartialEq for DirectionalLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
