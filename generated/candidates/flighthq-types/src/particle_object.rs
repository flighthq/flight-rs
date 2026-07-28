// @generated from upstream/packages/types/src/ParticleObject.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorTransform;

// Source: upstream/packages/types/src/ParticleObject.ts:5 (sha256:0cbee5ecbcb67d6d1a2432ec072bea4fbced4c50b41852989623aeba41020e1b)
#[derive(Clone, Default)]
pub struct ParticleObject {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
    pub alpha: f64,
    pub visible: bool,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for ParticleObject {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
