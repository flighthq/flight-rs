// @generated from upstream/packages/effects/src/radialBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::RadialBlurEffect;

// Source: upstream/packages/effects/src/radialBlurEffect.ts:3 (sha256:68819aed0525c78be2fd6a3c6f6ca844a70af6ad4acb49b81f34f3b4e401d66e)
#[derive(Clone)]
struct CreateRadialBlurEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateRadialBlurEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_radial_blur_effect(options: Option<RadialBlurEffect>) -> RadialBlurEffect {
    let options = options.unwrap_or(RadialBlurEffect {
        __flight_identity: std::sync::Arc::new(()),
        center_x: None,
        center_y: None,
        strength: None,
        samples: None,
    });
    return RadialBlurEffect {
        kind: "RadialBlurEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
