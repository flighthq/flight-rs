// @generated from upstream/packages/types/src/SceneRenderProxy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Material, Matrix3, Matrix4, MeshSubset};

// Source: upstream/packages/types/src/SceneRenderProxy.ts:27 (sha256:ef71456cb51dc060753a40631451e1faaa529a884a46a2d05160fd482624f090)
#[derive(Clone)]
pub struct SceneRenderProxy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub joint_matrices: Option<Vec<f32>>,
    pub material: Material,
    pub normal_matrix: Matrix3,
    pub subset: MeshSubset,
    pub world_matrix: Matrix4,
}
impl PartialEq for SceneRenderProxy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
