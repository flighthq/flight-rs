// @generated from upstream/packages/types/src/AnimationClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AnimationChannel;

// Source: upstream/packages/types/src/AnimationClip.ts:6 (sha256:b540e4865821c497739d7bba0d83302f5ebabca6dc06b33ff3efb5a1aa82218a)
#[derive(Clone)]
pub struct AnimationClip {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channels: Vec<AnimationChannel>,
    pub duration: f64,
}
impl PartialEq for AnimationClip {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
