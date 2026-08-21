// @generated from upstream/packages/types/src/CanvasRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasMaterialRenderer, CanvasRenderEffectRunner, CanvasShapeCommand,
    ColorAdjustmentUnsupportedGuard, EntityRuntime, KeyedTable, Matrix, Path, PathMesh,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderRootGuard, RenderState,
    Renderer, Scene2DClipHooks, Scene3DGraphSyncPolicy, SlotTable, StrokeStyle,
};

// Source: upstream/packages/types/src/CanvasRenderState.ts:10 (sha256:2d3ed80aeffa1af698defe21cc96fededc24c5de7d4a233df2684315565006c5)
#[derive(Clone, Default)]
pub struct CanvasRenderState {
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
                Box<dyn FnMut(CanvasRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub canvas_css_filter_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(CanvasRenderState, RenderProxy2D) -> Option<String> + Send + 'static>,
            >,
        >,
    >,
    pub canvas: crate::OpaqueHostValue,
    pub context: crate::OpaqueHostValue,
    pub context_attributes: crate::OpaqueHostValue,
}
impl PartialEq for CanvasRenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasRenderState {
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

// Source: upstream/packages/types/src/CanvasRenderState.ts:22 (sha256:bf54026159b9f0b3aa3951ced7856cb58c83480a7bfffbc5dd32f36ea5a03b02)
#[derive(Clone, Default)]
pub struct CanvasRenderRegistries {
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
    pub material_renderers: Option<KeyedTable<CanvasMaterialRenderer>>,
    pub render_effects: KeyedTable<CanvasRenderEffectRunner>,
}
impl PartialEq for CanvasRenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderState.ts:34 (sha256:5af720d86a9638ad751e184c1a7db541300dcdce38e6e5e5168e2c0fe5b00421)
#[doc(hidden)]
pub struct CanvasRenderStateRuntimeStorage {
    pub registries: CanvasRenderRegistries,
}
impl Default for CanvasRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            registries: Default::default(),
        }
    }
}
pub type CanvasRenderStateRuntime = crate::EntityRuntime;
