// @generated from upstream/packages/types/src/Environment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3;
use crate::{CubeTexture, EntityRuntime, Kind};

// Source: upstream/packages/types/src/Environment.ts:6 (sha256:37d6730ce4b1aa4065299ff5ae1ab3d22a8d91deca205f4b503605f00ff3a1ed)
#[derive(Clone, Default)]
pub struct Environment {
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
impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Environment {
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

// Source: upstream/packages/types/src/Environment.ts:12 (sha256:65722e8f1a29f53e8a8e974096ff0f2e8d794d9f898fcbc9e8cea99357346190)
pub const ENVIRONMENT_KIND: &'static str = "Environment";
