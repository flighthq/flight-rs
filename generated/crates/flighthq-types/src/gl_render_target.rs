// @generated from upstream/packages/types/src/GlRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, GlContext, RenderTargetAxes, RenderTargetColorSpace, RenderTargetDepth,
    RenderTargetFormat,
};

// Source: upstream/packages/types/src/GlRenderTarget.ts:15 (sha256:aa0b5e649ab4669396c36d7154fdb304a822df9ff4f67f553dbd94a239342c8b)
#[derive(Clone, Default)]
pub struct GlRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub gl: GlContext,
    pub framebuffer: Option<crate::OpaqueHostValue>,
    pub color_attachments: f64,
    pub color_space: RenderTargetColorSpace,
}
impl PartialEq for GlRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlRenderTarget {
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

// Source: upstream/packages/types/src/GlRenderTarget.ts:27 (sha256:b05e99467b2ba4fdc0655f3db9d76bb75d18b55764a54ea8e8293a1e2d037094)
#[derive(Clone, Default)]
pub struct GlScreenRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub gl: GlContext,
    pub framebuffer: crate::OpaqueHostValue,
    pub color_attachments: f64,
    pub color_space: RenderTargetColorSpace,
}
impl PartialEq for GlScreenRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlScreenRenderTarget {
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

// Source: upstream/packages/types/src/GlRenderTarget.ts:40 (sha256:28001d51e2d30ae8acab116b56ba7d1685441604e883637196e63c25b0300fbb)
#[derive(Clone, Default)]
pub struct GlTextureRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub gl: GlContext,
    pub framebuffer: crate::OpaqueHostValue,
    pub color_attachments: f64,
    pub color_space: RenderTargetColorSpace,
    pub requested_axes: RenderTargetAxes,
    pub format: RenderTargetFormat,
    pub color_formats: Vec<RenderTargetFormat>,
    pub depth: RenderTargetDepth,
    pub sample_count: f64,
    pub resolve_framebuffer: Option<crate::OpaqueHostValue>,
    pub textures: Vec<crate::OpaqueHostValue>,
    pub texture: crate::OpaqueHostValue,
    pub depth_texture: Option<crate::OpaqueHostValue>,
    pub color_renderbuffers: Vec<crate::OpaqueHostValue>,
    pub depth_stencil_renderbuffer: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlTextureRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlTextureRenderTarget {
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

// Source: upstream/packages/types/src/GlRenderTarget.ts:60 (sha256:e52c586f921f1a9563f462c49c8efffaa117b71e3852ee0b89a9cfacb48f0efd)
#[derive(Clone, Default)]
pub struct GlTextureRenderTargetPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub free: Vec<GlTextureRenderTarget>,
}
impl PartialEq for GlTextureRenderTargetPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlTextureRenderTargetPool {
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
