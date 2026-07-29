// @generated from upstream/packages/effects/src/motionBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MotionBlurEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/motionBlurEffect.ts:3 (sha256:527237d0c12d359f1246316cf7e8c878a73ce69edfc04420cc799245d42234cd)
#[derive(Clone, Default)]
struct CreateMotionBlurEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateMotionBlurEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_motion_blur_effect(options: Option<FlightOmitRecord1>) -> MotionBlurEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        samples: None,
    });
    return {
        let __flight_spread_1 = options;
        MotionBlurEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "MotionBlurEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            samples: __flight_spread_1.samples,
            ..Default::default()
        }
    };
}
