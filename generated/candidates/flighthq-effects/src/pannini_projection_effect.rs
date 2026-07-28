// @generated from upstream/packages/effects/src/panniniProjectionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::PanniniProjectionEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub compression: Option<f64>,
    pub crop: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/panniniProjectionEffect.ts:3 (sha256:baefe2699542025de3aa3d1c53564b01d183b79364be530356c8e6d0841ed62e)
#[derive(Clone)]
struct CreatePanniniProjectionEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreatePanniniProjectionEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_pannini_projection_effect(
    options: Option<FlightOmitRecord1>,
) -> PanniniProjectionEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        compression: None,
        crop: None,
    });
    return {
        let __flight_spread_1 = options;
        PanniniProjectionEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "PanniniProjectionEffect".to_owned(),
            compression: __flight_spread_1.compression,
            crop: __flight_spread_1.crop,
        }
    };
}
