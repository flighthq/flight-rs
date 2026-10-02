// @generated from upstream/packages/types/src/SignalTrackedConnectOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SignalScope;

// Source: upstream/packages/types/src/SignalTrackedConnectOptions.ts:9 (sha256:d74f06545d5ff30c2319644d892bb22bcaaa8c6dbca59a4201e2b0b5fb06d188)
#[derive(Clone, Default)]
pub struct SignalTrackedConnectOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub once: Option<bool>,
    pub priority: Option<f64>,
    pub scope: Option<SignalScope>,
}
impl PartialEq for SignalTrackedConnectOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
