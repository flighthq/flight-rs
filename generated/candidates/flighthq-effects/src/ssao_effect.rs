// @generated from upstream/packages/effects/src/ssaoEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SsaoEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord3526990773 {
    pub __flight_identity: std::sync::Arc<()>,
    pub radius: Option<f64>,
    pub intensity: Option<f64>,
    pub bias: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for FlightOmitRecord3526990773 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/ssaoEffect.ts:3 (sha256:6131e6956dcfe2a0de479b4a724e8a1534ab7c47195249f34f9834599a5b40b6)
#[derive(Clone, Default)]
struct CreateSsaoEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSsaoEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_ssao_effect(options: Option<FlightOmitRecord3526990773>) -> SsaoEffect {
    let options = options.unwrap_or(FlightOmitRecord3526990773 {
        __flight_identity: std::sync::Arc::new(()),
        radius: None,
        intensity: None,
        bias: None,
        samples: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        SsaoEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "SsaoEffect".to_owned(),
            radius: __flight_spread_1.radius,
            intensity: __flight_spread_1.intensity,
            bias: __flight_spread_1.bias,
            samples: __flight_spread_1.samples,
            ..Default::default()
        }
    };
}
