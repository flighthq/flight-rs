// @generated from upstream/packages/types/src/BitmapGradientGlowOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapGradientGlowOptions.ts:1 (sha256:5934bc8bbdd8f85c1b5ec613312ae1ec3003b2e0ea21b365a6e52226aee60329)
#[derive(Clone, Default)]
pub struct BitmapGradientGlowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for BitmapGradientGlowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
