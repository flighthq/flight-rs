// @generated from upstream/packages/types/src/RenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasShapeCommand, EffectPaddingResolver, EntityRuntime, HostCanvasCapability,
    HostImageCapability, Kind, NodeAny, NodeRenderer, Path, PathMesh, RenderProxy,
    RenderRegistrySignals, Scene2DClipHooks, StrokeStyle,
};

// Source: upstream/packages/types/src/RenderState.ts:25 (sha256:774d9b5364bf64a92a4ee998bc4ef3d6effcacc87376b78caf211241fa145de9)
pub type Scene3DGraphSyncPolicy = String;

// Source: upstream/packages/types/src/RenderState.ts:27 (sha256:9a5616988aae1bcb350042d2433c0745766edbef7e53ff2a551676151729b2aa)
#[derive(Clone, Default)]
pub struct RenderState {
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

// Source: upstream/packages/types/src/RenderState.ts:44 (sha256:4720d25f452d6cdd33c300a75fb8fa00210530353524da68255b47bd111d77ae)
#[derive(Clone, Default)]
pub struct RenderRegistries {
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
}
impl PartialEq for RenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderState.ts:64 (sha256:b1dc141bea32c444c8f795578aad8580220731980af56f0b73a812b4d41b6841)
pub type ColorAdjustmentUnsupportedGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(RenderState, NodeAny) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/RenderState.ts:65 (sha256:30548f6aeaf62330e89a2de67bd2324534c60ed84008f1cce166a80aba39c431)
pub type RenderRootGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(RenderState, NodeAny) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/RenderState.ts:86 (sha256:7a17b9297eaaf53e88968590f4cb20acfcb417c7af5fd8609ed8214d4e7e4184)
pub type StrokeTessellator = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Path, StrokeStyle, Option<f64>) -> Option<PathMesh> + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/RenderState.ts:97 (sha256:63a5697bee0139c22e6fef10b085d6b1a9ab5c4383096099df0592c49b6a9d05)
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
