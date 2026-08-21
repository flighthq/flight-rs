// @generated from upstream/packages/types/src/OrbitCameraController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Vector3, Vector3Like};

// Source: upstream/packages/types/src/OrbitCameraController.ts:14 (sha256:b5f317c10fcee34f5c8ab37de7d06314754e9bcc0fb48124146a562f9117cb5f)
#[derive(Clone, Default)]
pub struct OrbitCameraController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub azimuth: f64,
    pub distance: f64,
    pub goal_azimuth: f64,
    pub goal_distance: f64,
    pub goal_polar: f64,
    pub max_distance: f64,
    pub max_polar: f64,
    pub min_distance: f64,
    pub min_polar: f64,
    pub polar: f64,
    pub smooth_time: f64,
    pub target: Vector3,
}
impl PartialEq for OrbitCameraController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for OrbitCameraController {
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

// Source: upstream/packages/types/src/OrbitCameraController.ts:31 (sha256:f1dbf387c55015f5b44dbbe97535bcc3591c79fc936b1381180dc3f65b3da869)
#[derive(Clone, Default)]
pub struct OrbitCameraControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub azimuth: Option<f64>,
    pub distance: Option<f64>,
    pub max_distance: Option<f64>,
    pub max_polar: Option<f64>,
    pub min_distance: Option<f64>,
    pub min_polar: Option<f64>,
    pub polar: Option<f64>,
    pub smooth_time: Option<f64>,
    pub target: Option<Vector3Like>,
}
impl PartialEq for OrbitCameraControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
