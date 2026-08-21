// @generated from upstream/packages/types/src/RenderProxy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorScaleBias, EntityRuntime, Kind, Material, MaterialData, Renderable, Renderer,
    RendererData,
};

// Source: upstream/packages/types/src/RenderProxy.ts:9 (sha256:f0d40c25ffe0591e6ea74f08dd22ec61859b14d72b08dbf54d2b642fd68e5cb9)
#[derive(Clone)]
pub struct RenderProxy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub source: Renderable,
    pub kind: Kind,
    pub next: Option<Box<RenderProxy>>,
    pub alpha: f64,
    pub appearance_frame_id: f64,
    pub blend_mode: Option<BlendMode>,
    pub color_scale_bias: Option<ColorScaleBias>,
    pub color_matrix: Option<Vec<f64>>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub last_appearance_id: f64,
    pub last_children_id: f64,
    pub last_local_content_id: f64,
    pub last_local_transform_id: f64,
    pub last_parent_reference_id: f64,
    pub name: Option<String>,
    pub renderer: Option<Renderer>,
    pub renderer_data: Option<RendererData>,
    pub renderer_data_source: Option<Renderable>,
    pub renderer_map_id: f64,
    pub transform_frame_id: f64,
    pub visible: bool,
}
impl PartialEq for RenderProxy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderProxy {
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
