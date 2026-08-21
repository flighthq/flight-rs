// @generated from upstream/packages/types/src/RenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasShapeCommand, EntityRuntime, KeyedTable, Matrix, Path, PathMesh,
    RenderEffectPaddingResolver, RenderProxy, RenderRegistrySignals, Renderable, Renderer,
    Scene2DClipHooks, SlotTable, StrokeStyle,
};

// Source: upstream/packages/types/src/RenderState.ts:25 (sha256:774d9b5364bf64a92a4ee998bc4ef3d6effcacc87376b78caf211241fa145de9)
pub type Scene3DGraphSyncPolicy = String;

// Source: upstream/packages/types/src/RenderState.ts:27 (sha256:2f1b22ab88295c563dd5bc4c915601d3bfb0ceb7a0cf9390902bf8134147b6a3)
#[derive(Clone, Default)]
pub struct RenderState {
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
}
impl PartialEq for RenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderState {
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

// Source: upstream/packages/types/src/RenderState.ts:46 (sha256:025737da9ae647cb9dbb97d5b4a0fcee009a6fde35836d07bb03683375d9a3dc)
#[derive(Clone, Default)]
pub struct RenderRegistries {
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
}
impl PartialEq for RenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderState.ts:66 (sha256:c881e221a4581ec571bf702bb20885948eae7a78582f3a6bfc79f7183d5656a4)
pub type ColorAdjustmentUnsupportedGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/RenderState.ts:67 (sha256:18be36bffc729f37df5be1626ec7d20c9592417d1213fa795a217ab5b5182278)
pub type RenderRootGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/RenderState.ts:74 (sha256:1c285541caead8d5b1b57d898fffaf5eb240a01f1ca74a91bb910133f58947eb)
#[derive(Clone)]
pub struct RenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for RenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct RenderStateRuntimeStorage {
    pub registry_miss: Option<RenderStateRuntimeRecord1>,
    pub registries: RenderRegistries,
}
impl Default for RenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            registry_miss: Default::default(),
            registries: Default::default(),
        }
    }
}
pub type RenderStateRuntime = crate::EntityRuntime;
