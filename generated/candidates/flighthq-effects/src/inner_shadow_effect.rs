// @generated from upstream/packages/effects/src/innerShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_directional_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    InnerEffectSourceMode, InnerShadowEffect, RenderEffect, RenderEffectPadding, RenderState,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord348204748 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub angle: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<InnerEffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord348204748 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/innerShadowEffect.ts:6 (sha256:0fb9b1b51145317d567bd88695272e999cc3a21df22ed8d3c1868eb3615d8fb7)
#[derive(Clone, Default)]
struct CreateInnerShadowEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInnerShadowEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_inner_shadow_effect(options: Option<FlightOmitRecord348204748>) -> InnerShadowEffect {
    let options = options.unwrap_or(FlightOmitRecord348204748 {
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
        let __flight_spread_1 = (options).clone();
        InnerShadowEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "InnerShadowEffect".to_owned(),
            alpha: __flight_spread_1.alpha,
            angle: __flight_spread_1.angle,
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            color: __flight_spread_1.color,
            distance: __flight_spread_1.distance,
            quality: __flight_spread_1.quality,
            source_mode: (__flight_spread_1.source_mode).clone(),
            strength: __flight_spread_1.strength,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/innerShadowEffect.ts:10 (sha256:89b79f4fe4fbd9a05e55c7600e89f05a04724141de9ad77b47ee4f95f3e27eaf)
pub fn get_inner_shadow_effect_padding(effect: &InnerShadowEffect) -> RenderEffectPadding {
    let angle = (((effect.angle).unwrap_or(45.0_f64) * std::f64::consts::PI) / 180.0_f64);
    let distance = (effect.distance).unwrap_or(4.0_f64);
    return get_directional_render_effect_padding(
        (effect.blur_x).unwrap_or(4.0_f64),
        (effect.blur_y).unwrap_or(4.0_f64),
        ((angle).cos() * distance),
        ((angle).sin() * distance),
    );
}

// Source: upstream/packages/effects/src/innerShadowEffect.ts:21 (sha256:05133f50da5ca4066933d5d5aaf817e04dcca20e7051048c5ae579f0cb1b12a1)
pub fn register_inner_shadow_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "InnerShadowEffect".to_owned(),
        &(resolve_inner_shadow_effect_padding),
    );
}

// Source: upstream/packages/effects/src/innerShadowEffect.ts:25 (sha256:460cc22fc6ae25764eb24f871c9cb0d0e85abcdf81d813b4b59f9612e6c09a9e)
fn resolve_inner_shadow_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_inner_shadow_effect_padding(&{
        let __flight_source = &((*effect).clone());
        InnerShadowEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            alpha: __flight_source.alpha,
            angle: __flight_source.angle,
            blur_x: __flight_source.blur_x,
            blur_y: __flight_source.blur_y,
            color: __flight_source.color,
            distance: __flight_source.distance,
            quality: __flight_source.quality,
            source_mode: (__flight_source.source_mode).clone(),
            strength: __flight_source.strength,
            ..Default::default()
        }
    });
}
