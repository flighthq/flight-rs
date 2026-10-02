// @generated from upstream/packages/types/src/WgpuRenderTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, RenderTexture, WgpuRenderState, WgpuRenderTargetPool, WgpuTextureRenderTarget,
};

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:6 (sha256:2ec2ec6678145349a96ebcf33e25545566d1fcffc5017c52d13d89ac8178b393)
pub type WgpuRenderTextureStatus = String;

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:8 (sha256:d8a2c37edbe1cd43d94d9db1777975309683b22d99e2e8638e1699b5976b05f8)
#[derive(Clone, Default)]
pub struct WgpuRenderTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub status: WgpuRenderTextureStatus,
    pub width: f64,
}
impl PartialEq for WgpuRenderTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:15 (sha256:9279e143a2a35ca874d388be64c48bf31ded469a886383bee2521b8aa4438bd6)
#[derive(Clone, Default)]
pub struct WgpuRenderTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: WgpuRenderTextureStatus,
    pub target: WgpuTextureRenderTarget,
}
impl PartialEq for WgpuRenderTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:22 (sha256:01a9fdb104258e3f918c030e9129a4c3fdaa07a6dac0d7072e6c735ad483cedc)
#[derive(Clone, Default)]
pub struct WgpuRenderTexturePool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub device: Option<crate::OpaqueHostValue>,
    pub destroyed: bool,
    pub effect_targets: WgpuRenderTargetPool,
    pub free: Vec<RenderTexture>,
    pub leased: Vec<RenderTexture>,
}
impl PartialEq for WgpuRenderTexturePool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuRenderTexturePool {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:30 (sha256:1ff4489a9bfc7ddf6bcfe0bdf2650cfd22848ad67e613739bbafcae9bcd9aa27)
pub type WgpuRenderTextureGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(WgpuRenderState, RenderTexture, WgpuRenderTextureExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
