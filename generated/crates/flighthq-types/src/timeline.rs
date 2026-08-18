// @generated from upstream/packages/types/src/Timeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    FrameScript, Node2D, TimelineCueRegistry, TimelinePlayMode, TimelineSignals, TimelineSource,
};

// Source: upstream/packages/types/src/Timeline.ts:12 (sha256:aaf49d1e409fd3c60824a648cf8edd8e53ad11411923a0b5ab74c34be4da89a6)
#[derive(Clone, Default)]
pub struct Timeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
