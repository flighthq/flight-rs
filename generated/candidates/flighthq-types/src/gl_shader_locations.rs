// @generated from upstream/packages/types/src/GlShaderLocations.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/GlShaderLocations.ts:3 (sha256:a325101a93776ff6cf403aaf8ccb7ce58d8b7c489527809358af07d486f1a2b8)
#[derive(Clone)]
pub struct GlShaderLocations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_position: f64,
    pub loc_tex_coord: f64,
    pub loc_matrix: crate::OpaqueHostValue,
    pub loc_alpha: crate::OpaqueHostValue,
    pub loc_color_multiplier: Option<crate::OpaqueHostValue>,
    pub loc_color_offset: Option<crate::OpaqueHostValue>,
    pub loc_has_color_transform: Option<crate::OpaqueHostValue>,
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
