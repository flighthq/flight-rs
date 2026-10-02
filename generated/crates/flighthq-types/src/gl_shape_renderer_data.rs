// @generated from upstream/packages/types/src/GlShapeRendererData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasSurface, EntityRuntime, GlShapeMesh, ImageResource};

// Source: upstream/packages/types/src/GlShapeRendererData.ts:14 (sha256:eebfe72cf1a8da3d09790e097c92fade3e8632c8499bb484d4bf0858f2cef6e0)
#[derive(Clone, Default)]
pub struct GlShapeRendererData {
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
    pub mesh_version: f64,
    pub meshes: Option<Vec<GlShapeMesh>>,
    pub surface: Option<CanvasSurface>,
}
impl PartialEq for GlShapeRendererData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlShapeRendererData {
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
