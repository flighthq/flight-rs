// @generated from upstream/packages/types/src/DirectionalLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::CubeTexture;
use crate::{EntityRuntime, Kind, Vector3};

// Source: upstream/packages/types/src/DirectionalLight.ts:6 (sha256:5aa15d73a4d69dda6f617f278e05d90700a178b45474ec248e36e1a1139373ae)
#[derive(Clone, Default)]
pub struct DirectionalLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
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
impl PartialEq for DirectionalLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for DirectionalLight {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/DirectionalLight.ts:17 (sha256:14d67d7a33aa627cd20f5f9d8654dc1da943a0d5920a3d62d8399d764b3550de)
pub const DIRECTIONAL_LIGHT_KIND: &'static str = "DirectionalLight";
