// @generated from upstream/packages/types/src/GlRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderTargetColorSpace, RenderTargetFormat};

// Source: upstream/packages/types/src/GlRenderTarget.ts:12 (sha256:1d7b3079c6b24684ec694b81e6452a67319688ff4d9c5d28b29dd0e49cfab576)
#[derive(Clone)]
pub struct GlRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub format: RenderTargetFormat,
    pub color_space: RenderTargetColorSpace,
    pub clear_colors: Vec<f64>,
    pub clear_depth: f64,
    pub sample_count: f64,
    pub framebuffer: crate::OpaqueHostValue,
    pub resolve_framebuffer: Option<crate::OpaqueHostValue>,
    pub textures: Vec<crate::OpaqueHostValue>,
    pub texture: crate::OpaqueHostValue,
    pub depth_texture: Option<crate::OpaqueHostValue>,
    pub color_renderbuffers: Vec<crate::OpaqueHostValue>,
    pub depth_stencil_renderbuffer: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderTarget.ts:37 (sha256:2964c155d661a295bf99a92f68807ff0836cebfe176a49edfb6da5fec9a0ef41)
#[derive(Clone)]
pub struct GlRenderTargetPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub free: Vec<GlRenderTarget>,
}
impl PartialEq for GlRenderTargetPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
