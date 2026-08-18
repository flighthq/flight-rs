// @generated from upstream/packages/types/src/GlUnlitProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlUnlitProgram.ts:6 (sha256:0f2616b769eaaf92696448aaa31de31a4000b4a57a45a911ae7f2835918b3b43)
#[derive(Clone, Default)]
pub struct GlUnlitDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_color_map: bool,
    pub has_skin: Option<bool>,
    pub has_uv_transform: bool,
    pub vertex_color: bool,
}
impl PartialEq for GlUnlitDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlUnlitProgram.ts:21 (sha256:b6dabb6a67295f3fde8a10576757919722c6c2769695c3ba5f9d2e71fb817f78)
#[derive(Clone, Default)]
pub struct GlUnlitProgram {
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
    pub loc_alpha_cutoff: Option<crate::OpaqueHostValue>,
    pub loc_color: Option<crate::OpaqueHostValue>,
    pub loc_color_map: Option<crate::OpaqueHostValue>,
    pub loc_intensity: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlUnlitProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
