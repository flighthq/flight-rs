// @generated from upstream/packages/types/src/SpritesheetAnimationData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, SpritesheetAnimationDirection};

// Source: upstream/packages/types/src/SpritesheetAnimationData.ts:4 (sha256:a374482da96d0ae2dfbc18cd66e4eb605101957ee7a3a8fc5b9542baf3966da8)
#[derive(Clone, Default)]
pub struct SpritesheetAnimationData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub direction: SpritesheetAnimationDirection,
    pub frame_duration: f64,
    pub frame_durations: Option<Vec<f64>>,
    pub frame_names: Vec<String>,
    pub repeat_count: f64,
    pub name: String,
    pub origin_x: f64,
    pub origin_y: f64,
}
impl PartialEq for SpritesheetAnimationData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SpritesheetAnimationData {
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
