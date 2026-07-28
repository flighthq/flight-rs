// @generated from upstream/packages/effects/src/gradientGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{EffectSourceMode, GradientGlowEffect};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alphas: Vec<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub colors: Vec<f64>,
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

// Source: upstream/packages/effects/src/gradientGlowEffect.ts:4 (sha256:156011c5d67f1c2c702b7a6a7438ffb0bcb8c11b33c0424e0d96193ce6833158)
pub fn create_gradient_glow_effect(options: &FlightOmitRecord1) -> GradientGlowEffect {
    return {
        let __flight_spread_1 = options;
        GradientGlowEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "GradientGlowEffect".to_owned(),
            alphas: (__flight_spread_1.alphas).clone(),
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            colors: (__flight_spread_1.colors).clone(),
            quality: __flight_spread_1.quality,
            ratios: (__flight_spread_1.ratios).clone(),
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
            ..Default::default()
        }
    };
}
