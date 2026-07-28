// @generated from upstream/packages/effects/src/taaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::TaaEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub feedback: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/taaEffect.ts:3 (sha256:126b00855beb9589d638799a9fa0390c4d760b2d2c202cf9137473fe27fda25a)
#[derive(Clone)]
struct CreateTaaEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTaaEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_taa_effect(options: Option<FlightOmitRecord1>) -> TaaEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        feedback: None,
    });
    return {
        let __flight_spread_1 = options;
        TaaEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "TaaEffect".to_owned(),
            feedback: __flight_spread_1.feedback,
        }
    };
}
