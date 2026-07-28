// @generated from upstream/packages/effects/src/scanlinesEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ScanlinesEffect;

// Source: upstream/packages/effects/src/scanlinesEffect.ts:3 (sha256:14bd9d022c429f61b5f5eeb1af6c7488c4a0dbb23850dda7c1fa7d6d0b2a7bf9)
#[derive(Clone)]
struct CreateScanlinesEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateScanlinesEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_scanlines_effect(options: Option<ScanlinesEffect>) -> ScanlinesEffect {
    let options = options.unwrap_or(ScanlinesEffect {
        __flight_identity: std::sync::Arc::new(()),
        count: None,
        intensity: None,
    });
    return ScanlinesEffect {
        kind: "ScanlinesEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
