// @generated from upstream/packages/shading/src/createRimModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    MODIFIER_SLOT as modifier_slot_constant, RIM_MODIFIER_KIND as rim_modifier_kind_constant,
    RimModifier, RimModifierOptions,
};

// Source: upstream/packages/shading/src/createRimModifier.ts:13 (sha256:a41baac8bde45d1d5a68bc39096dfaa2c7b53f454c5bb5fa0a919be3930cd303)
pub fn create_rim_modifier(options: &RimModifierOptions) -> RimModifier {
    return RimModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (rim_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        color: options.color,
        power: Some((options.power).unwrap_or(3.0_f64)),
        intensity: Some((options.intensity).unwrap_or(1.0_f64)),
        bias: Some((options.bias).unwrap_or(0.0_f64)),
        ..Default::default()
    };
}
