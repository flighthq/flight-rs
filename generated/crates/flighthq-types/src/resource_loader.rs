// @generated from upstream/packages/types/src/ResourceLoader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ResourceLoadReport, Signal};

// Source: upstream/packages/types/src/ResourceLoader.ts:4 (sha256:e3550276e310c2635b9749863ee0dcc64b1809e9ecee8dae906ecb6e6dd0fd03)
#[derive(Clone)]
pub struct ResourceLoader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_cancel:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_complete: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Vec<ResourceLoadReport>) -> () + Send + 'static>>,
        >,
    >,
    pub on_error: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue, String) -> () + Send + 'static>>,
        >,
    >,
    pub on_pause: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_progress:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
    pub on_resume:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
