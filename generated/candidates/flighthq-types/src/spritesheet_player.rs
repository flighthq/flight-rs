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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animation: Option<SpritesheetAnimation>,
    pub complete: bool,
    pub elapsed: f64,
    pub paused: bool,
    pub speed: f64,
    pub frame_index: f64,
    pub on_complete:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_loop: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub queue: Vec<SpritesheetAnimation>,
}
impl PartialEq for SpritesheetPlayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
