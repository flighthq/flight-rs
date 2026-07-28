// @generated from upstream/packages/types/src/AnimatedNormalModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Texture, Vector2Like};

// Source: upstream/packages/types/src/AnimatedNormalModifier.ts:11 (sha256:ffe9e013055090ced18c33db3dc23624189ee7c37ad89167e5ab4878f51bab9c)
#[derive(Clone)]
pub struct AnimatedNormalModifier {
    pub kind: String,
    pub slot: String,
    pub map: Option<Texture>,
    pub scroll: Vector2Like,
    pub strength: Option<f64>,
    pub secondary_map: Option<Texture>,
    pub secondary_scroll: Option<Vector2Like>,
}

// Source: upstream/packages/types/src/AnimatedNormalModifier.ts:21 (sha256:5a1ff8bb5c3b8d845ef331b14f946160b5e56ab0b7721e194bedc09b663f69ce)
pub const ANIMATED_NORMAL_MODIFIER_KIND: &'static str = "AnimatedNormalModifier";
