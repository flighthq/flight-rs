// @generated from upstream/packages/types/src/MediaChannelSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/MediaChannelSignals.ts:2 (sha256:9b25c6ec95899a6da6bb480240484680f8a765976df53dc60c2bebda4c039847)
pub type MediaReadyState = String;

// Source: upstream/packages/types/src/MediaChannelSignals.ts:10 (sha256:76108dafdabde37d623c9d1607a7ddd92b70245189f8ee25a32eaddea46cdcdb)
#[derive(Clone)]
pub struct MediaChannelSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_buffering:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_complete:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_error:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub on_loop: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_pause: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_play: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_ready: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_seeked:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_stop: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for MediaChannelSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
