// @generated from upstream/packages/types/src/GlMatcapProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlMatcapProgram.ts:6 (sha256:0e26e6a9c5208d84322aef5a30d835b97696c997ee519df9eef1629233eff097)
#[derive(Clone, Default)]
pub struct GlMatcapDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_instances: Option<bool>,
    pub has_matcap: bool,
    pub has_skin: Option<bool>,
}
impl PartialEq for GlMatcapDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlMatcapProgram.ts:18 (sha256:97bd638a633ce260e33310e914fb6e0a453ebc2627245c8c8bf2e0e9838137c6)
#[derive(Clone, Default)]
pub struct GlMatcapProgram {
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
    pub loc_instance_palette: Option<crate::OpaqueHostValue>,
    pub loc_instance_color_palette: Option<crate::OpaqueHostValue>,
    pub loc_joint_normal_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_alpha_cutoff: Option<crate::OpaqueHostValue>,
    pub loc_matcap: Option<crate::OpaqueHostValue>,
    pub loc_tint: Option<crate::OpaqueHostValue>,
    pub loc_view: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlMatcapProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
