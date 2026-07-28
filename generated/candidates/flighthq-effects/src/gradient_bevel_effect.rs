// @generated from upstream/packages/effects/src/gradientBevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{EffectSourceMode, GradientBevelEffect};

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alphas: Vec<f64>,
    pub angle: Option<f64>,
    pub bevel_type: Option<String>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub colors: Vec<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub ratios: Vec<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/gradientBevelEffect.ts:4 (sha256:4cb3bdc0dcad1e5c16719a827a7acf1156d9de60481d2bf162759e64514596d6)
pub fn create_gradient_bevel_effect(options: &FlightOmitRecord1) -> GradientBevelEffect {
    return {
        let __flight_spread_1 = options;
        GradientBevelEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "GradientBevelEffect".to_owned(),
            alphas: (__flight_spread_1.alphas).clone(),
            angle: __flight_spread_1.angle,
            bevel_type: (__flight_spread_1.bevel_type).clone(),
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            colors: (__flight_spread_1.colors).clone(),
            distance: __flight_spread_1.distance,
            quality: __flight_spread_1.quality,
            ratios: (__flight_spread_1.ratios).clone(),
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
        }
    };
}
