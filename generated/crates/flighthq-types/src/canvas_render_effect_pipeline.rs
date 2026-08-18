// @generated from upstream/packages/types/src/CanvasRenderEffectPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasRenderState, CanvasRenderTarget, ColorLutCache, RenderEffect, RenderEffectPipelineOptions,
};

// Source: upstream/packages/types/src/CanvasRenderEffectPipeline.ts:14 (sha256:56f73a3c7c106c2cfc9affd8f47d517ff6685a4a5bad8083b9d9fe76d3fcf217)
#[derive(Clone, Default)]
pub struct CanvasRenderEffectContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: CanvasRenderState,
    pub source: CanvasRenderTarget,
    pub dest: CanvasRenderTarget,
    pub pool: CanvasRenderTargetPool,
}
impl PartialEq for CanvasRenderEffectContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderEffectPipeline.ts:25 (sha256:f662dfa49940a2bef7c0263f0651bb0ffe413e45c5dca07f43614c1d875cd5ac)
pub type CanvasRenderEffectRunner = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(CanvasRenderEffectContext, RenderEffect) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/CanvasRenderEffectPipeline.ts:34 (sha256:e0a62ab9b91837adf5488e49d0c33cd05027b35154948fc8d2167b4185d6ba2d)
#[derive(Clone, Default)]
pub struct CanvasRenderTargetPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub free: Vec<CanvasRenderTarget>,
    pub in_use: Vec<CanvasRenderTarget>,
}
impl PartialEq for CanvasRenderTargetPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderEffectPipeline.ts:43 (sha256:f0f92129b058ee5f05cd9b1720f2d60db4a348e6e4db086f74a9ceb8718520af)
#[derive(Clone, Default)]
pub struct CanvasRenderEffectPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub options: RenderEffectPipelineOptions,
    pub scene_target: Option<CanvasRenderTarget>,
    pub pool: CanvasRenderTargetPool,
    pub lut_cache: ColorLutCache,
}
impl PartialEq for CanvasRenderEffectPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
