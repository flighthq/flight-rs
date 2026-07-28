// @generated from upstream/packages/types/src/AnimationPlayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationClip, AnimationLoopMode, Signal};

// Source: upstream/packages/types/src/AnimationPlayer.ts:15 (sha256:0b9d7f9fbe702a6f038b3e5be956635e807a756ca48b0dd3b883bbea575664d3)
#[derive(Clone)]
pub struct AnimationPlayer {
    pub clip: AnimationClip,
    pub loop_: bool,
    pub loop_mode: Option<AnimationLoopMode>,
    pub on_finished: Option<Option<Signal>>,
    pub on_looped: Option<Option<Signal>>,
    pub playing: bool,
    pub repeat_count: Option<f64>,
    pub speed: f64,
    pub time: f64,
}
