// @generated from upstream/packages/types/src/LodMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, Mesh, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/LodMesh.ts:3 (sha256:7de356902d9f6fa19ea37b1b481cc713ad9031657c6d565f6ceb4d292413bb5d)
#[derive(Clone, Default)]
pub struct LodLevel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mesh: Mesh,
    pub min_distance: f64,
}
impl PartialEq for LodLevel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LodMesh.ts:7 (sha256:8278a6eef11a26e8d7f9edb0096080c4a1d2e04222097804093fcb7f2a78247c)
#[derive(Clone, Default)]
pub struct LodMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub active_level_index: f64,
    pub levels: Vec<LodLevel>,
}
impl PartialEq for LodMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for LodMesh {
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

// Source: upstream/packages/types/src/LodMesh.ts:11 (sha256:60287a96e475211f1b5acb3399fe6057d3096897d5553ad2ef8d1794adb98228)
pub type LodMeshRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/LodMesh.ts:12 (sha256:50ee97b4167eb730b791b333b6af2cb82bc02054b0d604e71702fd80de52fd90)
pub const LOD_MESH_KIND: &'static str = "LodMesh";
