// @generated from upstream/packages/types/src/SignalConnectOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SignalConnectOptions.ts:1 (sha256:f0c2369bb278f472da6dd918b39231ccfff01d9eb9835dd4b5cf3b34070af187)
#[derive(Clone, Default)]
pub struct SignalConnectOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub once: Option<bool>,
    pub priority: Option<f64>,
}
impl PartialEq for SignalConnectOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
