// @generated from upstream/packages/effects/src/barrelDistortionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BarrelDistortionEffect;

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

// Source: upstream/packages/effects/src/barrelDistortionEffect.ts:3 (sha256:ff0c477d6ebfcc411a27e6bd6821f14499dd408424a1de8104a3d5b67093846e)
#[derive(Clone, Default)]
struct CreateBarrelDistortionEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBarrelDistortionEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_barrel_distortion_effect(
    options: Option<FlightOmitRecord1694725934>,
) -> BarrelDistortionEffect {
    let options = options.unwrap_or(FlightOmitRecord1694725934 {
        __flight_identity: std::sync::Arc::new(()),
        amount: None,
        scale: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        BarrelDistortionEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BarrelDistortionEffect".to_owned(),
            amount: __flight_spread_1.amount,
            scale: __flight_spread_1.scale,
            ..Default::default()
        }
    };
}
