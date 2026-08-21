// @generated from upstream/packages/effects/src/scanlinesEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ScanlinesEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1128315592 {
    pub __flight_identity: std::sync::Arc<()>,
    pub count: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for FlightOmitRecord1128315592 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/scanlinesEffect.ts:3 (sha256:14bd9d022c429f61b5f5eeb1af6c7488c4a0dbb23850dda7c1fa7d6d0b2a7bf9)
#[derive(Clone, Default)]
struct CreateScanlinesEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateScanlinesEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_scanlines_effect(options: Option<FlightOmitRecord1128315592>) -> ScanlinesEffect {
    let options = options.unwrap_or(FlightOmitRecord1128315592 {
        __flight_identity: std::sync::Arc::new(()),
        count: None,
        intensity: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        ScanlinesEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ScanlinesEffect".to_owned(),
            count: __flight_spread_1.count,
            intensity: __flight_spread_1.intensity,
            ..Default::default()
        }
    };
}
