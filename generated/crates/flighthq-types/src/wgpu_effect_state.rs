// @generated from upstream/packages/types/src/WgpuEffectState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ColorLutCache, Effect, EffectStateOptions, WgpuColorLutTextureCache, WgpuRenderState,
    WgpuRenderTargetPool, WgpuTextureRenderTarget,
};

// Source: upstream/packages/types/src/WgpuEffectState.ts:14 (sha256:1b7d640b59e4a5675a331ee50704a55a81d67dc6004437d31e03d4fdf0723597)
#[derive(Clone, Default)]
pub struct WgpuEffectContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: WgpuRenderState,
    pub source: WgpuTextureRenderTarget,
    pub dest: WgpuTextureRenderTarget,
    pub pool: WgpuRenderTargetPool,
    pub scene_depth_texture: Option<crate::OpaqueHostValue>,
    pub scene_velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuEffectContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuEffectState.ts:26 (sha256:09b8199ba1a3a4e30d8c07126aa93ca58e2fe3797742f33c93596ccd0b14e343)
pub type WgpuEffectRunner = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuEffectContext, Effect) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuEffectState.ts:28 (sha256:1736f5dc3e4d903cd77735cf137ce53883ec6a4008d9ed0a811b1b796275bf37)
pub type WgpuEffectResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuRenderState, Effect) -> bool + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuEffectState.ts:30 (sha256:735f75688883a3e07a2769bd31b2e36ac2250d45eb553573fe95158167559096)
#[derive(Clone)]
pub struct WgpuEffectRegistration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_resolvable: Option<WgpuEffectResolver>,
    pub runner: WgpuEffectRunner,
}
impl PartialEq for WgpuEffectRegistration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuEffectState.ts:39 (sha256:135f8920c88dade800a63d549629d658dcb5173b00c9bda3d43d4fa2334d31ce)
#[derive(Clone, Default)]
pub struct WgpuEffectState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub options: EffectStateOptions,
    pub scene_target: Option<WgpuTextureRenderTarget>,
    pub pool: WgpuRenderTargetPool,
    pub lut_cache: ColorLutCache,
    pub lut_texture: WgpuColorLutTextureCache,
    pub velocity_texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuEffectState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuEffectState.ts:58 (sha256:190615ce44d83773cb14c1864ceb1a3150660a5c144adc85269de074a30e3182)
pub type WgpuEffectApplicationStatus = String;

// Source: upstream/packages/types/src/WgpuEffectState.ts:68 (sha256:07f803f77a4447f2e4f3374758c5da15c1e67a4bb9860e6d62fd4b4e5c957315)
#[derive(Clone, Default)]
pub struct WgpuEffectApplicationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_count: f64,
    pub requested_count: f64,
    pub status: WgpuEffectApplicationStatus,
    pub unregistered_kinds: Vec<String>,
    pub unresolved_indexes: Vec<f64>,
}
impl PartialEq for WgpuEffectApplicationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuEffectState.ts:79 (sha256:c1718b61fd570340b2920efddc03ddf85e51025379793ee80b4e60f8aa4f0681)
pub type WgpuEffectStateSkipGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuRenderState, String) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuEffectState.ts:83 (sha256:367fb9e1127e63b705938698bf007705893c2d63c50b2b578581e7e32f766e57)
pub type WgpuEffectStateSampleCountGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(WgpuRenderState, f64, f64) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/WgpuEffectState.ts:89 (sha256:92d0f2e202b3ceaf69a49a006600ee562da831b77c4a4b9ad4e5ec2140ac5a19)
pub type WgpuEffectApplicationGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(WgpuRenderState, WgpuEffectApplicationExplanation) -> () + Send + 'static>,
    >,
>;
