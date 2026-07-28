// @generated from upstream/packages/types/src/TweenPropertyDetail.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TweenPropertyDetail.ts:1 (sha256:d8b13478c32c050f10440ebe6dc0b1ef9dc33cabde40a9fddb26ab0bd47a1001)
#[derive(Clone)]
pub struct TweenPropertyDetail {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub change: f64,
    pub key: String,
    pub start: f64,
}
impl PartialEq for TweenPropertyDetail {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
