// @generated from upstream/packages/effects/src/lensDirtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LensDirtEffect;

// Source: upstream/packages/effects/src/lensDirtEffect.ts:3 (sha256:bff4fcf7cf78bda19e886c6bf78da4dcaf93623d9b2544b8dff06dfc8115fe93)
#[derive(Clone)]
struct CreateLensDirtEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLensDirtEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lens_dirt_effect(options: Option<LensDirtEffect>) -> LensDirtEffect {
    let options = options.unwrap_or(LensDirtEffect {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        threshold: None,
        seed: None,
    });
    return LensDirtEffect {
        kind: "LensDirtEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
