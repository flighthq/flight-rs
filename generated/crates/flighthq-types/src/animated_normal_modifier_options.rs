// @generated from upstream/packages/types/src/AnimatedNormalModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Texture, Vector2Like};

// Source: upstream/packages/types/src/AnimatedNormalModifierOptions.ts:4 (sha256:97105d620e4afa392d6e85532e6fc45385b94f13a602cd4b6770281e27eded33)
#[derive(Clone, Default)]
pub struct AnimatedNormalModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub map: Option<Texture>,
    pub scroll: Vector2Like,
    pub strength: Option<f64>,
    pub secondary_map: Option<Texture>,
    pub secondary_scroll: Option<Vector2Like>,
}
impl PartialEq for AnimatedNormalModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
