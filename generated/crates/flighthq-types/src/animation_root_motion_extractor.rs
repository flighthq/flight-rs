// @generated from upstream/packages/types/src/AnimationRootMotionExtractor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationChannel, AnimationClip, EntityRuntime};

// Source: upstream/packages/types/src/AnimationRootMotionExtractor.ts:8 (sha256:7b66783df68ff872ad421ae76f22aa65a493abbe89363e655126bd5366f4c849)
#[derive(Clone, Default)]
pub struct AnimationRootMotionExtractor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub channel: AnimationChannel,
    pub channel_index: f64,
    pub clip: AnimationClip,
    pub cycle_delta: Vec<f32>,
    pub from_motion: Vec<f32>,
    pub from_sample: Vec<f32>,
    pub power_scratch: Vec<f32>,
    pub start_sample: Vec<f32>,
    pub to_motion: Vec<f32>,
    pub to_sample: Vec<f32>,
}
impl PartialEq for AnimationRootMotionExtractor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationRootMotionExtractor {
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
