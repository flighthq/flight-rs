// @generated from upstream/packages/types/src/Camera3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Matrix4, Vector2};

// Source: upstream/packages/types/src/Camera3D.ts:11 (sha256:e8fd975d89ced7cc9d122fc1e0d3a4727601a9a8e6770385df067b7719867912)
#[derive(Clone)]
pub struct Camera3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub far: f64,
    pub inverse_view_projection: Matrix4,
    pub jitter: Vector2,
    pub near: f64,
    pub projection: Projection,
    pub view: Matrix4,
}
impl PartialEq for Camera3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Camera3D {
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

// Source: upstream/packages/types/src/Camera3D.ts:20 (sha256:823c7c5a16154958111d17aa69e89c7762028324aeca577cbff07d493d6240c1)
pub type Camera3DLike = Camera3D;

// Source: upstream/packages/types/src/Camera3D.ts:23 (sha256:21f9a9a587e79b904172f7c12a21872323b14399b691065d3e5e2227b3d88e20)
pub type Projection = crate::FlightUnion2<OrthographicProjection, PerspectiveProjection>;

// Source: upstream/packages/types/src/Camera3D.ts:27 (sha256:e3adb55f5918c927447f09149b2a6a9d9abee406e4abe3295085b6e634017e2a)
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

// Source: upstream/packages/types/src/Camera3D.ts:35 (sha256:e14f2b29ed5efd13226a5e9ca3bfc45e4904ee84f8ff0c7839a3de1b4e006497)
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
