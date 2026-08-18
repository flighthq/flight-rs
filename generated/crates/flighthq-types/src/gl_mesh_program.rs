// @generated from upstream/packages/types/src/GlMeshProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlMeshProgram.ts:7 (sha256:25ac84d4effc1f1758fbadbe9e06fde068ef3b4b8fc74721bf1940acb3180003)
#[derive(Clone, Default)]
pub struct GlMeshProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_color_scale: Option<crate::OpaqueHostValue>,
    pub loc_color_bias: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix0: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix1: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix2: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix3: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix_offset: Option<crate::OpaqueHostValue>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_alpha_is_coverage: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
}
impl PartialEq for GlMeshProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
