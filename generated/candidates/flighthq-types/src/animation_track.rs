// @generated from upstream/packages/types/src/AnimationTrack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationInterpolation, EasingFunction};

// Source: upstream/packages/types/src/AnimationTrack.ts:11 (sha256:ac84cd5cc037d1ec9ee5c821ffbe324e19534881eb9222bc13f3dbc685ce5825)
#[derive(Clone)]
pub struct AnimationTrack {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub interpolation: AnimationInterpolation,
    pub times: Vec<f64>,
    pub values: Vec<f64>,
    pub components: f64,
    pub quaternion: bool,
    pub easing: Option<EasingFunction>,
}
impl PartialEq for AnimationTrack {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
