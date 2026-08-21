// @generated from upstream/packages/types/src/Scene3DRenderProxy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ColorScaleBias, Material, Matrix3, Matrix4, MeshSubset};

// Source: upstream/packages/types/src/Scene3DRenderProxy.ts:28 (sha256:23b508e780cb7961f22f26d996610340b3542df2d5804c493ad65292e48a3e68)
#[derive(Clone, Default)]
pub struct Scene3DRenderProxy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub color_scale_bias: Option<ColorScaleBias>,
    pub color_matrix: Option<Vec<f64>>,
    pub joint_matrices: Option<Vec<f32>>,
    pub normal_matrices: Option<Vec<f32>>,
    pub material: Material,
    pub normal_matrix: Matrix3,
    pub subset: MeshSubset,
    pub world_matrix: Matrix4,
}
impl PartialEq for Scene3DRenderProxy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
