// @generated from upstream/packages/effects/src/godRaysEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GodRaysEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord2075458950 {
    pub __flight_identity: std::sync::Arc<()>,
    pub center_x: Option<f64>,
    pub center_y: Option<f64>,
    pub density: Option<f64>,
    pub decay: Option<f64>,
    pub weight: Option<f64>,
    pub exposure: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for FlightOmitRecord2075458950 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/godRaysEffect.ts:3 (sha256:1f649a2052d40e3944766a2f2827a6fdfb1db84b877ce1bad8b1be86e206b95b)
#[derive(Clone, Default)]
struct CreateGodRaysEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGodRaysEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_god_rays_effect(options: Option<FlightOmitRecord2075458950>) -> GodRaysEffect {
    let options = options.unwrap_or(FlightOmitRecord2075458950 {
        __flight_identity: std::sync::Arc::new(()),
        center_x: None,
        center_y: None,
        density: None,
        decay: None,
        weight: None,
        exposure: None,
        samples: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        GodRaysEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "GodRaysEffect".to_owned(),
            center_x: __flight_spread_1.center_x,
            center_y: __flight_spread_1.center_y,
            density: __flight_spread_1.density,
            decay: __flight_spread_1.decay,
            weight: __flight_spread_1.weight,
            exposure: __flight_spread_1.exposure,
            samples: __flight_spread_1.samples,
            ..Default::default()
        }
    };
}
