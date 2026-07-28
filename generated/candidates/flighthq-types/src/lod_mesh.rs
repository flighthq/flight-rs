// @generated from upstream/packages/types/src/LodMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Mesh, NodeData, Quaternion, SceneNodeRuntime, Vector3};

// Source: upstream/packages/types/src/LodMesh.ts:3 (sha256:7de356902d9f6fa19ea37b1b481cc713ad9031657c6d565f6ceb4d292413bb5d)
#[derive(Clone)]
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

// Source: upstream/packages/types/src/LodMesh.ts:7 (sha256:f20298e4daa6fca2ab54b6ee21e6f74a40612b8e57a3a28744270fcf0a4666c4)
#[derive(Clone)]
pub struct LodMesh {
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
    pub active_level_index: f64,
    pub levels: Vec<LodLevel>,
}
impl PartialEq for LodMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LodMesh.ts:11 (sha256:a786ee9098356714a4072e39afe89f5bdf4f8d0df9b7fa58b45d84ecc9687a64)
pub type LodMeshRuntime = SceneNodeRuntime;

// Source: upstream/packages/types/src/LodMesh.ts:12 (sha256:50ee97b4167eb730b791b333b6af2cb82bc02054b0d604e71702fd80de52fd90)
pub const LOD_MESH_KIND: &'static str = "LodMesh";
