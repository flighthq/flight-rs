// @generated from upstream/packages/effects/src/outerGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_gaussian_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    EffectSourceMode, OuterGlowEffect, RenderEffect, RenderEffectPadding, RenderState,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord3817894418 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord3817894418 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/outerGlowEffect.ts:6 (sha256:9e9b60be5f480a66fef755ca2d03553cc8be4a2e4ef7b2029ffd4663a8a3762c)
#[derive(Clone, Default)]
struct CreateOuterGlowEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateOuterGlowEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_outer_glow_effect(options: Option<FlightOmitRecord3817894418>) -> OuterGlowEffect {
    let options = options.unwrap_or(FlightOmitRecord3817894418 {
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
        let __flight_spread_1 = (options).clone();
        OuterGlowEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "OuterGlowEffect".to_owned(),
            alpha: __flight_spread_1.alpha,
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            color: __flight_spread_1.color,
            quality: __flight_spread_1.quality,
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/outerGlowEffect.ts:10 (sha256:666e7a6e59cf57611423b7d1fabc3c8dfc47eabc81de67640bebcbafa6298031)
pub fn get_outer_glow_effect_padding(effect: &OuterGlowEffect) -> RenderEffectPadding {
    return get_gaussian_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(6.0_f64),
        (effect.blur_y).clone().unwrap_or(6.0_f64),
    );
}

// Source: upstream/packages/effects/src/outerGlowEffect.ts:14 (sha256:72a62c74380fa69e5c61678f3084fb30189bfeb283c9a3fa8b5d15a26e131e4f)
pub fn register_outer_glow_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "OuterGlowEffect".to_owned(),
        &(resolve_outer_glow_effect_padding),
    );
}

// Source: upstream/packages/effects/src/outerGlowEffect.ts:18 (sha256:a4e23461d65241f6bb94fe3ab969b7ea89a7434db36ea3b1a46f8786eca77dfb)
fn resolve_outer_glow_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_outer_glow_effect_padding(&{
        let __flight_source = &((*effect).clone());
        OuterGlowEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            alpha: __flight_source.alpha,
            blur_x: __flight_source.blur_x,
            blur_y: __flight_source.blur_y,
            color: __flight_source.color,
            quality: __flight_source.quality,
            source_mode: (__flight_source.source_mode).clone(),
            strength: __flight_source.strength,
            ..Default::default()
        }
    });
}
