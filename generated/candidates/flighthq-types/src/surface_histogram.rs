// @generated from upstream/packages/types/src/SurfaceHistogram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SurfaceHistogram.ts:1 (sha256:fe3a4ec51065cf26290c1cf35547b7e8537b2a68f2d387b53f2f87cc0d961648)
#[derive(Clone, Default)]
pub struct SurfaceHistogram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Vec<f64>,
    pub blue: Vec<f64>,
    pub green: Vec<f64>,
    pub red: Vec<f64>,
}
impl PartialEq for SurfaceHistogram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
