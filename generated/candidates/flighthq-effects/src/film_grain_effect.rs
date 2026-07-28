// @generated from upstream/packages/effects/src/filmGrainEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FilmGrainEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub size: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/filmGrainEffect.ts:3 (sha256:21a9c56250775731b8a903f8c21cea59c93be48a6406f372ba2412fad0b99e8b)
#[derive(Clone, Default)]
struct CreateFilmGrainEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateFilmGrainEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_film_grain_effect(options: Option<FlightOmitRecord1>) -> FilmGrainEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        size: None,
        seed: None,
    });
    return {
        let __flight_spread_1 = options;
        FilmGrainEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "FilmGrainEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            size: __flight_spread_1.size,
            seed: __flight_spread_1.seed,
            ..Default::default()
        }
    };
}
