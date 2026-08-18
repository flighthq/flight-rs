// @generated from upstream/packages/types/src/BitmapInnerGlowOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapInnerGlowOptions.ts:3 (sha256:c525168fb9135b8977ad2f7e3387f329e049827da2c9f568e66e0ccaf7813a05)
#[derive(Clone, Default)]
pub struct BitmapInnerGlowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for BitmapInnerGlowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
