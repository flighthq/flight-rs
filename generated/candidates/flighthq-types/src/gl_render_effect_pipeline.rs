// @generated from upstream/packages/types/src/GlRenderEffectPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ColorLutCache, GlColorLutTextureCache, GlRenderState, GlRenderTarget, GlRenderTargetPool,
    RenderEffect, RenderTargetDepth, RenderTargetFormat,
};

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:15 (sha256:fdc15a042a1a80053691e6e5e9fdcec40ccc068adde5e22e51ae98764f1520a6)
#[derive(Clone)]
pub struct GlRenderEffectContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: GlRenderState,
    pub source: GlRenderTarget,
    pub dest: GlRenderTarget,
    pub pool: GlRenderTargetPool,
    pub scene_depth_texture: Option<crate::OpaqueHostValue>,
    pub scene_velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlRenderEffectContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:27 (sha256:69a0181b726aa9b774d6209246d94cf8759739ed70ac2330f0a2e3b646e2b9e1)
pub type GlRenderEffectRunner = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GlRenderEffectContext, RenderEffect) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:29 (sha256:4a319c9f224ad0ea33a1150b24214c3f6c6c7c49f71d4b2018541d137eb2d3d0)
#[derive(Clone)]
pub struct RenderEffectPipelineOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sample_count: Option<f64>,
    pub format: Option<RenderTargetFormat>,
    pub depth: Option<RenderTargetDepth>,
}
impl PartialEq for RenderEffectPipelineOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:41 (sha256:ea1b2223d50df5b640545106804895714d12cb99a7838ab014ed2e3816d701a9)
#[derive(Clone)]
pub struct GlRenderEffectPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub options: RenderEffectPipelineOptions,
    pub scene_target: Option<GlRenderTarget>,
    pub pool: GlRenderTargetPool,
    pub lut_cache: ColorLutCache,
    pub lut_texture: GlColorLutTextureCache,
    pub velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlRenderEffectPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
