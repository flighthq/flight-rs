// @generated from upstream/packages/effects/src/outerGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::OuterGlowEffect;

// Source: upstream/packages/effects/src/outerGlowEffect.ts:4 (sha256:9e9b60be5f480a66fef755ca2d03553cc8be4a2e4ef7b2029ffd4663a8a3762c)
#[derive(Clone)]
struct CreateOuterGlowEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateOuterGlowEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_outer_glow_effect(options: Option<OuterGlowEffect>) -> OuterGlowEffect {
    let options = options.unwrap_or(OuterGlowEffect {
        __flight_identity: std::sync::Arc::new(()),
        alpha: None,
        blur_x: None,
        blur_y: None,
        color: None,
        quality: None,
        source_mode: None,
        strength: None,
    });
    return OuterGlowEffect {
        kind: "OuterGlowEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
