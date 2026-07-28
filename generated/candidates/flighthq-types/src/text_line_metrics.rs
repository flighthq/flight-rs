// @generated from upstream/packages/types/src/TextLineMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextLineMetrics.ts:1 (sha256:64a88c9ccc0e9fe3487909249cc2864640081668dda970a173ad10e1e25ea147)
#[derive(Clone)]
pub struct TextLineMetrics {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ascent: f64,
    pub descent: f64,
    pub height: f64,
    pub leading: f64,
    pub width: f64,
    pub x: f64,
}
impl PartialEq for TextLineMetrics {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
