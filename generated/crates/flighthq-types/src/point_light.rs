// @generated from upstream/packages/types/src/PointLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;
use crate::{EntityRuntime, Kind, LightUnit, Vector3};

// Source: upstream/packages/types/src/PointLight.ts:14 (sha256:1f5e56b1211f2d6b779341e86a42afbc31db1687e765ea67916b658a0b0362e2)
#[derive(Clone, Default)]
pub struct PointLight {
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
impl PartialEq for PointLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for PointLight {
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

// Source: upstream/packages/types/src/PointLight.ts:50 (sha256:7cae53fe66853284c726ca2a459e1cd12fae6d97de6a22790b09f0feadf23d18)
pub const POINT_LIGHT_KIND: &'static str = "PointLight";
