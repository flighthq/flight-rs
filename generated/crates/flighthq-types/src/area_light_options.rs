// @generated from upstream/packages/types/src/AreaLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{LightUnit, Vector3Like};

// Source: upstream/packages/types/src/AreaLightOptions.ts:6 (sha256:0b4525699938a5628140650ab2034e8bad788875786387d90d814ad0d61e3d24)
#[derive(Clone, Default)]
pub struct AreaLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub decay: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub enabled: Option<bool>,
    pub intensity: Option<f64>,
    pub intensity_unit: Option<LightUnit>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub right: Option<Vector3Like>,
    pub shadow_bias: Option<f64>,
    pub shadow_far: Option<f64>,
    pub shadow_map_size: Option<f64>,
    pub shadow_near: Option<f64>,
    pub shadow_strength: Option<f64>,
    pub up: Option<Vector3Like>,
}
impl PartialEq for AreaLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
