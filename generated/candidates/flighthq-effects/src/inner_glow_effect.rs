// @generated from upstream/packages/effects/src/innerGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_gaussian_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    BlendMode, InnerEffectSourceMode, InnerGlowEffect, Matrix, RenderEffect, RenderEffectPadding,
    RenderState, Scene2DClipHooks, Scene3DGraphSyncPolicy,
};

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/innerGlowEffect.ts:6 (sha256:8ffdefb7949478edf386983ea702db37c25697510f18477650ed5fbecc1b3946)
#[derive(Clone, Default)]
struct CreateInnerGlowEffectRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInnerGlowEffectRecord3 {
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
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/innerGlowEffect.ts:10 (sha256:7f8cf5dc0d350d4e7002c9664e333ac8343e873609d1d078141249b511431ed6)
pub fn get_inner_glow_effect_padding(effect: &InnerGlowEffect) -> RenderEffectPadding {
    return get_gaussian_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(6.0_f64),
        (effect.blur_y).clone().unwrap_or(6.0_f64),
    );
}

// Source: upstream/packages/effects/src/innerGlowEffect.ts:14 (sha256:e619d80f70b94bae2fcba9cf887a3e96a9de4c3772637f03e63cedbdbea47efd)
pub fn register_inner_glow_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "InnerGlowEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_inner_glow_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/innerGlowEffect.ts:18 (sha256:3cb2c0cb30167c4f0e4e09983b72742113dc4edce4869e260dcece19da23cfc3)
fn resolve_inner_glow_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_inner_glow_effect_padding(&{
        let __flight_source = &((*effect).clone());
        InnerGlowEffect {
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
