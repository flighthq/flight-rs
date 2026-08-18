// @generated from upstream/packages/types/src/BitmapDropShadowOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapDropShadowOptions.ts:3 (sha256:e0ac0fd200249448cb889365994a4bc077cc0b4ae2aa518a439fbc514c83ea4e)
#[derive(Clone, Default)]
pub struct BitmapDropShadowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for BitmapDropShadowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
