// @generated from upstream/packages/types/src/SpotLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3Like;

// Source: upstream/packages/types/src/SpotLightOptions.ts:5 (sha256:d0bf1578d5df68cbb0e862e25e3ac2b9d7a0141cb8461c4a9226d61baecee179)
#[derive(Clone, Default)]
pub struct SpotLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub inner_cone_degrees: Option<f64>,
    pub intensity: Option<f64>,
    pub normal_bias: Option<f64>,
    pub outer_cone_degrees: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub shadow_bias: Option<f64>,
}
impl PartialEq for SpotLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
