// @generated from upstream/packages/effects/src/pixelateEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::PixelateEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub size: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/pixelateEffect.ts:3 (sha256:db945901f90476fd1ead5a7daf4df2b6619d7a93691e6e6f8b9eac4949e89189)
#[derive(Clone)]
struct CreatePixelateEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreatePixelateEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_pixelate_effect(options: Option<FlightOmitRecord1>) -> PixelateEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        size: None,
    });
    return {
        let __flight_spread_1 = options;
        PixelateEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "PixelateEffect".to_owned(),
            size: __flight_spread_1.size,
        }
    };
}
