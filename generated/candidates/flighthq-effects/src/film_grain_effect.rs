// @generated from upstream/packages/effects/src/filmGrainEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FilmGrainEffect;

// Source: upstream/packages/effects/src/filmGrainEffect.ts:3 (sha256:21a9c56250775731b8a903f8c21cea59c93be48a6406f372ba2412fad0b99e8b)
#[derive(Clone)]
struct CreateFilmGrainEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateFilmGrainEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_film_grain_effect(options: Option<FilmGrainEffect>) -> FilmGrainEffect {
    let options = options.unwrap_or(FilmGrainEffect {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        size: None,
        seed: None,
    });
    return FilmGrainEffect {
        kind: "FilmGrainEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
