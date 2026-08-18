// @generated from upstream/packages/types/src/BitmapInnerShadowOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapInnerShadowOptions.ts:3 (sha256:15a81da87400d1edfa0959703e257b8c722cb0ad8dc48b2dd12a60cfc57e44ba)
#[derive(Clone, Default)]
pub struct BitmapInnerShadowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
}
impl PartialEq for BitmapInnerShadowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
