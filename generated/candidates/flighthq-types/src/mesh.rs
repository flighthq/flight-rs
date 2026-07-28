// @generated from upstream/packages/types/src/Mesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Kind, Material, MeshGeometry, MeshMorph, NodeData, Quaternion, SceneNodeRuntime, Skin, Vector3,
};

// Source: upstream/packages/types/src/Mesh.ts:25 (sha256:9d1fc386d6d46d295994508c69ed48afe084c7671e04527565640f98f319b627)
#[derive(Clone)]
pub struct Mesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Mesh.ts:32 (sha256:9660a362828281d9d02e797dc5269a17867a46e13d811c7cdfdcb91529eab499)
pub type MeshRuntime = SceneNodeRuntime;

// Source: upstream/packages/types/src/Mesh.ts:34 (sha256:a8f5349b3e25c2229bba5168e8dd9eded0976e020eec223a302de6e83425c9e4)
pub const MESH_KIND: &'static str = "Mesh";
