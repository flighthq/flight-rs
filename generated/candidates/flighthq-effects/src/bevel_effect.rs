// @generated from upstream/packages/effects/src/bevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_directional_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{
    BevelEffect, BlendMode, EffectSourceMode, Matrix, RenderEffect, RenderEffectPadding,
    RenderState, Scene2DClipHooks, Scene3DGraphSyncPolicy,
};

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

// Source: upstream/packages/effects/src/bevelEffect.ts:6 (sha256:14b78e6e86a452b7dd16a5c6bb5194e2f2d6a40a72cc2350199439881dc149e2)
#[derive(Clone, Default)]
struct CreateBevelEffectRecord11 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBevelEffectRecord11 {
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

// Source: upstream/packages/effects/src/bevelEffect.ts:10 (sha256:21305b8240fd3f33cd4a1c1f81c7c95e7adce82eb301e09ed1476f9ab6df747b)
pub fn get_bevel_effect_padding(effect: &BevelEffect) -> RenderEffectPadding {
    let angle = (((effect.angle).clone().unwrap_or(45.0_f64) * std::f64::consts::PI) / 180.0_f64);
    let distance = (effect.distance).clone().unwrap_or(4.0_f64);
    return get_directional_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(4.0_f64),
        (effect.blur_y).clone().unwrap_or(4.0_f64),
        ((angle).cos() * distance),
        ((angle).sin() * distance),
    );
}

// Source: upstream/packages/effects/src/bevelEffect.ts:21 (sha256:e1c5b0eabe8d8189715c51bb1b7257d7402197786ca53bf0690536a5d59282d8)
pub fn register_bevel_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "BevelEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_bevel_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/bevelEffect.ts:25 (sha256:f41607c336578bfbfbf89f7b93110651e59351bec835b387ede2abf11a90d628)
fn resolve_bevel_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_bevel_effect_padding(&{
        let __flight_source = &((*effect).clone());
        BevelEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            angle: __flight_source.angle,
            bevel_type: (__flight_source.bevel_type).clone(),
            blur_x: __flight_source.blur_x,
            blur_y: __flight_source.blur_y,
            distance: __flight_source.distance,
            highlight_alpha: __flight_source.highlight_alpha,
            highlight_color: __flight_source.highlight_color,
            quality: __flight_source.quality,
            shadow_alpha: __flight_source.shadow_alpha,
            shadow_color: __flight_source.shadow_color,
            source_mode: (__flight_source.source_mode).clone(),
            strength: __flight_source.strength,
            ..Default::default()
        }
    });
}
