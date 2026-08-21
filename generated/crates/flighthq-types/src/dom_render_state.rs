// @generated from upstream/packages/types/src/DomRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Bitmap, BlendMode, CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, DomScene2DRectangle,
    DomTextureResolver, EntityRuntime, KeyedTable, Matrix, Path, PathMesh, PathWinding,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderRegistrySignals,
    RenderRootGuard, RenderState, Renderer, Scene2DClipHooks, Scene3DGraphSyncPolicy,
    ShapeRasterizer, SlotTable, StrokeStyle,
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

// Source: upstream/packages/types/src/DomRenderState.ts:21 (sha256:3b8ef9dbbfa02ffa5cbfa60a7794d052d50f559167276292a2a620425bedb3e0)
#[derive(Clone, Default)]
pub struct DomRenderRegistries {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_shape_commands: Option<KeyedTable<CanvasShapeCommand<crate::OpaqueHostValue>>>,
    pub color_adjustments: Option<
        SlotTable<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<
                        dyn FnMut(RenderState, RenderProxy, Option<RenderProxy>) -> ()
                            + Send
                            + 'static,
                    >,
                >,
            >,
        >,
    >,
    pub color_adjustment_unsupported_guard: Option<SlotTable<ColorAdjustmentUnsupportedGuard>>,
    pub effect_padding_resolvers: Option<KeyedTable<RenderEffectPaddingResolver>>,
    pub renderers: KeyedTable<Renderer>,
    pub render_root_guard: Option<SlotTable<RenderRootGuard>>,
    pub stroke_tessellator: SlotTable<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Path, StrokeStyle, Option<f64>) -> Option<PathMesh> + Send + 'static>,
            >,
        >,
    >,
    pub shape_rasterizer: SlotTable<ShapeRasterizer>,
    pub texture_resolvers: KeyedTable<DomTextureResolver>,
}
impl PartialEq for DomRenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DomRenderState.ts:30 (sha256:0a8d83da2d0248649e6b7200c1cef7462b9438b5ae01577f4efa27d8fb957109)
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
    pub registries: DomRenderRegistries,
    pub bitmap_element_cache: Option<Vec<(Bitmap, DomRenderStateRuntimeRecord1)>>,
}
impl Default for DomRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            registries: Default::default(),
            bitmap_element_cache: Default::default(),
        }
    }
}
pub type DomRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/DomRenderState.ts:57 (sha256:38fa448f3ab74d60dcd04814fef23fdd7681955b2281907978b8d6bc1f47c017)
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

// Source: upstream/packages/types/src/DomRenderState.ts:67 (sha256:36b4b685961078ec57e1899080a4a3acdcd42a8d1ccff62bd6e3259fa9b571cd)
pub type DomClipEntry = crate::FlightUnion2<DomClipContourEntry, DomScene2DRectangle>;

// Source: upstream/packages/types/src/DomRenderState.ts:72 (sha256:60fd7cdcf4fa1ac76ed25cb9780162dc456844bc2439b386623c4addf7067d88)
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
