// @generated from upstream/packages/effects/src/vignetteEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::VignetteEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub radius: Option<f64>,
    pub softness: Option<f64>,
    pub color: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/vignetteEffect.ts:3 (sha256:ff1e866bf28660a38b22f91ad21146cd788d71bdc84649da1088015c4d33d020)
#[derive(Clone, Default)]
struct CreateVignetteEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateVignetteEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_vignette_effect(options: Option<FlightOmitRecord1>) -> VignetteEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        radius: None,
        softness: None,
        color: None,
    });
    return {
        let __flight_spread_1 = options;
        VignetteEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "VignetteEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            radius: __flight_spread_1.radius,
            softness: __flight_spread_1.softness,
            color: __flight_spread_1.color,
            ..Default::default()
        }
    };
}
