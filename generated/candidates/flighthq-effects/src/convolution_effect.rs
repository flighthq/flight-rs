// @generated from upstream/packages/effects/src/convolutionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{
    BlendMode, ConvolutionEffect, Matrix, RenderEffect, RenderEffectPadding, RenderState,
    Scene2DClipHooks, Scene3DGraphSyncPolicy,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub matrix: Vec<f64>,
    pub matrix_x: f64,
    pub matrix_y: f64,
    pub bias: Option<f64>,
    pub clamp: Option<bool>,
    pub color: Option<f64>,
    pub divisor: Option<f64>,
    pub preserve_alpha: Option<bool>,
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

// Source: upstream/packages/effects/src/convolutionEffect.ts:5 (sha256:da09ff4b411a344966534cbd0fa54eea272b20411116ef4f0c27b20f6052566c)
pub fn create_convolution_effect(options: &FlightOmitRecord1) -> ConvolutionEffect {
    return {
        let __flight_spread_1 = options;
        ConvolutionEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ConvolutionEffect".to_owned(),
            matrix: (__flight_spread_1.matrix).clone(),
            matrix_x: __flight_spread_1.matrix_x,
            matrix_y: __flight_spread_1.matrix_y,
            bias: __flight_spread_1.bias,
            clamp: __flight_spread_1.clamp,
            color: __flight_spread_1.color,
            divisor: __flight_spread_1.divisor,
            preserve_alpha: __flight_spread_1.preserve_alpha,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/convolutionEffect.ts:9 (sha256:06469b1c3aee7a6852a88b1186b3d12af894d75830bc9fa847106077c11dee50)
pub fn get_convolution_effect_padding(effect: &ConvolutionEffect) -> RenderEffectPadding {
    let offset_x = ((0.0_f64).max(effect.matrix_x) * 0.5_f64).floor();
    let offset_y = ((0.0_f64).max(effect.matrix_y) * 0.5_f64).floor();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: offset_y,
        left: (0.0_f64).max(((effect.matrix_x - 1.0_f64) - offset_x)),
        right: offset_x,
        top: (0.0_f64).max(((effect.matrix_y - 1.0_f64) - offset_y)),
    };
}

// Source: upstream/packages/effects/src/convolutionEffect.ts:20 (sha256:96019c15994ab1440df382f516ee4c63578916d7d4c37aee1e98af3cd75394ab)
pub fn register_convolution_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "ConvolutionEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_convolution_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/convolutionEffect.ts:24 (sha256:8287b769ab276dcdc6f55669e07372a743f299f5dcdda38729f7e101b81b3bd3)
fn resolve_convolution_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_convolution_effect_padding(&{
        let __flight_source = &((*effect).clone());
        ConvolutionEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            matrix: (__flight_source.matrix).clone(),
            matrix_x: __flight_source.matrix_x,
            matrix_y: __flight_source.matrix_y,
            bias: __flight_source.bias,
            clamp: __flight_source.clamp,
            color: __flight_source.color,
            divisor: __flight_source.divisor,
            preserve_alpha: __flight_source.preserve_alpha,
            ..Default::default()
        }
    });
}
