// @generated from upstream/packages/types/src/CanvasRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasEffectRunner, CanvasQuadMaterialRenderer, CanvasRenderPass,
    CanvasRenderTarget, CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, EffectPaddingResolver,
    EntityRuntime, HostCanvasCapability, HostImageCapability, Kind, NodeRenderer, RenderProxy,
    RenderProxy2D, RenderRegistrySignals, RenderRootGuard, RenderState, Scene2DClipHooks,
    Scene3DGraphSyncPolicy, StrokeTessellator,
};

// Source: upstream/packages/types/src/CanvasRenderState.ts:12 (sha256:c72f2856e8dd8dd19c52c0d7f04e04bdbd4ea5c516ad7693ef9a60b395e4c00a)
#[derive(Clone, Default)]
pub struct CanvasRenderState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub allow_smoothing: bool,
    pub current_clip_depth: f64,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: f64,
    pub canvas_host: Option<HostCanvasCapability>,
    pub image_host: Option<HostImageCapability>,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
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
    pub registries: CanvasRenderRegistries,
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

// Source: upstream/packages/types/src/CanvasRenderState.ts:29 (sha256:9adf90ffc8be2ba4d9eaec5dfda17f595cf5c0ec889a5bcac67189db09974d6d)
#[derive(Clone, Default)]
pub struct CanvasRenderRegistries {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_shape_commands: Option<Vec<(Kind, CanvasShapeCommand<crate::OpaqueHostValue>)>>,
    pub color_adjustments: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(RenderState, RenderProxy, Option<RenderProxy>) -> () + Send + 'static,
                >,
            >,
        >,
    >,
    pub color_adjustment_unsupported_guard: Option<ColorAdjustmentUnsupportedGuard>,
    pub effect_padding_resolvers: Option<Vec<(Kind, EffectPaddingResolver)>>,
    pub node_renderers: Vec<(Kind, NodeRenderer)>,
    pub render_root_guard: Option<RenderRootGuard>,
    pub stroke_tessellator: Option<StrokeTessellator>,
    pub blend_mode_application: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(CanvasRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub material_renderers: Option<Vec<(Kind, CanvasQuadMaterialRenderer)>>,
    pub effects: Vec<(Kind, CanvasEffectRunner)>,
}
impl PartialEq for CanvasRenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderState.ts:46 (sha256:1d18a23b6fb715a6851248f802da02971d89c573a16ca83efb122d7d9cf9337c)
#[derive(Clone)]
pub struct CanvasRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for CanvasRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct CanvasRenderStateRuntimeStorage {
    pub registries: CanvasRenderRegistries,
    pub pass_stack: Vec<CanvasRenderPass>,
    pub current_render_target: Option<CanvasRenderTarget>,
    pub teardowns: Vec<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(CanvasRenderState) -> () + Send + 'static>>>,
    >,
}
impl Default for CanvasRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            registries: Default::default(),
            pass_stack: Default::default(),
            current_render_target: Default::default(),
            teardowns: Default::default(),
        }
    }
}
pub type CanvasRenderStateRuntime = crate::EntityRuntime;
