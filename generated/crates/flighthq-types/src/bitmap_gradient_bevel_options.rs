// @generated from upstream/packages/types/src/BitmapGradientBevelOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BitmapBevelType;

// Source: upstream/packages/types/src/BitmapGradientBevelOptions.ts:3 (sha256:307bfb2ceb94a4971dc35171139a7328552f9ffa06a0d76954467b57d86f1f46)
#[derive(Clone, Default)]
pub struct BitmapGradientBevelOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub angle: Option<f64>,
    pub distance: Option<f64>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub intensity: Option<f64>,
    pub type_: Option<BitmapBevelType>,
}
impl PartialEq for BitmapGradientBevelOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
