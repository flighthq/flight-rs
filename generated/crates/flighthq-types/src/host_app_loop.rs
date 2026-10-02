// @generated from upstream/packages/types/src/HostAppLoop.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HostAppLoop.ts:1 (sha256:0434964093d6d0a329c01fed2a0ecd5d9e8be7b96c5e8009a75a82ad2e429f9e)
#[derive(Clone)]
pub struct HostAppLoopCapability {
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
impl PartialEq for HostAppLoopCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
