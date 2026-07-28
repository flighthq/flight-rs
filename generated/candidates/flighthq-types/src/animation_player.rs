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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: AnimationClip,
    pub loop_: bool,
    pub loop_mode: Option<AnimationLoopMode>,
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
