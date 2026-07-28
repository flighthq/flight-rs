// @generated from upstream/packages/effects/src/dropShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::DropShadowEffect;

// Source: upstream/packages/effects/src/dropShadowEffect.ts:4 (sha256:3ab0759bb38792b708d22eb5f5a8d8352d1d2ba3181a9bd200650c5d0a6f40d9)
#[derive(Clone)]
struct CreateDropShadowEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDropShadowEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_drop_shadow_effect(options: Option<DropShadowEffect>) -> DropShadowEffect {
    let options = options.unwrap_or(DropShadowEffect {
        __flight_identity: std::sync::Arc::new(()),
        alpha: None,
        angle: None,
        blur_x: None,
        blur_y: None,
        color: None,
        distance: None,
        quality: None,
        source_mode: None,
        strength: None,
    });
    return DropShadowEffect {
        kind: "DropShadowEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
