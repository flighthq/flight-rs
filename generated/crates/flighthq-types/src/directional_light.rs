// @generated from upstream/packages/types/src/DirectionalLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;
use crate::{EntityRuntime, Kind, Vector3};

// Source: upstream/packages/types/src/DirectionalLight.ts:10 (sha256:5aa15d73a4d69dda6f617f278e05d90700a178b45474ec248e36e1a1139373ae)
#[derive(Clone, Default)]
pub struct DirectionalLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
    pub environment: Option<Texture>,
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
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/DirectionalLight.ts:28 (sha256:14d67d7a33aa627cd20f5f9d8654dc1da943a0d5920a3d62d8399d764b3550de)
pub const DIRECTIONAL_LIGHT_KIND: &'static str = "DirectionalLight";

// Source: upstream/packages/types/src/DirectionalLight.ts:33 (sha256:85457b1393209f4be48e2db03c298d8f3a8ddfe60d529fcdb0f29f1a820f6252)
pub const DIRECTIONAL_SHADOW_MAP_SIZE: f64 = 1024.0_f64;

// Source: upstream/packages/types/src/DirectionalLight.ts:38 (sha256:073a5056125967550050c76ee63e58887241d92e4802f61370e9f9a30006ffe7)
pub const MAX_DIRECTIONAL_SHADOW_PCF_RADIUS: f64 = 2.0_f64;
