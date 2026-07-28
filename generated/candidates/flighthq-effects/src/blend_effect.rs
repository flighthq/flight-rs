// @generated from upstream/packages/effects/src/blendEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AdvancedBlendMode, BlendEffect};

// Source: upstream/packages/effects/src/blendEffect.ts:9 (sha256:757aecefa83975696363ffae04cff0201f7889f8dc06ca59c5d88e81d1e4a7c9)
#[derive(Clone)]
struct CreateBlendEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBlendEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_blend_effect(mode: AdvancedBlendMode, options: Option<BlendEffect>) -> BlendEffect {
    let options = options.unwrap_or(BlendEffect {
        __flight_identity: std::sync::Arc::new(()),
        backdrop_key: None,
        opacity: None,
    });
    return BlendEffect {
        kind: "BlendEffect".to_owned(),
        mode: (mode).clone(),
        ..((options).clone()).clone()
    };
}
