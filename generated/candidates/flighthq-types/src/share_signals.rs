// @generated from upstream/packages/types/src/ShareSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ShareResult, Signal};

// Source: upstream/packages/types/src/ShareSignals.ts:6 (sha256:7522c9fddb92d36d1848e1facdffa80f784d41eb67a492b0b83c738bc4dadc61)
#[derive(Clone)]
pub struct ShareSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_share_result: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ShareResult) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for ShareSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
