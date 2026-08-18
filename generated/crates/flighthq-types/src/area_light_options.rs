// @generated from upstream/packages/types/src/AreaLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3Like;

// Source: upstream/packages/types/src/AreaLightOptions.ts:5 (sha256:2692ed3168285d890bf7abb2054c889a2ddeeaf73b7ed333349c732c3705c662)
#[derive(Clone, Default)]
pub struct AreaLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub intensity: Option<f64>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub right: Option<Vector3Like>,
    pub shadow_bias: Option<f64>,
    pub up: Option<Vector3Like>,
}
impl PartialEq for AreaLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
