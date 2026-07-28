// @generated from upstream/packages/types/src/Timeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DisplayObject, TimelinePlayMode, TimelineSignals, TimelineSource};

// Source: upstream/packages/types/src/Timeline.ts:11 (sha256:22eaec57774f6ccaf44aec27c892f4911b2d8891b4dc81635fb147fe9f11f61b)
#[derive(Clone)]
pub struct Timeline {
    pub source: Option<TimelineSource>,
    pub target: Option<DisplayObject>,
    pub current_frame: f64,
    pub frame_scripts: Option<crate::OpaqueHostValue>,
    pub is_playing: bool,
    pub time_elapsed: f64,
    pub last_frame_update: f64,
    pub play_mode: TimelinePlayMode,
    pub signals: Option<TimelineSignals>,
}
