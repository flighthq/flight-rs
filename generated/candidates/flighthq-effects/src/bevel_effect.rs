// @generated from upstream/packages/effects/src/bevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BevelEffect, EffectSourceMode};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub angle: Option<f64>,
    pub bevel_type: Option<String>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub distance: Option<f64>,
    pub highlight_alpha: Option<f64>,
    pub highlight_color: Option<f64>,
    pub quality: Option<f64>,
    pub shadow_alpha: Option<f64>,
    pub shadow_color: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/bevelEffect.ts:4 (sha256:14b78e6e86a452b7dd16a5c6bb5194e2f2d6a40a72cc2350199439881dc149e2)
#[derive(Clone, Default)]
struct CreateBevelEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBevelEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_bevel_effect(options: Option<FlightOmitRecord1>) -> BevelEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        angle: None,
        bevel_type: None,
        blur_x: None,
        blur_y: None,
        distance: None,
        highlight_alpha: None,
        highlight_color: None,
        quality: None,
        shadow_alpha: None,
        shadow_color: None,
        source_mode: None,
        strength: None,
    });
    return {
        let __flight_spread_1 = options;
        BevelEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BevelEffect".to_owned(),
            angle: __flight_spread_1.angle,
            bevel_type: (__flight_spread_1.bevel_type).clone(),
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            distance: __flight_spread_1.distance,
            highlight_alpha: __flight_spread_1.highlight_alpha,
            highlight_color: __flight_spread_1.highlight_color,
            quality: __flight_spread_1.quality,
            shadow_alpha: __flight_spread_1.shadow_alpha,
            shadow_color: __flight_spread_1.shadow_color,
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
            ..Default::default()
        }
    };
}
