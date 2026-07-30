// @generated from upstream/packages/types/src/HasTransform3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/HasTransform3D.ts:12 (sha256:ac6499b869984707d016242a71546b88681eeb1961c741ac5cae8a0e70f63a28)
#[derive(Clone, Default)]
pub struct HasTransform3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for HasTransform3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for HasTransform3D {
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

// Source: upstream/packages/types/src/HasTransform3D.ts:18 (sha256:1d64b827bad156d3d07cd1ca90f99aecb21a157026d06c1920d015992b2d3e2e)
pub type HasTransform3DRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/HasTransform3D.ts:28 (sha256:da9298db6d25709679d19c3277a12efc13fb2ea8c4793a128cb1288dba978dcf)
#[derive(Clone, Default)]
pub struct Transform3DNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for Transform3DNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Transform3DNode {
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
