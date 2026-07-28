// @generated from upstream/packages/effects/src/blurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BlurEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/blurEffect.ts:6 (sha256:c201dc944b5cc997ed7759922a210418e3312ca5fa5dc98527912545d52f144e)
#[derive(Clone)]
struct CreateBlurEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBlurEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_blur_effect(options: Option<FlightOmitRecord1>) -> BlurEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        blur_x: None,
        blur_y: None,
    });
    return {
        let __flight_spread_1 = options;
        BlurEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BlurEffect".to_owned(),
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
        }
    };
}
