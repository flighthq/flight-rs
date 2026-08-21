// @generated from upstream/packages/effects/src/gradientBevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_directional_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    EffectSourceMode, GradientBevelEffect, RenderEffect, RenderEffectPadding, RenderState,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1682838785 {
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
impl PartialEq for FlightOmitRecord1682838785 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/gradientBevelEffect.ts:6 (sha256:4cb3bdc0dcad1e5c16719a827a7acf1156d9de60481d2bf162759e64514596d6)
pub fn create_gradient_bevel_effect(options: &FlightOmitRecord1682838785) -> GradientBevelEffect {
    return {
        let __flight_spread_1 = (*options).clone();
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
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/gradientBevelEffect.ts:10 (sha256:fea1ca0f12e8320993a77d4c55ead18dddd15cce04058ac0d513e5ac51efb501)
pub fn get_gradient_bevel_effect_padding(effect: &GradientBevelEffect) -> RenderEffectPadding {
    let angle = (((effect.angle).clone().unwrap_or(45.0_f64) * std::f64::consts::PI) / 180.0_f64);
    let distance = (effect.distance).clone().unwrap_or(4.0_f64);
    return get_directional_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(4.0_f64),
        (effect.blur_y).clone().unwrap_or(4.0_f64),
        ((angle).cos() * distance),
        ((angle).sin() * distance),
    );
}

// Source: upstream/packages/effects/src/gradientBevelEffect.ts:21 (sha256:0a4dcba0559e8d83a9c948b2be2e3f7c7d106eb5d5454bfd68e281a65319415c)
pub fn register_gradient_bevel_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "GradientBevelEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_gradient_bevel_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/gradientBevelEffect.ts:25 (sha256:9b07cb1a4bc2552f69f592db88d58b84171a64d12cf7811039382e06d7fe2959)
fn resolve_gradient_bevel_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_gradient_bevel_effect_padding(&{
        let __flight_source = &((*effect).clone());
        GradientBevelEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            alphas: (__flight_source.alphas).clone(),
            angle: __flight_source.angle,
            bevel_type: (__flight_source.bevel_type).clone(),
            blur_x: __flight_source.blur_x,
            blur_y: __flight_source.blur_y,
            colors: (__flight_source.colors).clone(),
            distance: __flight_source.distance,
            quality: __flight_source.quality,
            ratios: (__flight_source.ratios).clone(),
            source_mode: (__flight_source.source_mode).clone(),
            strength: __flight_source.strength,
            ..Default::default()
        }
    });
}
