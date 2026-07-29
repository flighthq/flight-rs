// @generated from upstream/packages/effects/src/radialBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::RadialBlurEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub center_x: Option<f64>,
    pub center_y: Option<f64>,
    pub strength: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/radialBlurEffect.ts:3 (sha256:68819aed0525c78be2fd6a3c6f6ca844a70af6ad4acb49b81f34f3b4e401d66e)
#[derive(Clone, Default)]
struct CreateRadialBlurEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateRadialBlurEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_radial_blur_effect(options: Option<FlightOmitRecord1>) -> RadialBlurEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        center_x: None,
        center_y: None,
        strength: None,
        samples: None,
    });
    return {
        let __flight_spread_1 = options;
        RadialBlurEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "RadialBlurEffect".to_owned(),
            center_x: __flight_spread_1.center_x,
            center_y: __flight_spread_1.center_y,
            strength: __flight_spread_1.strength,
            samples: __flight_spread_1.samples,
            ..Default::default()
        }
    };
}
