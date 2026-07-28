// @generated from upstream/packages/effects/src/autoExposureEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::AutoExposureEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub adaptation_speed: Option<f64>,
    pub exposure_compensation: Option<f64>,
    pub max_exposure: Option<f64>,
    pub min_exposure: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/autoExposureEffect.ts:3 (sha256:747c0be704448251bac3207e5d4e1c414cd5a0fffcf06e7482a98db6f499be88)
#[derive(Clone, Default)]
struct CreateAutoExposureEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateAutoExposureEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_auto_exposure_effect(options: Option<FlightOmitRecord1>) -> AutoExposureEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        adaptation_speed: None,
        exposure_compensation: None,
        max_exposure: None,
        min_exposure: None,
    });
    return {
        let __flight_spread_1 = options;
        AutoExposureEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "AutoExposureEffect".to_owned(),
            adaptation_speed: __flight_spread_1.adaptation_speed,
            exposure_compensation: __flight_spread_1.exposure_compensation,
            max_exposure: __flight_spread_1.max_exposure,
            min_exposure: __flight_spread_1.min_exposure,
            ..Default::default()
        }
    };
}
