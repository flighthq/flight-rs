// @generated from upstream/packages/types/src/CrtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CrtEffect.ts:3 (sha256:5e68df6723770b9c423c2bfb2e8b4b0535c8c16e64ebadb99798468ce7acadff)
#[derive(Clone)]
pub struct CrtEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub curvature: Option<f64>,
    pub scanline_intensity: Option<f64>,
    pub vignette: Option<f64>,
    pub aberration: Option<f64>,
}
impl PartialEq for CrtEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
