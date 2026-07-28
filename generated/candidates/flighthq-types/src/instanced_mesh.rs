// @generated from upstream/packages/types/src/InstancedMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Kind, Material, Matrix4, MeshGeometry, NodeData, Quaternion, SceneNodeRuntime, Vector3,
};

// Source: upstream/packages/types/src/InstancedMesh.ts:5 (sha256:4f4896f9cd96c723d6c18c6df5a192c8a1afec0d5c9d3bee2524ab37607ab205)
#[derive(Clone)]
pub struct InstancedMesh {
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

// Source: upstream/packages/types/src/InstancedMesh.ts:12 (sha256:c04bff99a0216c5367c6862c96ffd3dc91f3604baeded3e4d6188698a4044992)
pub type InstancedMeshRuntime = SceneNodeRuntime;

// Source: upstream/packages/types/src/InstancedMesh.ts:13 (sha256:3b16f50f0d50be0e7776dd017914d125dea366a0073f49b884831fc999793fb9)
pub const INSTANCED_MESH_KIND: &'static str = "InstancedMesh";
