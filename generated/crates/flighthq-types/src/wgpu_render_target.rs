// @generated from upstream/packages/types/src/WgpuRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, RenderTargetColorSpace, WgpuPresentationSurface, WgpuRenderState,
    WgpuTextureBindings,
};

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:18 (sha256:fd1a6651f627582c172d2a7647d168f34a87e0695b394e2588be6a2353d8059d)
#[derive(Clone, Default)]
pub struct WgpuRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub color_attachments: f64,
    pub color_space: RenderTargetColorSpace,
    pub context: Option<crate::OpaqueHostValue>,
    pub depth_stencil_texture: crate::OpaqueHostValue,
    pub depth_stencil_view: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub sample_count: f64,
}
impl PartialEq for WgpuRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuRenderTarget {
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

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:41 (sha256:b7149eea502a1fc76698f7285ac9b9e5cbea8fc15c7629b7a410cf3b7489b9e5)
#[derive(Clone, Default)]
pub struct WgpuRenderTargetPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub free: Vec<WgpuTextureRenderTarget>,
}
impl PartialEq for WgpuRenderTargetPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuRenderTargetPool {
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

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:51 (sha256:3b7f541dfbab54d62a4fc7b3c51d4d7c4311eb1155cf16f2f494a872641f65f0)
#[derive(Clone, Default)]
pub struct WgpuScreenRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub color_attachments: f64,
    pub color_space: RenderTargetColorSpace,
    pub context: crate::OpaqueHostValue,
    pub depth_stencil_texture: crate::OpaqueHostValue,
    pub depth_stencil_view: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub sample_count: f64,
    pub antialias: bool,
    pub antialias_resolve_bind_group: Option<crate::OpaqueHostValue>,
    pub antialias_texture: Option<crate::OpaqueHostValue>,
    pub antialias_view: Option<crate::OpaqueHostValue>,
    pub acquire_antialias_view: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(WgpuRenderState, WgpuScreenRenderTarget) -> crate::OpaqueHostValue
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub encode_antialias_resolve: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(WgpuRenderState, WgpuScreenRenderTarget, crate::OpaqueHostValue) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub capture_buffer: Option<crate::OpaqueHostValue>,
    pub capture_bytes_per_row: f64,
    pub capture_enabled: bool,
    pub capture_height: f64,
    pub capture_texture: Option<crate::OpaqueHostValue>,
    pub capture_width: f64,
    pub acquire_capture_texture: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(WgpuScreenRenderTarget) -> crate::OpaqueHostValue + Send + 'static>,
            >,
        >,
    >,
    pub encode_capture: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(WgpuScreenRenderTarget, crate::OpaqueHostValue) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub device: crate::OpaqueHostValue,
    pub presentation_view: Option<crate::OpaqueHostValue>,
    pub surface: WgpuPresentationSurface,
}
impl PartialEq for WgpuScreenRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuScreenRenderTarget {
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

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:100 (sha256:3f9422343b2af7462394cf3d6d47f37df492a999d36afb9a3ef14d9c26fd087d)
#[derive(Clone, Default)]
pub struct WgpuScreenRenderTargetOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mode: Option<crate::OpaqueHostValue>,
    pub color_space: Option<RenderTargetColorSpace>,
    pub format: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuScreenRenderTargetOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:111 (sha256:6edcec4c7a332c84c1b3657a5d430d958d0ca9fda6b5dac819609b030c913937)
#[derive(Clone, Default)]
pub struct WgpuTextureRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub color_attachments: f64,
    pub color_space: RenderTargetColorSpace,
    pub context: crate::OpaqueHostValue,
    pub depth_stencil_texture: crate::OpaqueHostValue,
    pub depth_stencil_view: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub sample_count: f64,
    pub bindings: WgpuTextureBindings,
    pub mip_level_count: f64,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for WgpuTextureRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuTextureRenderTarget {
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
