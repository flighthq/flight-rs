// @generated from upstream/packages/effects/src/whiteBalanceEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::WhiteBalanceEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub temperature: Option<f64>,
    pub tint: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/whiteBalanceEffect.ts:3 (sha256:b7080261a2c18434a5374ab542bd861f0dba9e4ba2ca8faf9fb7e2fbb6133e74)
#[derive(Clone, Default)]
struct CreateWhiteBalanceEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateWhiteBalanceEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_white_balance_effect(options: Option<FlightOmitRecord1>) -> WhiteBalanceEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        temperature: None,
        tint: None,
    });
    return {
        let __flight_spread_1 = options;
        WhiteBalanceEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "WhiteBalanceEffect".to_owned(),
            temperature: __flight_spread_1.temperature,
            tint: __flight_spread_1.tint,
            ..Default::default()
        }
    };
}
