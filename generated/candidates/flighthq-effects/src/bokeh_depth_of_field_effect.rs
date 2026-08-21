// @generated from upstream/packages/effects/src/bokehDepthOfFieldEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{BokehDepthOfFieldEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord3186256068 {
    pub __flight_identity: std::sync::Arc<()>,
    pub focus_distance: Option<f64>,
    pub focus_range: Option<f64>,
    pub max_blur: Option<f64>,
}
impl PartialEq for FlightOmitRecord3186256068 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/bokehDepthOfFieldEffect.ts:5 (sha256:0b78c1e4a0b23c1ecf901b3287c73a2234ea6f1d8c6d93ce36bd68c17f59d5c3)
#[derive(Clone, Default)]
struct CreateBokehDepthOfFieldEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBokehDepthOfFieldEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_bokeh_depth_of_field_effect(
    options: Option<FlightOmitRecord3186256068>,
) -> BokehDepthOfFieldEffect {
    let options = options.unwrap_or(FlightOmitRecord3186256068 {
        __flight_identity: std::sync::Arc::new(()),
        focus_distance: None,
        focus_range: None,
        max_blur: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        BokehDepthOfFieldEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BokehDepthOfFieldEffect".to_owned(),
            focus_distance: __flight_spread_1.focus_distance,
            focus_range: __flight_spread_1.focus_range,
            max_blur: __flight_spread_1.max_blur,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/bokehDepthOfFieldEffect.ts:11 (sha256:26d21b6b2f2bcf091d3650ce48dc6a1204033523eb7821562f4a2028c74eed2a)
pub fn get_bokeh_depth_of_field_effect_padding(
    effect: &BokehDepthOfFieldEffect,
) -> RenderEffectPadding {
    let radius = ((0.0_f64).max((effect.max_blur).clone().unwrap_or(4.0_f64))).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: radius,
        left: radius,
        right: radius,
        top: radius,
    };
}

// Source: upstream/packages/effects/src/bokehDepthOfFieldEffect.ts:16 (sha256:4167049d0739027950be87f34bafa92f788c2e5d716a80d02f985890c17d2a1c)
pub fn register_bokeh_depth_of_field_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "BokehDepthOfFieldEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_bokeh_depth_of_field_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/bokehDepthOfFieldEffect.ts:20 (sha256:86858b9caca52de548cb4fd98cc39e311bae2f0da0b21c6fbcf42428596d5744)
fn resolve_bokeh_depth_of_field_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_bokeh_depth_of_field_effect_padding(&{
        let __flight_source = &((*effect).clone());
        BokehDepthOfFieldEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            focus_distance: __flight_source.focus_distance,
            focus_range: __flight_source.focus_range,
            max_blur: __flight_source.max_blur,
            ..Default::default()
        }
    });
}
