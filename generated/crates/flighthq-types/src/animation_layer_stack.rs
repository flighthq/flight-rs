// @generated from upstream/packages/types/src/AnimationLayerStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AnimationBlendTree, AnimationChannel, AnimationPlayer, AnimationStateMachine, EntityRuntime,
};

// Source: upstream/packages/types/src/AnimationLayerStack.ts:7 (sha256:5a441593b3afa689a2eeb556524ba3b4d1feeada4b4250dfed4b65c40c3ecb77)
#[derive(Clone, Default)]
pub struct AnimationLayerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub additive: Option<bool>,
    pub channel_indices: Option<Vec<f64>>,
    pub weight: Option<f64>,
}
impl PartialEq for AnimationLayerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationLayerStack.ts:16 (sha256:49e24f8195b6c8f54063c8f1b16b6b8a574fdb6d28380516aa63a0b33289fd55)
#[derive(Clone, Default)]
pub struct AnimationLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub additive: bool,
    pub blend_tree: Option<AnimationBlendTree>,
    pub channel_indices: Option<Vec<f64>>,
    pub state_machine: Option<AnimationStateMachine>,
    pub weight: f64,
}
impl PartialEq for AnimationLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationLayer {
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

// Source: upstream/packages/types/src/AnimationLayerStack.ts:25 (sha256:d5066c5d2784d8e253bf0eaea44241ea2c0eae4d95440c7b2ec649de78a8ae09)
#[derive(Clone, Default)]
pub struct AnimationLayerStackChannelSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channel_index: f64,
    pub layer_index: f64,
}
impl PartialEq for AnimationLayerStackChannelSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationLayerStack.ts:31 (sha256:9d46d35ab194d129b0c60a82cc260ec545d9c9be36d37d06a76b937ef307f883)
#[derive(Clone, Default)]
pub struct AnimationLayerStackChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channel: AnimationChannel,
    pub sources: Vec<AnimationLayerStackChannelSource>,
}
impl PartialEq for AnimationLayerStackChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationLayerStack.ts:38 (sha256:8dcda29032e3645a19eb6084d397110b860b0be81538c518508e62fe6102e21c)
#[derive(Clone, Default)]
pub struct AnimationLayerStack {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub advance_scratch: Vec<AnimationPlayer>,
    pub blend_trees: Vec<AnimationBlendTree>,
    pub channels: Vec<AnimationLayerStackChannel>,
    pub layers: Vec<AnimationLayer>,
    pub sample_scratch: Vec<f32>,
    pub state_machines: Vec<AnimationStateMachine>,
}
impl PartialEq for AnimationLayerStack {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationLayerStack {
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
