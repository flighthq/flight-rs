// @generated from upstream/packages/types/src/Timeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, FrameScript, Node2D, TimelineCueRegistry, TimelinePlayMode, TimelineSignals,
    TimelineSource,
};

// Source: upstream/packages/types/src/Timeline.ts:13 (sha256:a6a870efc5bdcb320b8dcaea5b90e2f01c986c1c2936690ab25698da1d7af86d)
#[derive(Clone, Default)]
pub struct Timeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub source: Option<TimelineSource>,
    pub target: Option<Node2D>,
    pub current_frame: f64,
    pub cue_registry: Option<TimelineCueRegistry>,
    pub frame_scripts: Option<Vec<(f64, FrameScript)>>,
    pub is_playing: bool,
    pub time_elapsed: f64,
    pub last_frame_update: f64,
    pub play_mode: TimelinePlayMode,
    pub signals: Option<TimelineSignals>,
}
impl PartialEq for Timeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Timeline {
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
