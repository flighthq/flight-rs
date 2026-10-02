// @generated from upstream/packages/types/src/InstancedMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Aabb, EntityRuntime, InstancedMeshSignals, Kind, Material3D, Matrix4, MeshGeometry, NodeData,
    Quaternion, Vector3,
};

// Source: upstream/packages/types/src/InstancedMesh.ts:7 (sha256:7561e0f97161bbe396fc3fc57ddab3c0c08dd507bc641bafe19293e8e1234670)
#[derive(Clone, Default)]
pub struct InstancedMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
    pub materials: Vec<Option<Material3D>>,
    pub version: f64,
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

// Source: upstream/packages/types/src/InstancedMesh.ts:29 (sha256:d12f054f4bb9d97e39bb1842bc34dc1490ef797f850cb747e922cf4b6b9047df)
#[derive(Clone, Default)]
pub struct InstancedMeshCullRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_local_bounds: Option<Aabb>,
    pub instance_local_bounds_version: Option<f64>,
}
impl PartialEq for InstancedMeshCullRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InstancedMesh.ts:37 (sha256:526c39733dec453ea844c1eecbf845a55cf0f4a453f2bddf2c43fc2064f607b5)
#[derive(Clone, Default)]
pub struct InstancedMeshSignalsRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instanced_mesh_signals: Option<InstancedMeshSignals>,
}
impl PartialEq for InstancedMeshSignalsRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InstancedMesh.ts:41 (sha256:549cb7ce407e97722a3629820a466797923a53da59a182b8245669515a81b705)
pub type InstancedMeshRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/InstancedMesh.ts:42 (sha256:3b16f50f0d50be0e7776dd017914d125dea366a0073f49b884831fc999793fb9)
pub const INSTANCED_MESH_KIND: &'static str = "InstancedMesh";
