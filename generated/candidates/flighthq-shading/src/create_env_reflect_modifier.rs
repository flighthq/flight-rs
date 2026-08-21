// @generated from upstream/packages/shading/src/createEnvReflectModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ENV_REFLECT_MODIFIER_KIND as env_reflect_modifier_kind_constant, EnvReflectModifier,
    EnvReflectModifierOptions, MODIFIER_SLOT as modifier_slot_constant,
};

// Source: upstream/packages/shading/src/createEnvReflectModifier.ts:14 (sha256:b3df03fb0ea0cd5e5f773666c81e21f3b30a5f7f61c0671804e0346ef5e5c999)
pub fn create_env_reflect_modifier(
    options: Option<EnvReflectModifierOptions>,
) -> EnvReflectModifier {
    return EnvReflectModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (env_reflect_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        tint: (options.as_ref().and_then(|value| value.tint))
            .clone()
            .unwrap_or(4294967295.0_f64),
        intensity: Some(
            (options.as_ref().and_then(|value| value.intensity))
                .clone()
                .unwrap_or(1.0_f64),
        ),
        fresnel_bias: Some(
            (options.as_ref().and_then(|value| value.fresnel_bias))
                .clone()
                .unwrap_or(0.04_f64),
        ),
        roughness: Some(
            (options.as_ref().and_then(|value| value.roughness))
                .clone()
                .unwrap_or(0.0_f64),
        ),
        ..Default::default()
    };
}
