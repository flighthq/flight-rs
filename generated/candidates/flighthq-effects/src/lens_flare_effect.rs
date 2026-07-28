// @generated from upstream/packages/effects/src/lensFlareEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LensFlareEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: Option<f64>,
    pub intensity: Option<f64>,
    pub ghosts: Option<f64>,
    pub halo: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/lensFlareEffect.ts:3 (sha256:59979a471e4b460a4183b4739fd11756d4610ff8c7ddeae72ca27034a6283cc6)
#[derive(Clone)]
struct CreateLensFlareEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLensFlareEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lens_flare_effect(options: Option<FlightOmitRecord1>) -> LensFlareEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        intensity: None,
        ghosts: None,
        halo: None,
    });
    return {
        let __flight_spread_1 = options;
        LensFlareEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "LensFlareEffect".to_owned(),
            threshold: __flight_spread_1.threshold,
            intensity: __flight_spread_1.intensity,
            ghosts: __flight_spread_1.ghosts,
            halo: __flight_spread_1.halo,
        }
    };
}
