// @generated from upstream/packages/types/src/DomRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Bitmap, BlendMode, DomScene2DRectangle, EntityRuntime, Matrix, PathWinding, RenderProxy2D,
    RenderRegistrySignals, Scene2DClipHooks, Scene3DGraphSyncPolicy,
};

// Source: upstream/packages/types/src/DomRenderState.ts:11 (sha256:7dd771caabb5913c54dc523f2804bfcdff548e84aca709060279660fb26ea9af)
#[derive(Clone, Default)]
pub struct DomRenderState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub allow_smoothing: bool,
    pub background_color: f64,
    pub background_color_rgba: Vec<f64>,
    pub background_color_string: String,
    pub current_clip_depth: f64,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: f64,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Scene3DGraphSyncPolicy,
    pub round_pixels: bool,
    pub apply_blend_mode: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(crate::OpaqueHostValue, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub dom_css_filter_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderProxy2D) -> Option<String> + Send + 'static>>,
        >,
    >,
    pub element: crate::OpaqueHostValue,
}
impl PartialEq for DomRenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for DomRenderState {
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

// Source: upstream/packages/types/src/DomRenderState.ts:23 (sha256:2fc485e81e3cee06d54afe96b9e729f984503a325de3c0fa9fb2eb466f01ed3b)
#[derive(Clone, Default)]
pub struct DomRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub element: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for DomRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct DomRenderStateRuntimeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for DomRenderStateRuntimeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct DomRenderStateRuntimeStorage {
    pub bitmap_element_cache: Option<Vec<(Bitmap, DomRenderStateRuntimeRecord1)>>,
}
impl Default for DomRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            bitmap_element_cache: Default::default(),
        }
    }
}
pub type DomRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/DomRenderState.ts:56 (sha256:38fa448f3ab74d60dcd04814fef23fdd7681955b2281907978b8d6bc1f47c017)
#[derive(Clone, Default)]
pub struct DomClipContourEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub contours: Vec<Vec<f64>>,
    pub kind: String,
    pub winding: PathWinding,
}
impl PartialEq for DomClipContourEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DomRenderState.ts:66 (sha256:36b4b685961078ec57e1899080a4a3acdcd42a8d1ccff62bd6e3259fa9b571cd)
pub type DomClipEntry = crate::FlightUnion2<DomClipContourEntry, DomScene2DRectangle>;

// Source: upstream/packages/types/src/DomRenderState.ts:71 (sha256:60fd7cdcf4fa1ac76ed25cb9780162dc456844bc2439b386623c4addf7067d88)
#[derive(Clone)]
pub struct DomClipHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(DomRenderState, RenderProxy2D) -> () + Send + 'static>>,
    >,
}
impl PartialEq for DomClipHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
