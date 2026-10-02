// @generated from upstream/packages/types/src/GlRenderTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, GlContext, GlRenderState, GlTextureRenderTarget, GlTextureRenderTargetPool,
    RenderTexture,
};

// Source: upstream/packages/types/src/GlRenderTexture.ts:7 (sha256:7fe14e2761d838a139225d2488a7b28f0769eae7ae0959ffe91dd62e81a3f401)
pub type GlRenderTextureStatus = String;

// Source: upstream/packages/types/src/GlRenderTexture.ts:9 (sha256:39dac1b1bb916941fd370d477ba6730f1eded94d2396ef556e3c5b9ed02afbce)
#[derive(Clone, Default)]
pub struct GlRenderTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub status: GlRenderTextureStatus,
    pub width: f64,
}
impl PartialEq for GlRenderTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderTexture.ts:15 (sha256:a0302b3d0cd97be5c890947cf8080529de3f43f457cc51ba928dbfdf2c06f842)
#[derive(Clone, Default)]
pub struct GlRenderTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: GlRenderTextureStatus,
    pub target: GlTextureRenderTarget,
}
impl PartialEq for GlRenderTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderTexture.ts:22 (sha256:6c4f7b992feff04b65f8e1a22373fe2f67ab60e94b87ca7281a70fa176dc0eb4)
#[derive(Clone, Default)]
pub struct GlRenderTexturePool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub context: Option<GlContext>,
    pub destroyed: bool,
    pub effect_targets: GlTextureRenderTargetPool,
    pub free: Vec<RenderTexture>,
    pub leased: Vec<RenderTexture>,
}
impl PartialEq for GlRenderTexturePool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlRenderTexturePool {
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

// Source: upstream/packages/types/src/GlRenderTexture.ts:30 (sha256:2f3a2dad10d2d1210708e1690cfa69d093b7d7f66bbe1aaa2ef0193b4b12bdc2)
pub type GlRenderTextureGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(GlRenderState, RenderTexture, GlRenderTextureExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
