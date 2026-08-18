// @generated from upstream/packages/types/src/Mesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Aabb, EntityRuntime, Kind, Material, MeshGeometry, MeshMorph, NodeData, Quaternion, Skin,
    Vector3,
};

// Source: upstream/packages/types/src/Mesh.ts:26 (sha256:7706a57bd2ba7e0313a79f3f77ee6031a1740fc17da12c42efc4657bbd66b507)
#[derive(Clone, Default)]
pub struct Mesh {
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
    pub materials: Vec<Option<Material>>,
    pub morph: Option<MeshMorph>,
    pub skin: Option<Skin>,
}
impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Mesh {
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

// Source: upstream/packages/types/src/Mesh.ts:48 (sha256:309789facdca8259b69791b1d2d86ca8fe8f35b14f6cc88f314f08ba78fc465b)
#[derive(Clone, Default)]
pub struct MeshDeformRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deformed_local_bounds: Option<Aabb>,
}
impl PartialEq for MeshDeformRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Mesh.ts:52 (sha256:11b601c164d5cf768ec952596383dfe3f29bb1929edc700cea4c4abc977c50dd)
pub type MeshRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Mesh.ts:54 (sha256:a8f5349b3e25c2229bba5168e8dd9eded0976e020eec223a302de6e83425c9e4)
pub const MESH_KIND: &'static str = "Mesh";
