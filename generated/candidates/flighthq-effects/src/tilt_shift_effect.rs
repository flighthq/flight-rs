// @generated from upstream/packages/effects/src/tiltShiftEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{
    BlendMode, Matrix, RenderEffect, RenderEffectPadding, RenderState, Scene2DClipHooks,
    Scene3DGraphSyncPolicy, TiltShiftEffect,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub center: Option<f64>,
    pub width: Option<f64>,
    pub blur: Option<f64>,
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

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:5 (sha256:2ff62cd921c5f37d54bffaed6b7d2015ef8107f8a5f8ca2c084ded52800c023d)
#[derive(Clone, Default)]
struct CreateTiltShiftEffectRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTiltShiftEffectRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tilt_shift_effect(options: Option<FlightOmitRecord1>) -> TiltShiftEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        center: None,
        width: None,
        blur: None,
    });
    return {
        let __flight_spread_1 = options;
        TiltShiftEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "TiltShiftEffect".to_owned(),
            center: __flight_spread_1.center,
            width: __flight_spread_1.width,
            blur: __flight_spread_1.blur,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:9 (sha256:2a5b7bc34d13ede313ead5958317ed6e3422f59d7d282ef54a27b69d2f824fdb)
pub fn get_tilt_shift_effect_padding(effect: &TiltShiftEffect) -> RenderEffectPadding {
    let vertical = ((0.0_f64).max((effect.blur).clone().unwrap_or(4.0_f64)) * 3.0_f64).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: vertical,
        left: 0.0_f64,
        right: 0.0_f64,
        top: vertical,
    };
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:14 (sha256:552bbe3b57d51af21eb3e3fcb4d00062cf82d7f28f7add599504e8cccaa6dec2)
pub fn register_tilt_shift_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "TiltShiftEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_tilt_shift_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:18 (sha256:31c1ac3b51e158d38187dfdfcf544b136f88c14e3e561b4967a795a89786ac09)
fn resolve_tilt_shift_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_tilt_shift_effect_padding(&{
        let __flight_source = &((*effect).clone());
        TiltShiftEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            center: __flight_source.center,
            width: __flight_source.width,
            blur: __flight_source.blur,
            ..Default::default()
        }
    });
}
