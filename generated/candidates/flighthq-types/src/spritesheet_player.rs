// @generated from upstream/packages/types/src/SpritesheetPlayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Signal, SpritesheetAnimation};

// Source: upstream/packages/types/src/SpritesheetPlayer.ts:4 (sha256:b7441e36e6a0e7d4e6f0d030132c22cc1dc806624358cdb6326f491bdb8f43d3)
#[derive(Clone)]
pub struct SpritesheetPlayer {
    pub animation: Option<SpritesheetAnimation>,
    pub complete: bool,
    pub elapsed: f64,
    pub paused: bool,
    pub speed: f64,
    pub frame_index: f64,
    pub on_complete: Signal,
    pub on_loop: Signal,
    pub queue: Vec<SpritesheetAnimation>,
}
