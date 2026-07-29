// @generated from upstream/packages/effects/src/displacementEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::DisplacementEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub frequency: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/displacementEffect.ts:3 (sha256:afd4c94e0ffcb042de03804d5b8d728e4525beb0996e75687a5259ff6e585183)
#[derive(Clone, Default)]
struct CreateDisplacementEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDisplacementEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_displacement_effect(options: Option<FlightOmitRecord1>) -> DisplacementEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        frequency: None,
        seed: None,
    });
    return {
        let __flight_spread_1 = options;
        DisplacementEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "DisplacementEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            frequency: __flight_spread_1.frequency,
            seed: __flight_spread_1.seed,
            ..Default::default()
        }
    };
}
