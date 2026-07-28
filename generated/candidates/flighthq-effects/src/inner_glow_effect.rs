// @generated from upstream/packages/effects/src/innerGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{InnerEffectSourceMode, InnerGlowEffect};

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<InnerEffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/innerGlowEffect.ts:4 (sha256:8ffdefb7949478edf386983ea702db37c25697510f18477650ed5fbecc1b3946)
#[derive(Clone)]
struct CreateInnerGlowEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInnerGlowEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_inner_glow_effect(options: Option<FlightOmitRecord1>) -> InnerGlowEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        alpha: None,
        blur_x: None,
        blur_y: None,
        color: None,
        quality: None,
        source_mode: None,
        strength: None,
    });
    return {
        let __flight_spread_1 = options;
        InnerGlowEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "InnerGlowEffect".to_owned(),
            alpha: __flight_spread_1.alpha,
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            color: __flight_spread_1.color,
            quality: __flight_spread_1.quality,
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
        }
    };
}
