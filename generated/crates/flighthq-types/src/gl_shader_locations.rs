// @generated from upstream/packages/types/src/GlShaderLocations.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/GlShaderLocations.ts:3 (sha256:966bcc46298e490b6b57b9ed4eba613df279e74d504470ba591c2ac8163e5f94)
#[derive(Clone, Default)]
pub struct GlShaderLocations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_position: f64,
    pub loc_tex_coord: f64,
    pub loc_matrix: crate::OpaqueHostValue,
    pub loc_alpha: crate::OpaqueHostValue,
    pub loc_color_scale: Option<crate::OpaqueHostValue>,
    pub loc_color_bias: Option<crate::OpaqueHostValue>,
    pub loc_has_color_scale_bias: Option<crate::OpaqueHostValue>,
    pub loc_texture: crate::OpaqueHostValue,
}
impl PartialEq for GlShaderLocations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlShaderLocations.ts:15 (sha256:03af8860521550065246472af1b2f29e1cfb10ae91dea499d730faf388970eca)
#[derive(Clone)]
pub struct GlBitmapShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(crate::OpaqueHostValue, GlRenderState, RenderProxy2D) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub locations: GlShaderLocations,
}
impl PartialEq for GlBitmapShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
