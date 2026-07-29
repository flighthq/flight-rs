// @generated from upstream/packages/shading/src/createAnimatedNormalModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ANIMATED_NORMAL_MODIFIER_KIND as animated_normal_modifier_kind_constant,
    AnimatedNormalModifier, MODIFIER_SLOT as modifier_slot_constant, Texture, Vector2Like,
};

// Source: upstream/packages/shading/src/createAnimatedNormalModifier.ts:8 (sha256:97105d620e4afa392d6e85532e6fc45385b94f13a602cd4b6770281e27eded33)
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

// Source: upstream/packages/shading/src/createAnimatedNormalModifier.ts:22 (sha256:d6a7c91590f398c56fd9d330bb4f1809b7d2d612d9820c2d48abc4c965087674)
pub fn create_animated_normal_modifier(
    options: &AnimatedNormalModifierOptions,
) -> AnimatedNormalModifier {
    let mut modifier: AnimatedNormalModifier = AnimatedNormalModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (animated_normal_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.normal).clone(),
        map: (options.map).clone(),
        scroll: (options.scroll).clone(),
        strength: Some((options.strength).unwrap_or(1.0_f64)),
        secondary_map: None,
        secondary_scroll: None,
        ..Default::default()
    };
    if ((options.secondary_map).clone()).is_some() {
        modifier.secondary_map = (options.secondary_map).clone();
    }
    if ((options.secondary_scroll).clone()).is_some() {
        modifier.secondary_scroll = (options.secondary_scroll).clone();
    }
    return modifier;
}
