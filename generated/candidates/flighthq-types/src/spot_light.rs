// @generated from upstream/packages/types/src/SpotLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::CubeTexture;
use crate::{Kind, Vector3};

// Source: upstream/packages/types/src/SpotLight.ts:8 (sha256:0edd1b2af064957ca6052730fdd198fd35252f7c85d90270daca821babeaf691)
#[derive(Clone, Default)]
pub struct SpotLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub casts_shadow: bool,
    pub color: f64,
    pub direction: Vector3,
    pub inner_cone_cos: f64,
    pub intensity: f64,
    pub normal_bias: f64,
    pub outer_cone_cos: f64,
    pub pcf_radius: f64,
    pub position: Vector3,
    pub range: f64,
    pub shadow_bias: f64,
    pub ground_color: f64,
    pub sky_color: f64,
    pub environment: Option<CubeTexture>,
    pub right: Vector3,
    pub up: Vector3,
}
impl PartialEq for SpotLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpotLight.ts:23 (sha256:73bb7d27ca0a0a47cb52a7fc69a43e7d2dc3fc9800a4ca1a4f05304bd8810c98)
pub const SPOT_LIGHT_KIND: &'static str = "SpotLight";
