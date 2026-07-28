// @generated from upstream/packages/effects/src/tiltShiftEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::TiltShiftEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub center: Option<f64>,
    pub width: Option<f64>,
    pub blur: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:3 (sha256:2ff62cd921c5f37d54bffaed6b7d2015ef8107f8a5f8ca2c084ded52800c023d)
#[derive(Clone, Default)]
struct CreateTiltShiftEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTiltShiftEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tilt_shift_effect(options: Option<FlightOmitRecord1>) -> TiltShiftEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        center: None,
        width: None,
        blur: None,
    });
    return {
        let __flight_spread_1 = options;
        TiltShiftEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "TiltShiftEffect".to_owned(),
            center: __flight_spread_1.center,
            width: __flight_spread_1.width,
            blur: __flight_spread_1.blur,
            ..Default::default()
        }
    };
}
