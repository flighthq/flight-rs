// @generated from upstream/packages/types/src/ResourceLoader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ResourceLoadReport, Signal};

// Source: upstream/packages/types/src/ResourceLoader.ts:4 (sha256:d464dbdb897420a48df91f33c33d1d29edfdae91f7295ae21b89c492ccf7e595)
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
            std::sync::Mutex<Box<dyn FnMut(crate::FlightValue, String) -> () + Send + 'static>>,
        >,
    >,
    pub on_pause: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_progress:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_resume:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
