// @generated from upstream/packages/types/src/ResourceLoaderItemSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ResourceLoaderItemSignals.ts:3 (sha256:f1a71d116e06af2fa29ebdc39336b424dcadd9cb8a10c2bd25fe46efc486d1b8)
#[derive(Clone)]
pub struct ResourceLoaderItemSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_item_complete: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(String, crate::FlightValue) -> () + Send + 'static>>,
        >,
    >,
    pub on_item_error: Signal<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(String, crate::FlightValue, f64) -> () + Send + 'static>,
            >,
        >,
    >,
    pub on_item_retry: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64, f64) -> () + Send + 'static>>>,
    >,
    pub on_item_start:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
}
impl PartialEq for ResourceLoaderItemSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
