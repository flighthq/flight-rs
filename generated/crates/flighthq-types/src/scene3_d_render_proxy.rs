// @generated from upstream/packages/types/src/Scene3DRenderProxy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ColorScaleBias, Material3D, Matrix3, Matrix4, MeshSubset};

// Source: upstream/packages/types/src/Scene3DRenderProxy.ts:28 (sha256:fa0de87cec9f9e4227e743935b551048e7a72cdb2770c427d9f830c3c55e0ec6)
#[derive(Clone, Default)]
pub struct Scene3DRenderProxy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub color_scale_bias: Option<ColorScaleBias>,
    pub color_matrix: Option<Vec<f64>>,
    pub instance_count: Option<f64>,
    pub instance_matrices: Option<Vec<f32>>,
    pub instance_colors: Option<Vec<f32>>,
    pub joint_matrices: Option<Vec<f32>>,
    pub normal_matrices: Option<Vec<f32>>,
    pub material: Material3D,
    pub normal_matrix: Matrix3,
    pub subset: MeshSubset,
    pub world_matrix: Matrix4,
}
impl PartialEq for Scene3DRenderProxy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
