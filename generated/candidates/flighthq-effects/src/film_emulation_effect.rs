// @generated from upstream/packages/effects/src/filmEmulationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FilmEmulationEffect;

// Source: upstream/packages/effects/src/filmEmulationEffect.ts:3 (sha256:c477602139c9e9cff2013aa4bc0d84e327f1e8beaa141f9a6111d80d8b50bdf1)
#[derive(Clone)]
struct CreateFilmEmulationEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateFilmEmulationEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_film_emulation_effect(options: Option<FilmEmulationEffect>) -> FilmEmulationEffect {
    let options = options.unwrap_or(FilmEmulationEffect {
        __flight_identity: std::sync::Arc::new(()),
        gate_weave: None,
        grain_intensity: None,
        halation_radius: None,
        halation_strength: None,
    });
    return FilmEmulationEffect {
        kind: "FilmEmulationEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
