// @generated from upstream/packages/types/src/Camera2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Rectangle};

// Source: upstream/packages/types/src/Camera2D.ts:16 (sha256:3f18baf39b1a5b3d4ee4a9a799d731ce2b9eee3951d3512590ffc0246dea25c9)
#[derive(Clone, Default)]
pub struct Camera2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub rotation: f64,
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}
impl PartialEq for Camera2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Camera2D {
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

// Source: upstream/packages/types/src/Camera2D.ts:34 (sha256:1a6e6412cf22c4268643adadbc08fab5e10f9911ce8eac4b47b5883e9c739143)
#[derive(Clone, Default)]
pub struct Camera2DFollowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deadzone_half_height: Option<f64>,
    pub deadzone_half_width: Option<f64>,
    pub smooth_time: Option<f64>,
    pub world_bounds: Option<Rectangle>,
}
impl PartialEq for Camera2DFollowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Camera2D.ts:43 (sha256:a2dd28e6be1aa508210a4e0081af1eda684944dbeee1bc32166e096a2bc2dced)
#[derive(Clone, Default)]
pub struct Camera2DOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rotation: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub zoom: Option<f64>,
}
impl PartialEq for Camera2DOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
