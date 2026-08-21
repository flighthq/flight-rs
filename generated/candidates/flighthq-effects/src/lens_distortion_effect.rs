// @generated from upstream/packages/effects/src/lensDistortionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LensDistortionEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1694725934 {
    pub __flight_identity: std::sync::Arc<()>,
    pub amount: Option<f64>,
    pub scale: Option<f64>,
}
impl PartialEq for FlightOmitRecord1694725934 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/lensDistortionEffect.ts:3 (sha256:3c272223b98303b3de96ea3f91824da1e932424f3a5eda24672deda3d98e0062)
#[derive(Clone, Default)]
struct CreateLensDistortionEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLensDistortionEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lens_distortion_effect(
    options: Option<FlightOmitRecord1694725934>,
) -> LensDistortionEffect {
    let options = options.unwrap_or(FlightOmitRecord1694725934 {
        __flight_identity: std::sync::Arc::new(()),
        amount: None,
        scale: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        LensDistortionEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "LensDistortionEffect".to_owned(),
            amount: __flight_spread_1.amount,
            scale: __flight_spread_1.scale,
            ..Default::default()
        }
    };
}
