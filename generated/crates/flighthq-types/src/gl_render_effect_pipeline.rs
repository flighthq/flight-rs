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
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:43 (sha256:38669b83aab86a47661e1e12f33c80550379c3f0b7543cb86f88409f7a250ddf)
pub type GlRenderEffectApplicationStatus = String;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:51 (sha256:15e53d2503c0496748f20338561a40b594875934b0cd93d9ea5cf712d7b4bbe4)
#[derive(Clone, Default)]
pub struct GlRenderEffectApplicationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_count: f64,
    pub requested_count: f64,
    pub status: GlRenderEffectApplicationStatus,
    pub unregistered_kinds: Vec<String>,
}
impl PartialEq for GlRenderEffectApplicationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:58 (sha256:12b35a91d9190b8cfc94d382998304d41cb7ea0f43988651a47c8cde753c94e0)
pub type GlRenderEffectApplicationGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(GlRenderState, GlRenderEffectApplicationExplanation) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:63 (sha256:4a319c9f224ad0ea33a1150b24214c3f6c6c7c49f71d4b2018541d137eb2d3d0)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:75 (sha256:ea1b2223d50df5b640545106804895714d12cb99a7838ab014ed2e3816d701a9)
#[derive(Clone, Default)]
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
