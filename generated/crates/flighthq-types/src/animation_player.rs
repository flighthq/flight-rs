// @generated from upstream/packages/types/src/AnimationPlayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationClip, AnimationClipEvent, AnimationLoopMode, EntityRuntime, Signal};

// Source: upstream/packages/types/src/AnimationPlayer.ts:18 (sha256:7737db1e82e8f1bf7d07b4ebd21bd0f18946927b22ba3b0216c93a3d85241c6d)
#[derive(Clone, Default)]
pub struct AnimationPlayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub clip: AnimationClip,
    pub loop_: bool,
    pub loop_mode: Option<AnimationLoopMode>,
    pub on_event: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(AnimationClipEvent) -> () + Send + 'static>>,
            >,
        >,
    >,
    pub on_finished:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_looped:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub playing: bool,
    pub repeat_count: Option<f64>,
    pub speed: f64,
    pub time: f64,
}
impl PartialEq for AnimationPlayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationPlayer {
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
