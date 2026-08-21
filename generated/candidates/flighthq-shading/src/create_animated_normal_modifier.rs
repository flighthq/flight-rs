// @generated from upstream/packages/shading/src/createAnimatedNormalModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ANIMATED_NORMAL_MODIFIER_KIND as animated_normal_modifier_kind_constant,
    AnimatedNormalModifier, AnimatedNormalModifierOptions, MODIFIER_SLOT as modifier_slot_constant,
};

// Source: upstream/packages/shading/src/createAnimatedNormalModifier.ts:15 (sha256:d6a7c91590f398c56fd9d330bb4f1809b7d2d612d9820c2d48abc4c965087674)
pub fn create_animated_normal_modifier(
    options: &AnimatedNormalModifierOptions,
) -> AnimatedNormalModifier {
    let mut modifier: AnimatedNormalModifier = AnimatedNormalModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (animated_normal_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.normal).clone(),
        map: (options.map).clone(),
        scroll: (options.scroll).clone(),
        strength: Some((options.strength).clone().unwrap_or(1.0_f64)),
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
