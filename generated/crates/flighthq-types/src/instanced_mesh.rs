// @generated from upstream/packages/types/src/InstancedMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, Material, Matrix4, MeshGeometry, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/InstancedMesh.ts:5 (sha256:fc19257d964eee709bcf7fe8343c6d0f90d037f7772f7c8c9ff55e2b37fe4fb2)
#[derive(Clone, Default)]
pub struct InstancedMesh {
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
    pub geometry: MeshGeometry,
    pub instance_colors: Option<Vec<u32>>,
    pub instance_count: f64,
    pub instance_matrices: Vec<Matrix4>,
    pub materials: Vec<Option<Material>>,
}
impl PartialEq for InstancedMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for InstancedMesh {
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

// Source: upstream/packages/types/src/InstancedMesh.ts:12 (sha256:afdb316830da07b5dca13b153fbd7dba94fb6d8d6793d0bc89f4405d99e4dd08)
pub type InstancedMeshRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/InstancedMesh.ts:13 (sha256:3b16f50f0d50be0e7776dd017914d125dea366a0073f49b884831fc999793fb9)
pub const INSTANCED_MESH_KIND: &'static str = "InstancedMesh";
