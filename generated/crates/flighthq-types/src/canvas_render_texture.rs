// @generated from upstream/packages/types/src/CanvasRenderTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasRenderState, CanvasRenderTargetPool, CanvasTextureRenderTarget, EntityRuntime,
    RenderTexture,
};

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:7 (sha256:fd069efe688deec5cc0c65e6a976dc2148345bf96474876535b91afd4f8b8943)
pub type CanvasRenderTextureStatus = String;

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:9 (sha256:5110cc247aaba889f23dcefc02330f1204c91fbc99863105fe96425fa255bda2)
#[derive(Clone, Default)]
pub struct CanvasRenderTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub status: CanvasRenderTextureStatus,
    pub width: f64,
}
impl PartialEq for CanvasRenderTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:15 (sha256:07aefc1d3598be7fecb0b208403af8438c55ad1fd79b6f5f236c5d02348f6e17)
#[derive(Clone, Default)]
pub struct CanvasRenderTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: CanvasRenderTextureStatus,
    pub target: CanvasTextureRenderTarget,
}
impl PartialEq for CanvasRenderTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:22 (sha256:394020aeb0da3c4fcffdbccca6e4114a8ac1f918c4bb2acf29ae07a3ac99198d)
#[derive(Clone)]
pub struct CanvasRenderTexturePool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub destroyed: bool,
    pub effect_targets: CanvasRenderTargetPool,
    pub free: Vec<RenderTexture>,
    pub leased: Vec<RenderTexture>,
    pub owner: Option<CanvasRenderState>,
}
impl PartialEq for CanvasRenderTexturePool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasRenderTexturePool {
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
