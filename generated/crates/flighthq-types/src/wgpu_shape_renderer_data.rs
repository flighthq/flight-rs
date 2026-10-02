// @generated from upstream/packages/types/src/WgpuShapeRendererData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasSurface, EntityRuntime, ImageResource, WgpuShapeMesh, WgpuShapeMeshBuffers};

// Source: upstream/packages/types/src/WgpuShapeRendererData.ts:16 (sha256:e3ea0ef6a1993e6c87e401481d3bfdbdbf56bb036185e01d0d15152ef618d557)
#[derive(Clone, Default)]
pub struct WgpuShapeRendererData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub image: Option<ImageResource>,
    pub last_content_id: f64,
    pub last_h: f64,
    pub last_pixel_ratio: f64,
    pub last_w: f64,
    pub mesh_buffers: WgpuShapeMeshBuffers,
    pub mesh_version: f64,
    pub meshes: Option<Vec<WgpuShapeMesh>>,
    pub surface: Option<CanvasSurface>,
}
impl PartialEq for WgpuShapeRendererData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuShapeRendererData {
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
