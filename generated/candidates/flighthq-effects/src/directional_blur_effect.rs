// @generated from upstream/packages/effects/src/directionalBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{
    BlendMode, DirectionalBlurEffect, Matrix, RenderEffect, RenderEffectPadding, RenderState,
    Scene2DClipHooks, Scene3DGraphSyncPolicy,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub angle: Option<f64>,
    pub length: Option<f64>,
    pub samples: Option<f64>,
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

// Source: upstream/packages/effects/src/directionalBlurEffect.ts:5 (sha256:401d47059c3e52d92662d0ebb64d0ca602ef23544159ab858bd9ef7d119d512c)
#[derive(Clone, Default)]
struct CreateDirectionalBlurEffectRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDirectionalBlurEffectRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_directional_blur_effect(options: Option<FlightOmitRecord1>) -> DirectionalBlurEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        angle: None,
        length: None,
        samples: None,
    });
    return {
        let __flight_spread_1 = options;
        DirectionalBlurEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "DirectionalBlurEffect".to_owned(),
            angle: __flight_spread_1.angle,
            length: __flight_spread_1.length,
            samples: __flight_spread_1.samples,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/directionalBlurEffect.ts:11 (sha256:93986a0c9fb77cd943bba0b78df84813f795b364804870a76b0f716eba645046)
pub fn get_directional_blur_effect_padding(effect: &DirectionalBlurEffect) -> RenderEffectPadding {
    let angle = (((effect.angle).clone().unwrap_or(0.0_f64) * std::f64::consts::PI) / 180.0_f64);
    let half_length = ((0.0_f64).max((effect.length).clone().unwrap_or(8.0_f64)) * 0.5_f64);
    let projected_x = ((angle).cos() * half_length).abs();
    let projected_y = ((angle).sin() * half_length).abs();
    let horizontal = if (projected_x < 1e-10_f64) {
        0.0_f64
    } else {
        (projected_x).ceil()
    };
    let vertical = if (projected_y < 1e-10_f64) {
        0.0_f64
    } else {
        (projected_y).ceil()
    };
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: vertical,
        left: horizontal,
        right: horizontal,
        top: vertical,
    };
}

// Source: upstream/packages/effects/src/directionalBlurEffect.ts:24 (sha256:eabab599b02b269ae1bbf7674e94ba88262deb9f7ef034d3b25d09314ee02e20)
pub fn register_directional_blur_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "DirectionalBlurEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_directional_blur_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/directionalBlurEffect.ts:28 (sha256:503ab9621664634d8d734b5696249025d8913ea965108b9563f1fafb5c527c38)
fn resolve_directional_blur_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_directional_blur_effect_padding(&{
        let __flight_source = &((*effect).clone());
        DirectionalBlurEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            angle: __flight_source.angle,
            length: __flight_source.length,
            samples: __flight_source.samples,
            ..Default::default()
        }
    });
}
