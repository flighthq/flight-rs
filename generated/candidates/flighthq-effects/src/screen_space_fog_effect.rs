// @generated from upstream/packages/effects/src/screenSpaceFogEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ScreenSpaceFogEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<f64>,
    pub near: Option<f64>,
    pub far: Option<f64>,
    pub density: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/screenSpaceFogEffect.ts:3 (sha256:6a0bbf280ed41f5d3c962e3862f956fe874b6e6e3fec9362c09c85d2e424dd5e)
#[derive(Clone)]
struct CreateScreenSpaceFogEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateScreenSpaceFogEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_screen_space_fog_effect(options: Option<FlightOmitRecord1>) -> ScreenSpaceFogEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        color: None,
        near: None,
        far: None,
        density: None,
    });
    return {
        let __flight_spread_1 = options;
        ScreenSpaceFogEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ScreenSpaceFogEffect".to_owned(),
            color: __flight_spread_1.color,
            near: __flight_spread_1.near,
            far: __flight_spread_1.far,
            density: __flight_spread_1.density,
        }
    };
}
