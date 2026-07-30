// @generated from upstream/packages/types/src/RenderProxy2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorTransform, EntityRuntime, Kind, Material, MaterialData, Matrix, RenderProxy,
    Renderable, Renderer, RendererData,
};

// Source: upstream/packages/types/src/RenderProxy2D.ts:7 (sha256:272ac35d464e6e3a9ff7ab35c445b3fd389a16e2ffc6b301a47f16b3b2d11dcc)
#[derive(Clone)]
pub struct RenderProxy2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub source: Renderable,
    pub kind: Kind,
    pub next: Option<RenderProxy>,
    pub alpha: f64,
    pub appearance_frame_id: f64,
    pub blend_mode: Option<BlendMode>,
    pub color_transform: Option<ColorTransform>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub last_appearance_id: f64,
    pub last_local_content_id: f64,
    pub last_local_transform_id: f64,
    pub name: Option<String>,
    pub renderer: Option<Renderer>,
    pub renderer_data: Option<RendererData>,
    pub renderer_data_source: Option<Renderable>,
    pub renderer_map_id: f64,
    pub transform_frame_id: f64,
    pub visible: bool,
    pub transform2_d: Matrix,
    pub traverse_children: bool,
    pub clip_depth: f64,
}
impl PartialEq for RenderProxy2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderProxy2D {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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
