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
#[derive(Clone)]
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
#[derive(Clone)]
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
