// @generated from upstream/packages/types/src/TimelineSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Node2D, TimelineCue, TimelineLabel};

// Source: upstream/packages/types/src/TimelineSource.ts:11 (sha256:48c002a80cb4ca1bfe9a263e1c8a1ef8e82f67918880ff58f093312a7acc1a6e)
#[derive(Clone)]
pub struct TimelineSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub total_frames: f64,
    pub labels: Vec<TimelineLabel>,
    pub cues: Vec<TimelineCue>,
    pub frame_rate: Option<f64>,
    pub construct_frame:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node2D, f64) -> () + Send + 'static>>>,
}
impl PartialEq for TimelineSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TimelineSource {
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
