// @generated from upstream/packages/shading/src/createEnvReflectModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ENV_REFLECT_MODIFIER_KIND as env_reflect_modifier_kind_constant, EnvReflectModifier,
    MODIFIER_SLOT as modifier_slot_constant,
};

// Source: upstream/packages/shading/src/createEnvReflectModifier.ts:7 (sha256:dbf49aefbe4ffe9612339955a74a9bde2c3a25b4ddeeae0b3d484ddd27a61eec)
#[derive(Clone, Default)]
pub struct EnvReflectModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tint: Option<f64>,
    pub intensity: Option<f64>,
    pub fresnel_bias: Option<f64>,
    pub roughness: Option<f64>,
}
impl PartialEq for EnvReflectModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/createEnvReflectModifier.ts:20 (sha256:b3df03fb0ea0cd5e5f773666c81e21f3b30a5f7f61c0671804e0346ef5e5c999)
pub fn create_env_reflect_modifier(
    options: Option<EnvReflectModifierOptions>,
) -> EnvReflectModifier {
    return EnvReflectModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (env_reflect_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        tint: (options.as_ref().and_then(|value| value.tint)).unwrap_or(4294967295.0_f64),
        intensity: Some((options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64)),
        fresnel_bias: Some(
            (options.as_ref().and_then(|value| value.fresnel_bias)).unwrap_or(0.04_f64),
        ),
        roughness: Some((options.as_ref().and_then(|value| value.roughness)).unwrap_or(0.0_f64)),
        ..Default::default()
    };
}
