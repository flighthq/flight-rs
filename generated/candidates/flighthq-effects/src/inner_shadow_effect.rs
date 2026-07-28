// @generated from upstream/packages/effects/src/innerShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::InnerShadowEffect;

// Source: upstream/packages/effects/src/innerShadowEffect.ts:4 (sha256:0fb9b1b51145317d567bd88695272e999cc3a21df22ed8d3c1868eb3615d8fb7)
#[derive(Clone)]
struct CreateInnerShadowEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInnerShadowEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_inner_shadow_effect(options: Option<InnerShadowEffect>) -> InnerShadowEffect {
    let options = options.unwrap_or(InnerShadowEffect {
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
    return InnerShadowEffect {
        kind: "InnerShadowEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
