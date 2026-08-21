// @generated from upstream/packages/effects/src/lensDirtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LensDirtEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord651454342 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub threshold: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FlightOmitRecord651454342 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/lensDirtEffect.ts:3 (sha256:bff4fcf7cf78bda19e886c6bf78da4dcaf93623d9b2544b8dff06dfc8115fe93)
#[derive(Clone, Default)]
struct CreateLensDirtEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLensDirtEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lens_dirt_effect(options: Option<FlightOmitRecord651454342>) -> LensDirtEffect {
    let options = options.unwrap_or(FlightOmitRecord651454342 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        threshold: None,
        seed: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        LensDirtEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "LensDirtEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            threshold: __flight_spread_1.threshold,
            seed: __flight_spread_1.seed,
            ..Default::default()
        }
    };
}
