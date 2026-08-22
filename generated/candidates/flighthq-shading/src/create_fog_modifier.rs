// @generated from upstream/packages/shading/src/createFogModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    FOG_MODIFIER_KIND as fog_modifier_kind_constant,
    FOG_MODIFIER_MODE as fog_modifier_mode_constant, FogModifier, FogModifierOptions,
    MODIFIER_SLOT as modifier_slot_constant,
};

// Source: upstream/packages/shading/src/createFogModifier.ts:14 (sha256:f3924635c3468c4ff7344861990fe3e94bf55585ec576d0007dc937a02b53bf0)
pub fn create_fog_modifier(options: &FogModifierOptions) -> FogModifier {
    return FogModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (fog_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        color: options.color,
        mode: Some(((options.mode).clone()).unwrap_or((fog_modifier_mode_constant.linear).clone())),
        near: Some((options.near).unwrap_or(0.0_f64)),
        far: Some((options.far).unwrap_or(1.0_f64)),
        density: Some((options.density).unwrap_or(1.0_f64)),
        ..Default::default()
    };
}
