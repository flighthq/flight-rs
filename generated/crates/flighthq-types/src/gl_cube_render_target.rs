// @generated from upstream/packages/types/src/GlCubeRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GlContext, RenderTargetColorSpace};

// Source: upstream/packages/types/src/GlCubeRenderTarget.ts:8 (sha256:14766576db19447de9dd022eeb99354cca89e9168e6d9cb7a7b7cf01447a199a)
#[derive(Clone, Default)]
pub struct GlCubeRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub gl: GlContext,
    pub color_space: RenderTargetColorSpace,
    pub depth_stencil_renderbuffer: Option<crate::OpaqueHostValue>,
    pub framebuffer: crate::OpaqueHostValue,
    pub height: f64,
    pub size: f64,
    pub texture: crate::OpaqueHostValue,
    pub textures: Vec<crate::OpaqueHostValue>,
    pub width: f64,
}
impl PartialEq for GlCubeRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlCubeRenderTarget {
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

// Source: upstream/packages/types/src/GlCubeRenderTarget.ts:20 (sha256:8d99787e275a6fc9c3aacf70e5d6cedc51f96a567acfeeb38ac1c6f3e9612eda)
#[derive(Clone, Default)]
pub struct GlCubeRenderTargetOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub depth: Option<bool>,
}
impl PartialEq for GlCubeRenderTargetOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
