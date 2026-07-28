// @generated from upstream/packages/effects/src/dropShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{DropShadowEffect, EffectSourceMode};

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub angle: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/dropShadowEffect.ts:4 (sha256:3ab0759bb38792b708d22eb5f5a8d8352d1d2ba3181a9bd200650c5d0a6f40d9)
#[derive(Clone)]
struct CreateDropShadowEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDropShadowEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_drop_shadow_effect(options: Option<FlightOmitRecord1>) -> DropShadowEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        alpha: None,
        angle: None,
        blur_x: None,
        blur_y: None,
        color: None,
        distance: None,
        quality: None,
        source_mode: None,
        strength: None,
    });
    return {
        let __flight_spread_1 = options;
        DropShadowEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "DropShadowEffect".to_owned(),
            alpha: __flight_spread_1.alpha,
            angle: __flight_spread_1.angle,
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            color: __flight_spread_1.color,
            distance: __flight_spread_1.distance,
            quality: __flight_spread_1.quality,
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
        }
    };
}
