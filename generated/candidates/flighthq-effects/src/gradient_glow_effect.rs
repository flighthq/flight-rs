// @generated from upstream/packages/effects/src/gradientGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_gaussian_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    EffectSourceMode, GradientGlowEffect, RenderEffect, RenderEffectPadding, RenderState,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2348048241 {
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
impl PartialEq for FlightOmitRecord2348048241 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/gradientGlowEffect.ts:6 (sha256:156011c5d67f1c2c702b7a6a7438ffb0bcb8c11b33c0424e0d96193ce6833158)
pub fn create_gradient_glow_effect(options: &FlightOmitRecord2348048241) -> GradientGlowEffect {
    return {
        let __flight_spread_1 = (*options).clone();
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

// Source: upstream/packages/effects/src/gradientGlowEffect.ts:10 (sha256:dfb433f5cabfccd588c04da5b3cba2b9413601b34e39d5ca866241a9a4692dd1)
pub fn get_gradient_glow_effect_padding(effect: &GradientGlowEffect) -> RenderEffectPadding {
    return get_gaussian_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(6.0_f64),
        (effect.blur_y).clone().unwrap_or(6.0_f64),
    );
}

// Source: upstream/packages/effects/src/gradientGlowEffect.ts:14 (sha256:9e19515807271cea73e96d1dc084b7d884e4ef176f3618693af12bd9c2083560)
pub fn register_gradient_glow_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "GradientGlowEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_gradient_glow_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/gradientGlowEffect.ts:18 (sha256:98be64089837cdf7820dcbfb95b737ab82a4fb2e46a3140b16ba0f56b0e76a39)
fn resolve_gradient_glow_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_gradient_glow_effect_padding(&{
        let __flight_source = &((*effect).clone());
        GradientGlowEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            alphas: (__flight_source.alphas).clone(),
            blur_x: __flight_source.blur_x,
            blur_y: __flight_source.blur_y,
            colors: (__flight_source.colors).clone(),
            quality: __flight_source.quality,
            ratios: (__flight_source.ratios).clone(),
            source_mode: (__flight_source.source_mode).clone(),
            strength: __flight_source.strength,
            ..Default::default()
        }
    });
}
