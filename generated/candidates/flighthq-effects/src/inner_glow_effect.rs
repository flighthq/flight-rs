// @generated from upstream/packages/effects/src/innerGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::InnerGlowEffect;

// Source: upstream/packages/effects/src/innerGlowEffect.ts:4 (sha256:8ffdefb7949478edf386983ea702db37c25697510f18477650ed5fbecc1b3946)
#[derive(Clone)]
struct CreateInnerGlowEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInnerGlowEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_inner_glow_effect(options: Option<InnerGlowEffect>) -> InnerGlowEffect {
    let options = options.unwrap_or(InnerGlowEffect {
        __flight_identity: std::sync::Arc::new(()),
        alpha: None,
        blur_x: None,
        blur_y: None,
        color: None,
        quality: None,
        source_mode: None,
        strength: None,
    });
    return InnerGlowEffect {
        kind: "InnerGlowEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
