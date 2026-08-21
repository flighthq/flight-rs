// @generated from upstream/packages/effects/src/dropShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_directional_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    DropShadowEffect, EffectSourceMode, RenderEffect, RenderEffectPadding, RenderState,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1537644524 {
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
impl PartialEq for FlightOmitRecord1537644524 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/dropShadowEffect.ts:6 (sha256:3ab0759bb38792b708d22eb5f5a8d8352d1d2ba3181a9bd200650c5d0a6f40d9)
#[derive(Clone, Default)]
struct CreateDropShadowEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDropShadowEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_drop_shadow_effect(options: Option<FlightOmitRecord1537644524>) -> DropShadowEffect {
    let options = options.unwrap_or(FlightOmitRecord1537644524 {
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
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/dropShadowEffect.ts:10 (sha256:c9f9ecca08a780d61bd15fe6520ae22032da29f3bf0150caf8e6bc11cbdbee0e)
pub fn get_drop_shadow_effect_padding(effect: &DropShadowEffect) -> RenderEffectPadding {
    let angle = (((effect.angle).clone().unwrap_or(45.0_f64) * std::f64::consts::PI) / 180.0_f64);
    let distance = (effect.distance).clone().unwrap_or(4.0_f64);
    return get_directional_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(4.0_f64),
        (effect.blur_y).clone().unwrap_or(4.0_f64),
        ((angle).cos() * distance),
        ((angle).sin() * distance),
    );
}

// Source: upstream/packages/effects/src/dropShadowEffect.ts:21 (sha256:d3c08dd7b2f27b8287de5cf6b982cbfd03b368b1556d6357089eec518060c8d3)
pub fn register_drop_shadow_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "DropShadowEffect".to_owned(),
        &(resolve_drop_shadow_effect_padding),
    );
}

// Source: upstream/packages/effects/src/dropShadowEffect.ts:25 (sha256:b33e3c4058ffa81bff4e1e12931286fc1c05de941855ea40d501ae5d8ed15ed5)
fn resolve_drop_shadow_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_drop_shadow_effect_padding(&{
        let __flight_source = &((*effect).clone());
        DropShadowEffect {
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
