// @generated from upstream/packages/types/src/SignalThrottleOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SignalThrottleOptions.ts:1 (sha256:9ff1029d5c3091e092ff0d59362f51d66eed27c820470f270b0a384f0b0e01b6)
#[derive(Clone, Default)]
pub struct SignalThrottleOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub leading: Option<bool>,
    pub trailing: Option<bool>,
}
impl PartialEq for SignalThrottleOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
