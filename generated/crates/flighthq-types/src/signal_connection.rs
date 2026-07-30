// @generated from upstream/packages/types/src/SignalConnection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/SignalConnection.ts:11 (sha256:91655ac3159a3beced78493debe1fafd9a7bd7d8955b20146b5d6596a63265ca)
#[derive(Clone)]
pub struct SignalConnection<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub signal: Signal<T>,
    pub slot: T,
    pub connected: bool,
    pub paused: bool,
}
impl<T> PartialEq for SignalConnection<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
