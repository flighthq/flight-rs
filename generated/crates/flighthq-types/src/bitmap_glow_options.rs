// @generated from upstream/packages/types/src/BitmapGlowOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapGlowOptions.ts:3 (sha256:8a690fbf8ebcefbd6be6ef013ad219371b8d1ed973af988968d185c6e795de7a)
#[derive(Clone, Default)]
pub struct BitmapGlowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for BitmapGlowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
