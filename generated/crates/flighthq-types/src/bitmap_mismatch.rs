// @generated from upstream/packages/types/src/BitmapMismatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapMismatch.ts:7 (sha256:b8353c3131873203af07996961213c16cea9d4e44c4115307a06fa75687f17a5)
#[derive(Clone, Default)]
pub struct BitmapMismatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mismatched_pixels: f64,
    pub total_pixels: f64,
    pub fraction: f64,
    pub max_channel_delta: f64,
}
impl PartialEq for BitmapMismatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
