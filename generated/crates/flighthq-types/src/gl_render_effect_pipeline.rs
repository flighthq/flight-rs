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

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:56 (sha256:ac427124db3f2b53e08f9373389b982c2267380d691daaa081731d8e17289c3a)
pub type GlRenderEffectApplicationStatus = String;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:66 (sha256:9b76f1af7c0c56fb8e63f46501e7fbd383c1ca70e99a8c2150f52e1ecd678a6d)
#[derive(Clone, Default)]
pub struct GlRenderEffectApplicationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_count: f64,
    pub requested_count: f64,
    pub status: GlRenderEffectApplicationStatus,
    pub unregistered_kinds: Vec<String>,
    pub unresolved_indexes: Vec<f64>,
}
impl PartialEq for GlRenderEffectApplicationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:83 (sha256:37080a7856647042a89a35275530aa8fd466d6e71c227a4fd27df0667d5149cd)
pub type GlRenderEffectResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GlRenderState, RenderEffect) -> bool + Send + 'static>>,
>;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:87 (sha256:e3ef142a4fce9362c012481e5bf23a1411b7645dcccd1f92d1ba85adeb4d4197)
#[derive(Clone)]
pub struct GlRenderEffectRegistration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_resolvable: Option<GlRenderEffectResolver>,
    pub runner: GlRenderEffectRunner,
}
impl PartialEq for GlRenderEffectRegistration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:97 (sha256:0ee3a6edfc18e564ea69f987317de996030c4c6b55d236b164042e50d2cb892b)
pub type GlCustomShaderSourceGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GlRenderState, String, String, String) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:107 (sha256:83a4ab622091c0b1da1732735e95aa9a9ea37f857879cbe07f4e38f95e512551)
pub type GlRenderEffectPipelineSkipGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState, String) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:109 (sha256:12b35a91d9190b8cfc94d382998304d41cb7ea0f43988651a47c8cde753c94e0)
pub type GlRenderEffectApplicationGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(GlRenderState, GlRenderEffectApplicationExplanation) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:114 (sha256:4a319c9f224ad0ea33a1150b24214c3f6c6c7c49f71d4b2018541d137eb2d3d0)
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

// Source: upstream/packages/types/src/GlRenderEffectPipeline.ts:126 (sha256:ea1b2223d50df5b640545106804895714d12cb99a7838ab014ed2e3816d701a9)
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
