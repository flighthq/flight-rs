// @generated from upstream/packages/types/src/WgpuHost.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AppWindow, EntityRuntime, NativeSurfaceHandle, Surface};

// Source: upstream/packages/types/src/WgpuHost.ts:15 (sha256:d3fb1906a5ab4a2d6e44196f32af2614aa1308faa7c98e20de9bc3953ad3f7cd)
#[derive(Clone, Default)]
pub struct WgpuPresentationSurface {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for WgpuPresentationSurface {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuHost.ts:20 (sha256:0699dc71c1012a54874e90085d4cebd4ed5f1c53933d756c14e516683b2157bc)
#[derive(Clone, Default)]
pub struct WgpuHostAcquisition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub context: crate::OpaqueHostValue,
    pub device: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub ownership: String,
    pub surface: WgpuPresentationSurface,
}
impl PartialEq for WgpuHostAcquisition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuHostAcquisition {
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

// Source: upstream/packages/types/src/WgpuHost.ts:32 (sha256:0bf03679c802737357b333f28adcb6aefcd1ccb148931f7deef55eecb6484517)
#[derive(Clone, Default)]
pub struct WgpuHostAcquisitionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<crate::OpaqueHostValue>,
    pub power_preference: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuHostAcquisitionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuHost.ts:41 (sha256:57130869b11e36f2588c57140fd7a95c3e4ccb90a48985373321dbede7b8b385)
#[derive(Clone)]
pub struct HostWgpuCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Surface,
                        WgpuHostAcquisitionOptions,
                    ) -> crate::FlightTask<WgpuHostAcquisition>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub create: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AppWindow, f64, f64) -> Option<NativeSurfaceHandle> + Send + 'static>,
        >,
    >,
    pub attach_surface: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Surface, WgpuSurfaceAttachment) -> Option<WgpuSurfaceAttachResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub is_supported: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub release: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(WgpuHostAcquisition) -> () + Send + 'static>>,
    >,
}
impl PartialEq for HostWgpuCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuHost.ts:59 (sha256:269816038da9a49c03126739b7c22d64d68b57f387439faefca2f4c7457dac90)
#[derive(Clone, Default)]
pub struct WgpuSurfaceAttachResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: crate::OpaqueHostValue,
    pub surface: WgpuPresentationSurface,
}
impl PartialEq for WgpuSurfaceAttachResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuHost.ts:64 (sha256:5b63583e39ce0b52a6b9f74e2806c30f376d40e588fe9c38ff31df4988d4d7ec)
#[derive(Clone, Default)]
pub struct WgpuSurfaceAttachment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mode: crate::OpaqueHostValue,
    pub device: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
}
impl PartialEq for WgpuSurfaceAttachment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
