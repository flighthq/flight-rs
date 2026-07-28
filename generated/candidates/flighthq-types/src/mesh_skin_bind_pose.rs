// @generated from upstream/packages/types/src/MeshSkinBindPose.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MeshSkinBindPose.ts:10 (sha256:736e4bf4ec95bf4937e25e58fcd614373bff9626fbfd4431685b2b025bd9f67c)
#[derive(Clone)]
pub struct MeshSkinBindPose {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joints: Vec<f32>,
    pub normals: Vec<f32>,
    pub positions: Vec<f32>,
    pub skinned_normals: Vec<f32>,
    pub skinned_positions: Vec<f32>,
    pub weights: Vec<f32>,
}
impl PartialEq for MeshSkinBindPose {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
