// @generated from upstream/packages/effects/src/ssrEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SsrEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord3905770745 {
    pub __flight_identity: std::sync::Arc<()>,
    pub max_distance: Option<f64>,
    pub resolution: Option<f64>,
    pub steps: Option<f64>,
}
impl PartialEq for FlightOmitRecord3905770745 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/ssrEffect.ts:3 (sha256:30717c7fd0026c3b42818f92458ad6af6bfa29454faf42e1fced442ccd6580f6)
#[derive(Clone, Default)]
struct CreateSsrEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSsrEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_ssr_effect(options: Option<FlightOmitRecord3905770745>) -> SsrEffect {
    let options = options.unwrap_or(FlightOmitRecord3905770745 {
        __flight_identity: std::sync::Arc::new(()),
        max_distance: None,
        resolution: None,
        steps: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        SsrEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "SsrEffect".to_owned(),
            max_distance: __flight_spread_1.max_distance,
            resolution: __flight_spread_1.resolution,
            steps: __flight_spread_1.steps,
            ..Default::default()
        }
    };
}
