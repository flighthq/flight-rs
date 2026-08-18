// @generated from upstream/packages/types/src/AnimationStateMachine.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationBlendTree, AnimationChannel, AnimationPlayer, EasingFunction, EntityRuntime};

// Source: upstream/packages/types/src/AnimationStateMachine.ts:9 (sha256:61309ced3413df8041d6b2ae9688589d56cd8c53406af08e2d975a4c7dc40cbc)
#[derive(Clone, Default)]
pub struct AnimationStateMachineState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub blend_tree: AnimationBlendTree,
    pub name: String,
}
impl PartialEq for AnimationStateMachineState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationStateMachineState {
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

// Source: upstream/packages/types/src/AnimationStateMachine.ts:15 (sha256:454c30877bccdda403fac1dd002f9773048d6cb022fd88f025b11f2a6f45b950)
#[derive(Clone, Default)]
pub struct AnimationStateMachineChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channel: AnimationChannel,
    pub state_channel_indices: Vec<Option<f64>>,
}
impl PartialEq for AnimationStateMachineChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationStateMachine.ts:23 (sha256:fe1c6f9a7092aaf16cd71f41a141a74e9ef235eec04fa6afa7ea048439b63fde)
#[derive(Clone)]
pub struct AnimationStateMachine {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub advance_scratch: Vec<AnimationPlayer>,
    pub channels: Vec<AnimationStateMachineChannel>,
    pub current_state_index: f64,
    pub from_sample: Vec<f32>,
    pub states: Vec<AnimationStateMachineState>,
    pub to_sample: Vec<f32>,
    pub transition_curve: EasingFunction,
    pub transition_duration: f64,
    pub transition_elapsed: f64,
    pub transition_from_state_index: Option<f64>,
    pub transition_to_state_index: Option<f64>,
    pub transition_weight: f64,
}
impl PartialEq for AnimationStateMachine {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationStateMachine {
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
