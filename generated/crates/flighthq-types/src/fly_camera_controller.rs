// @generated from upstream/packages/types/src/FlyCameraController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Vector3, Vector3Like};

// Source: upstream/packages/types/src/FlyCameraController.ts:13 (sha256:5ec7dbb9000ec57efedee41fc853a74e39bf8f7b229155779007e9579f9407b7)
#[derive(Clone, Default)]
pub struct FlyCameraController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub goal_pitch: f64,
    pub goal_yaw: f64,
    pub max_pitch: f64,
    pub min_pitch: f64,
    pub pitch: f64,
    pub position: Vector3,
    pub smooth_time: f64,
    pub yaw: f64,
}
impl PartialEq for FlyCameraController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FlyCameraController {
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

// Source: upstream/packages/types/src/FlyCameraController.ts:26 (sha256:bd85b1817bcc1e551af340721e5efe989bc68b5d35234abedbb53e229cf8d9d4)
#[derive(Clone, Default)]
pub struct FlyCameraControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_pitch: Option<f64>,
    pub min_pitch: Option<f64>,
    pub pitch: Option<f64>,
    pub position: Option<Vector3Like>,
    pub smooth_time: Option<f64>,
    pub yaw: Option<f64>,
}
impl PartialEq for FlyCameraControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
