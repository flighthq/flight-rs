// @generated from upstream/packages/effects/src/filmEmulationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FilmEmulationEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord2425118314 {
    pub __flight_identity: std::sync::Arc<()>,
    pub gate_weave: Option<f64>,
    pub grain_intensity: Option<f64>,
    pub halation_radius: Option<f64>,
    pub halation_strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord2425118314 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/filmEmulationEffect.ts:3 (sha256:c477602139c9e9cff2013aa4bc0d84e327f1e8beaa141f9a6111d80d8b50bdf1)
#[derive(Clone, Default)]
struct CreateFilmEmulationEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateFilmEmulationEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_film_emulation_effect(
    options: Option<FlightOmitRecord2425118314>,
) -> FilmEmulationEffect {
    let options = options.unwrap_or(FlightOmitRecord2425118314 {
        __flight_identity: std::sync::Arc::new(()),
        gate_weave: None,
        grain_intensity: None,
        halation_radius: None,
        halation_strength: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        FilmEmulationEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "FilmEmulationEffect".to_owned(),
            gate_weave: __flight_spread_1.gate_weave,
            grain_intensity: __flight_spread_1.grain_intensity,
            halation_radius: __flight_spread_1.halation_radius,
            halation_strength: __flight_spread_1.halation_strength,
            ..Default::default()
        }
    };
}
