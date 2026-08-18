// @generated from upstream/packages/types/src/RenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, EntityRuntime, Matrix, RenderRegistrySignals, Scene2DClipHooks};

// Source: upstream/packages/types/src/RenderState.ts:24 (sha256:774d9b5364bf64a92a4ee998bc4ef3d6effcacc87376b78caf211241fa145de9)
pub type Scene3DGraphSyncPolicy = String;

// Source: upstream/packages/types/src/RenderState.ts:26 (sha256:2f1b22ab88295c563dd5bc4c915601d3bfb0ceb7a0cf9390902bf8134147b6a3)
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

// Source: upstream/packages/types/src/RenderState.ts:48 (sha256:b27249a8cd675e578a7deb9802c0ebbf90928b97b9000fc7b47a1165ad38f419)
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
}
impl Default for RenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            registry_miss: Default::default(),
        }
    }
}
pub type RenderStateRuntime = crate::EntityRuntime;
