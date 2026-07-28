// @generated from upstream/packages/effects/src/halftoneEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HalftoneEffect;

// Source: upstream/packages/effects/src/halftoneEffect.ts:3 (sha256:c0a7642f369912c99adfe0329fb649695a982e51a91ec5c9f4b041fc55c09c50)
#[derive(Clone)]
struct CreateHalftoneEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateHalftoneEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_halftone_effect(options: Option<HalftoneEffect>) -> HalftoneEffect {
    let options = options.unwrap_or(HalftoneEffect {
        __flight_identity: std::sync::Arc::new(()),
        scale: None,
        angle: None,
    });
    return HalftoneEffect {
        kind: "HalftoneEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
