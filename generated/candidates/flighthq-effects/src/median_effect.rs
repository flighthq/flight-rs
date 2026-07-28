// @generated from upstream/packages/effects/src/medianEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MedianEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub radius: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/medianEffect.ts:3 (sha256:242f61ead3733bc4a73a7602e314a77353bb6dff116c0b4df976d02486f19625)
#[derive(Clone)]
struct CreateMedianEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateMedianEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_median_effect(options: Option<FlightOmitRecord1>) -> MedianEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        radius: None,
    });
    return {
        let __flight_spread_1 = options;
        MedianEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "MedianEffect".to_owned(),
            radius: __flight_spread_1.radius,
        }
    };
}
