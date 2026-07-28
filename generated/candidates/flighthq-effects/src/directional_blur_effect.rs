// @generated from upstream/packages/effects/src/directionalBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::DirectionalBlurEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub angle: Option<f64>,
    pub length: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/directionalBlurEffect.ts:3 (sha256:401d47059c3e52d92662d0ebb64d0ca602ef23544159ab858bd9ef7d119d512c)
#[derive(Clone)]
struct CreateDirectionalBlurEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDirectionalBlurEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_directional_blur_effect(options: Option<FlightOmitRecord1>) -> DirectionalBlurEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        angle: None,
        length: None,
        samples: None,
    });
    return {
        let __flight_spread_1 = options;
        DirectionalBlurEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "DirectionalBlurEffect".to_owned(),
            angle: __flight_spread_1.angle,
            length: __flight_spread_1.length,
            samples: __flight_spread_1.samples,
        }
    };
}
