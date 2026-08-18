// @generated from upstream/packages/types/src/BitmapSharpenOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapSharpenOptions.ts:1 (sha256:9a33d303df677725810a9a07b8c14983a93af01b349dba0062a5a15748dec095)
#[derive(Clone, Default)]
pub struct BitmapSharpenOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub amount: Option<f64>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
}
impl PartialEq for BitmapSharpenOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
