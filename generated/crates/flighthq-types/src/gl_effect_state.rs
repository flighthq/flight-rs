// @generated from upstream/packages/types/src/GlEffectState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ColorLutCache, Effect, GlColorLutTextureCache, GlRenderState, GlTextureRenderTarget,
    GlTextureRenderTargetPool, RenderTargetDepth, RenderTargetFormat,
};

// Source: upstream/packages/types/src/GlEffectState.ts:15 (sha256:034b63dcadb141a8ccd904dbac5c8b8d5f0b30a36404217420951e1b75d35456)
#[derive(Clone, Default)]
pub struct GlEffectContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: GlRenderState,
    pub source: GlTextureRenderTarget,
    pub dest: GlTextureRenderTarget,
    pub pool: GlTextureRenderTargetPool,
    pub scene_depth_texture: Option<crate::OpaqueHostValue>,
    pub scene_velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlEffectContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlEffectState.ts:27 (sha256:22101b6f715a47297446030b14380e4cc5135d8e4f2aa81d9a7d05043b5f2945)
pub type GlEffectRunner = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GlEffectContext, Effect) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/GlEffectState.ts:56 (sha256:5e5f404cd096d0415dac298d13b1b6b0b749cc54db158e648a74815da564e42d)
pub type GlEffectApplicationStatus = String;

// Source: upstream/packages/types/src/GlEffectState.ts:66 (sha256:37e87cf71a613f3198baabf69c234df23a94ba89e86df04b9b266f5ed26b0046)
#[derive(Clone, Default)]
pub struct GlEffectApplicationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_count: f64,
    pub requested_count: f64,
    pub status: GlEffectApplicationStatus,
    pub unregistered_kinds: Vec<String>,
    pub unresolved_indexes: Vec<f64>,
}
impl PartialEq for GlEffectApplicationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlEffectState.ts:83 (sha256:6edfdad6af76314b7d15a69a6d57eb1af476d67459003c1fa9f42cb3aade3958)
pub type GlEffectResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GlRenderState, Effect) -> bool + Send + 'static>>,
>;

// Source: upstream/packages/types/src/GlEffectState.ts:87 (sha256:43a964ad9b8ae82b629ddc361679abb34da85e2b2fa477d3a52d2a35d36977a1)
#[derive(Clone)]
pub struct GlEffectRegistration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_resolvable: Option<GlEffectResolver>,
    pub runner: GlEffectRunner,
}
impl PartialEq for GlEffectRegistration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlEffectState.ts:97 (sha256:0ee3a6edfc18e564ea69f987317de996030c4c6b55d236b164042e50d2cb892b)
pub type GlCustomShaderSourceGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GlRenderState, String, String, String) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/GlEffectState.ts:107 (sha256:4b33801a6ab583033f382b3e2fdbd14b406679e251bd6565c747a3e4e36ab8d6)
pub type GlEffectStateSkipGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState, String) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/GlEffectState.ts:109 (sha256:2ee8837a7210e7087c17600d46f129d41179cae277b664b50823e70790199f52)
pub type GlEffectApplicationGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(GlRenderState, GlEffectApplicationExplanation) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/GlEffectState.ts:114 (sha256:c16830b47925637d5d53fca6de4749425ac3d97698f7c620374d7b70b54c42b4)
#[derive(Clone, Default)]
pub struct EffectStateOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub depth: Option<RenderTargetDepth>,
    pub format: Option<RenderTargetFormat>,
    pub sample_count: Option<f64>,
}
impl PartialEq for EffectStateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlEffectState.ts:126 (sha256:3f8cd833ee7ac0a56399ee8c20bf57618958c28cc96d8771b1c99c700ce7005f)
#[derive(Clone, Default)]
pub struct GlEffectState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub options: EffectStateOptions,
    pub scene_target: Option<GlTextureRenderTarget>,
    pub pool: GlTextureRenderTargetPool,
    pub lut_cache: ColorLutCache,
    pub lut_texture: GlColorLutTextureCache,
    pub velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlEffectState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
