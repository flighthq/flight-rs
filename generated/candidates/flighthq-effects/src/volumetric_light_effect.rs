// @generated from upstream/packages/effects/src/volumetricLightEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::VolumetricLightEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub density: Option<f64>,
    pub light_color: Option<f64>,
    pub light_x: Option<f64>,
    pub light_y: Option<f64>,
    pub samples: Option<f64>,
    pub scattering: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/volumetricLightEffect.ts:3 (sha256:169af6783f03baa47eb6fa38ca00da245644f336396e7b6809f9969533e0bafd)
#[derive(Clone)]
struct CreateVolumetricLightEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateVolumetricLightEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_volumetric_light_effect(options: Option<FlightOmitRecord1>) -> VolumetricLightEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        density: None,
        light_color: None,
        light_x: None,
        light_y: None,
        samples: None,
        scattering: None,
    });
    return {
        let __flight_spread_1 = options;
        VolumetricLightEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "VolumetricLightEffect".to_owned(),
            density: __flight_spread_1.density,
            light_color: __flight_spread_1.light_color,
            light_x: __flight_spread_1.light_x,
            light_y: __flight_spread_1.light_y,
            samples: __flight_spread_1.samples,
            scattering: __flight_spread_1.scattering,
        }
    };
}
