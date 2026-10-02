// @generated from upstream/packages/types/src/Light.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind};
use crate::{LightUnit, Texture, Vector3};

// Source: upstream/packages/types/src/Light.ts:22 (sha256:11378cc025586905b984f2649a5cbdfc57d770ba6450991d9acb4714ea46dae7)
#[derive(Clone, Default)]
pub struct Light {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub casts_shadow: bool,
    pub color: f64,
    pub decay: f64,
    pub direction: Vector3,
    pub enabled: bool,
    pub inner_cone_cos: f64,
    pub intensity: f64,
    pub intensity_unit: LightUnit,
    pub layer_mask: f64,
    pub priority: f64,
    pub normal_bias: f64,
    pub outer_cone_cos: f64,
    pub pcf_radius: f64,
    pub position: Vector3,
    pub range: f64,
    pub shadow_bias: f64,
    pub shadow_far: f64,
    pub shadow_map_size: f64,
    pub shadow_near: f64,
    pub shadow_strength: f64,
    pub spot_blend: f64,
    pub ground_color: f64,
    pub sky_color: f64,
    pub environment: Option<Texture>,
    pub cascade_count: f64,
    pub cascade_splits: Vec<f64>,
    pub right: Vector3,
    pub up: Vector3,
}
impl PartialEq for Light {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Light {
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
