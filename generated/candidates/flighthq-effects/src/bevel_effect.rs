// @generated from upstream/packages/effects/src/bevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BevelEffect;

// Source: upstream/packages/effects/src/bevelEffect.ts:4 (sha256:14b78e6e86a452b7dd16a5c6bb5194e2f2d6a40a72cc2350199439881dc149e2)
#[derive(Clone)]
struct CreateBevelEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBevelEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_bevel_effect(options: Option<BevelEffect>) -> BevelEffect {
    let options = options.unwrap_or(BevelEffect {
        __flight_identity: std::sync::Arc::new(()),
        angle: None,
        bevel_type: None,
        blur_x: None,
        blur_y: None,
        distance: None,
        highlight_alpha: None,
        highlight_color: None,
        quality: None,
        shadow_alpha: None,
        shadow_color: None,
        source_mode: None,
        strength: None,
    });
    return BevelEffect {
        kind: "BevelEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
