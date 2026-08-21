// @generated from upstream/packages/types/src/LoopBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LoopBackend.ts:3 (sha256:2553303d0a627764378cbf08fb7a0df92efd3414472bc0605d09377d05cc640c)
#[derive(Clone)]
pub struct LoopBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub request_frame: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
                    ) -> crate::FlightValue
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub cancel_frame:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(crate::FlightValue) -> () + Send + 'static>>>,
    pub now: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
}
impl PartialEq for LoopBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
