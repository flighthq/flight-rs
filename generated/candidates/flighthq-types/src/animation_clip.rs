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
    pub channels: Vec<AnimationChannel>,
    pub duration: f64,
}
