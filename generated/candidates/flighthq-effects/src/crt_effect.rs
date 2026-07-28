// @generated from upstream/packages/effects/src/crtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CrtEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub curvature: Option<f64>,
    pub scanline_intensity: Option<f64>,
    pub vignette: Option<f64>,
    pub aberration: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/crtEffect.ts:3 (sha256:45c15cf0b5f39d30b11eefbc49b6d4f93c662f10428e5a88b145a96a0f11dd34)
#[derive(Clone)]
struct CreateCrtEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateCrtEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_crt_effect(options: Option<FlightOmitRecord1>) -> CrtEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        curvature: None,
        scanline_intensity: None,
        vignette: None,
        aberration: None,
    });
    return {
        let __flight_spread_1 = options;
        CrtEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "CrtEffect".to_owned(),
            curvature: __flight_spread_1.curvature,
            scanline_intensity: __flight_spread_1.scanline_intensity,
            vignette: __flight_spread_1.vignette,
            aberration: __flight_spread_1.aberration,
        }
    };
}
