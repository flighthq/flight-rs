// @generated from upstream/packages/types/src/WgpuRenderEffectPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ColorLutCache, RenderEffect, RenderEffectPipelineOptions, WgpuColorLutTextureCache,
    WgpuRenderState, WgpuRenderTarget, WgpuRenderTargetPool,
};

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:14 (sha256:fd9b6f3f63bcd3f4391e10fb091fcb7444196085bcefc1d301287787dfe3a3e2)
#[derive(Clone, Default)]
pub struct WgpuRenderEffectContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: WgpuRenderState,
    pub source: WgpuRenderTarget,
    pub dest: WgpuRenderTarget,
    pub pool: WgpuRenderTargetPool,
    pub scene_depth_texture: Option<crate::OpaqueHostValue>,
    pub scene_velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuRenderEffectContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:26 (sha256:8a5a5b5576f625c8b18f2d1946bd885214918198b1fb9faa9536850fe9c0565b)
pub type WgpuRenderEffectRunner = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuRenderEffectContext, RenderEffect) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:32 (sha256:a7039648e61c44e19af4213680f60efe61ad6452ffc0b16c420927d0116c0349)
#[derive(Clone, Default)]
pub struct WgpuRenderEffectPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub options: RenderEffectPipelineOptions,
    pub scene_target: Option<WgpuRenderTarget>,
    pub pool: WgpuRenderTargetPool,
    pub lut_cache: ColorLutCache,
    pub lut_texture: WgpuColorLutTextureCache,
    pub velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuRenderEffectPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:54 (sha256:a448719affdbe4b3bee9161799291715a66c5077aebfa488749aef959e96f759)
pub type WgpuRenderEffectApplicationStatus = String;

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:62 (sha256:80df47c236a0be8f8f8c65094ce725003a23fcc7821dd35426171af61c391349)
#[derive(Clone, Default)]
pub struct WgpuRenderEffectApplicationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_count: f64,
    pub requested_count: f64,
    pub status: WgpuRenderEffectApplicationStatus,
    pub unregistered_kinds: Vec<String>,
}
impl PartialEq for WgpuRenderEffectApplicationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:72 (sha256:74953457a8bcd7c38685cd66d99f441b334b251ffc6a7ea900a73c84f24e64ea)
pub type WgpuRenderEffectPipelineSkipGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuRenderState, String) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:76 (sha256:203dbdb946e3c38fbfd165441720f389d0369550a61cf0be59c48f3aedb5d281)
pub type WgpuRenderEffectPipelineSampleCountGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuRenderState, f64, f64) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuRenderEffectPipeline.ts:82 (sha256:5ecb09a7d7550431036e0bc3702226ddb453918cfd8544a7908cc1712653d8b2)
pub type WgpuRenderEffectApplicationGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(WgpuRenderState, WgpuRenderEffectApplicationExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
