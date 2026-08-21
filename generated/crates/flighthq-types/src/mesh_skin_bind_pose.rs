// @generated from upstream/packages/types/src/MeshSkinBindPose.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MeshSkinBindPose.ts:14 (sha256:77bf0f172a896ccce04fb27b31e7fdedec6d24293ad0864f737721597d4d0aa7)
#[derive(Clone, Default)]
pub struct MeshSkinBindPose {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joints: Vec<f32>,
    pub normals: Vec<f32>,
    pub positions: Vec<f32>,
    pub skinned_normals: Vec<f32>,
    pub skinned_positions: Vec<f32>,
    pub skinned_tangents: Vec<f32>,
    pub tangents: Vec<f32>,
    pub weights: Vec<f32>,
}
impl PartialEq for MeshSkinBindPose {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
