// @generated from upstream/packages/effects/src/vignetteEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::VignetteEffect;

// Source: upstream/packages/effects/src/vignetteEffect.ts:3 (sha256:ff1e866bf28660a38b22f91ad21146cd788d71bdc84649da1088015c4d33d020)
#[derive(Clone)]
struct CreateVignetteEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateVignetteEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_vignette_effect(options: Option<VignetteEffect>) -> VignetteEffect {
    let options = options.unwrap_or(VignetteEffect {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        radius: None,
        softness: None,
        color: None,
    });
    return VignetteEffect {
        kind: "VignetteEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
