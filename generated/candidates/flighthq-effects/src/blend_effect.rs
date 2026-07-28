// @generated from upstream/packages/effects/src/blendEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AdvancedBlendMode, BlendEffect};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub backdrop_key: Option<String>,
    pub opacity: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/blendEffect.ts:9 (sha256:757aecefa83975696363ffae04cff0201f7889f8dc06ca59c5d88e81d1e4a7c9)
#[derive(Clone, Default)]
struct CreateBlendEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBlendEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_blend_effect(
    mode: AdvancedBlendMode,
    options: Option<FlightOmitRecord1>,
) -> BlendEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        backdrop_key: None,
        opacity: None,
    });
    return {
        let __flight_spread_2 = options;
        BlendEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BlendEffect".to_owned(),
            mode: (mode).clone(),
            backdrop_key: (__flight_spread_2.backdrop_key).clone(),
            opacity: __flight_spread_2.opacity,
            ..Default::default()
        }
    };
}
