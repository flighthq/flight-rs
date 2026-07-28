// @generated from upstream/packages/effects/src/glitchEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GlitchEffect;

// Source: upstream/packages/effects/src/glitchEffect.ts:3 (sha256:03b2073852f91ad105439e1f2e07636700322abc0e950f3007c8e1f4ef056073)
#[derive(Clone)]
struct CreateGlitchEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGlitchEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_glitch_effect(options: Option<GlitchEffect>) -> GlitchEffect {
    let options = options.unwrap_or(GlitchEffect {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        block_size: None,
        color_shift: None,
        seed: None,
    });
    return GlitchEffect {
        kind: "GlitchEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
