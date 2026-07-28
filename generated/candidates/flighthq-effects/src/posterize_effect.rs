// @generated from upstream/packages/effects/src/posterizeEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::PosterizeEffect;

// Source: upstream/packages/effects/src/posterizeEffect.ts:3 (sha256:b10227aa045230cc1a29b322f74d83f21cf6ba06a76918beed134fd9eb4f9e45)
#[derive(Clone)]
struct CreatePosterizeEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreatePosterizeEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_posterize_effect(options: Option<PosterizeEffect>) -> PosterizeEffect {
    let options = options.unwrap_or(PosterizeEffect {
        __flight_identity: std::sync::Arc::new(()),
        levels: None,
    });
    return PosterizeEffect {
        kind: "PosterizeEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
