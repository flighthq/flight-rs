// @generated from upstream/packages/types/src/SurfaceMismatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SurfaceMismatch.ts:7 (sha256:5f3316dd663fb43b928a0aa7c6a2a7dcc8eb53576c487164eac51368915db919)
#[derive(Clone)]
pub struct SurfaceMismatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mismatched_pixels: f64,
    pub total_pixels: f64,
    pub fraction: f64,
    pub max_channel_delta: f64,
}
impl PartialEq for SurfaceMismatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
