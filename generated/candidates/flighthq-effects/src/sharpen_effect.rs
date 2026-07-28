// @generated from upstream/packages/effects/src/sharpenEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SharpenEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub amount: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/sharpenEffect.ts:3 (sha256:ece45a5e6d19b1eeb2d570ee485943e797ab701bfd20ea1b99845cfeabd6ef5e)
#[derive(Clone, Default)]
struct CreateSharpenEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSharpenEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_sharpen_effect(options: Option<FlightOmitRecord1>) -> SharpenEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        amount: None,
    });
    return {
        let __flight_spread_1 = options;
        SharpenEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "SharpenEffect".to_owned(),
            amount: __flight_spread_1.amount,
            ..Default::default()
        }
    };
}
