// @generated from upstream/packages/effects/src/toneMapEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ToneMapEffect, ToneMapOperator};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub operator: Option<ToneMapOperator>,
    pub exposure: Option<f64>,
    pub white: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/toneMapEffect.ts:3 (sha256:911baa094a5c4361d603a14376b771ce11472f9a4b2778bd07ba77293ff2b88a)
#[derive(Clone, Default)]
struct CreateToneMapEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateToneMapEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tone_map_effect(options: Option<FlightOmitRecord1>) -> ToneMapEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        operator: None,
        exposure: None,
        white: None,
    });
    return {
        let __flight_spread_1 = options;
        ToneMapEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ToneMapEffect".to_owned(),
            operator: (__flight_spread_1.operator).clone(),
            exposure: __flight_spread_1.exposure,
            white: __flight_spread_1.white,
            ..Default::default()
        }
    };
}
