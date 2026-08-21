// @generated from upstream/packages/effects/src/halftoneEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HalftoneEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1456389143 {
    pub __flight_identity: std::sync::Arc<()>,
    pub scale: Option<f64>,
    pub angle: Option<f64>,
}
impl PartialEq for FlightOmitRecord1456389143 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/halftoneEffect.ts:3 (sha256:c0a7642f369912c99adfe0329fb649695a982e51a91ec5c9f4b041fc55c09c50)
#[derive(Clone, Default)]
struct CreateHalftoneEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateHalftoneEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_halftone_effect(options: Option<FlightOmitRecord1456389143>) -> HalftoneEffect {
    let options = options.unwrap_or(FlightOmitRecord1456389143 {
        __flight_identity: std::sync::Arc::new(()),
        scale: None,
        angle: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        HalftoneEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "HalftoneEffect".to_owned(),
            scale: __flight_spread_1.scale,
            angle: __flight_spread_1.angle,
            ..Default::default()
        }
    };
}
