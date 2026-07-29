// @generated from upstream/packages/types/src/TextShaperSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Signal, TextShaperBackend};

// Source: upstream/packages/types/src/TextShaperSignals.ts:3 (sha256:9648998b1896e81a67ad52f8e75e2b8bd43c8a9fb1cadedde301263663c20683)
#[derive(Clone)]
pub struct TextShaperSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_backend_changed: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Option<TextShaperBackend>) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for TextShaperSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
