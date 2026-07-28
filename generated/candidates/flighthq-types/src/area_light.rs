// @generated from upstream/packages/types/src/AreaLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3;

// Source: upstream/packages/types/src/AreaLight.ts:7 (sha256:9ea86c550f139c78db1e1e5f74465c7b5551ae23b4269fa6f342fabefe27471a)
#[derive(Clone)]
pub struct AreaLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub casts_shadow: bool,
    pub color: f64,
    pub direction: Vector3,
    pub intensity: f64,
    pub normal_bias: f64,
    pub pcf_radius: f64,
    pub position: Vector3,
    pub range: f64,
    pub right: Vector3,
    pub shadow_bias: f64,
    pub up: Vector3,
}
impl PartialEq for AreaLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AreaLight.ts:22 (sha256:21690015a79821fc4547e0dc9f64d013890ae35719b1ac62b5204730b6e69d00)
pub const AREA_LIGHT_KIND: &'static str = "AreaLight";
