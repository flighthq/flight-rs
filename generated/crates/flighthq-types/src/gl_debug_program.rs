// @generated from upstream/packages/types/src/GlDebugProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlDebugProgram.ts:9 (sha256:c6b775023bccac932cdaa74645de9bea419037af0fe4180653169035336ed62f)
#[derive(Clone, Default)]
pub struct GlDebugProgram {
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
    pub loc_joint_normal_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_far: Option<crate::OpaqueHostValue>,
    pub loc_near: Option<crate::OpaqueHostValue>,
    pub loc_normal_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_scale: Option<crate::OpaqueHostValue>,
    pub loc_view: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlDebugProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlDebugProgram.ts:20 (sha256:84b574a8e798cf7cf7855c728fe3c4973ebad64c9e3095ebe300af621cdafa7a)
#[derive(Clone, Default)]
pub struct GlDebugDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_normal_map: bool,
    pub has_skin: Option<bool>,
    pub mode: String,
}
impl PartialEq for GlDebugDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
