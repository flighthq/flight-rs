// @generated from upstream/packages/effects/src/smaaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SmaaEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/smaaEffect.ts:3 (sha256:9c5fa8e51982ceba3092993c452263c5c6afaeac130240b525438d9105dbb230)
#[derive(Clone)]
struct CreateSmaaEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSmaaEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_smaa_effect(options: Option<FlightOmitRecord1>) -> SmaaEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
    });
    return {
        let __flight_spread_1 = options;
        SmaaEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "SmaaEffect".to_owned(),
            threshold: __flight_spread_1.threshold,
        }
    };
}
