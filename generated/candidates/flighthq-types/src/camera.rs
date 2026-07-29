// @generated from upstream/packages/types/src/Camera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Matrix4, Vector2};

// Source: upstream/packages/types/src/Camera.ts:12 (sha256:5b25d9f44809b168101850d099cc8f5e2ff9f30356638216b00fbd21e08fce43)
#[derive(Clone)]
pub struct Camera {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub far: f64,
    pub inverse_view_projection: Matrix4,
    pub jitter: Vector2,
    pub near: f64,
    pub projection: Projection,
    pub view: Matrix4,
}
impl PartialEq for Camera {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Camera {
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

// Source: upstream/packages/types/src/Camera.ts:21 (sha256:b7d45d66db148544041c5acf5f092944f4890be57ce36e2b8b2cce26d507c557)
pub type CameraLike = Camera;

// Source: upstream/packages/types/src/Camera.ts:24 (sha256:21f9a9a587e79b904172f7c12a21872323b14399b691065d3e5e2227b3d88e20)
pub type Projection = crate::FlightUnion2<OrthographicProjection, PerspectiveProjection>;

// Source: upstream/packages/types/src/Camera.ts:28 (sha256:e3adb55f5918c927447f09149b2a6a9d9abee406e4abe3295085b6e634017e2a)
#[derive(Clone, Default)]
pub struct PerspectiveProjection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub aspect: f64,
    pub fov_y: f64,
    pub kind: String,
}
impl PartialEq for PerspectiveProjection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Camera.ts:36 (sha256:e14f2b29ed5efd13226a5e9ca3bfc45e4904ee84f8ff0c7839a3de1b4e006497)
#[derive(Clone, Default)]
pub struct OrthographicProjection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub half_height: f64,
    pub half_width: f64,
    pub kind: String,
}
impl PartialEq for OrthographicProjection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
