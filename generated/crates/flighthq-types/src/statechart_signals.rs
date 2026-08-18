// @generated from upstream/packages/types/src/StatechartSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/StatechartSignals.ts:5 (sha256:f34a3d73eb96b5530f76c1dfc8763fbcfe46e139e333e8ec775d3d3dcb6a9911)
#[derive(Clone)]
pub struct StatechartSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_state_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for StatechartSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
