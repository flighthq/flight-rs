// @generated from upstream/packages/types/src/CanvasRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasSurface, EntityRuntime};

// Source: upstream/packages/types/src/CanvasRenderTarget.ts:9 (sha256:d55faed0344cf8049c48861ff9ae79e5344cec98dfc4656b46661b4de7383b03)
#[derive(Clone, Default)]
pub struct CanvasRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub canvas: crate::OpaqueHostValue,
    pub color_attachments: f64,
    pub context: crate::OpaqueHostValue,
    pub surface: CanvasSurface,
    pub surface_ownership: CanvasRenderTargetSurfaceOwnership,
}
impl PartialEq for CanvasRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasRenderTarget {
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

// Source: upstream/packages/types/src/CanvasRenderTarget.ts:20 (sha256:a07ef4764276a63666c17573b2ab48fb915da0a7f555bc86c765c1dd943d1ce9)
pub type CanvasRenderTargetSurfaceOwnership = String;

// Source: upstream/packages/types/src/CanvasRenderTarget.ts:27 (sha256:c6de1e77e743b144c48edc14e9556f7e8bb404660093b3374b003c907c5da29f)
#[derive(Clone, Default)]
pub struct CanvasScreenRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub canvas: crate::OpaqueHostValue,
    pub color_attachments: f64,
    pub context: crate::OpaqueHostValue,
    pub surface: CanvasSurface,
    pub surface_ownership: CanvasRenderTargetSurfaceOwnership,
}
impl PartialEq for CanvasScreenRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasScreenRenderTarget {
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

// Source: upstream/packages/types/src/CanvasRenderTarget.ts:33 (sha256:7857b32e90953b5c64051c705cee626fcce43fd33cea1d54c4231ef7960ad733)
#[derive(Clone, Default)]
pub struct CanvasTextureRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub width: f64,
    pub canvas: crate::OpaqueHostValue,
    pub color_attachments: f64,
    pub context: crate::OpaqueHostValue,
    pub surface: CanvasSurface,
    pub surface_ownership: CanvasRenderTargetSurfaceOwnership,
}
impl PartialEq for CanvasTextureRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasTextureRenderTarget {
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
