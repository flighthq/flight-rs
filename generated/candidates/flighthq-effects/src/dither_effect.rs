// @generated from upstream/packages/effects/src/ditherEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::DitherEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord2751102098 {
    pub __flight_identity: std::sync::Arc<()>,
    pub levels: Option<f64>,
}
impl PartialEq for FlightOmitRecord2751102098 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/ditherEffect.ts:3 (sha256:a94783752ee994f0a6723ed9a6a297fa4a4d02f01bed52b36be69b8116d2142b)
#[derive(Clone, Default)]
struct CreateDitherEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDitherEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_dither_effect(options: Option<FlightOmitRecord2751102098>) -> DitherEffect {
    let options = options.unwrap_or(FlightOmitRecord2751102098 {
        __flight_identity: std::sync::Arc::new(()),
        levels: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        DitherEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "DitherEffect".to_owned(),
            levels: __flight_spread_1.levels,
            ..Default::default()
        }
    };
}
