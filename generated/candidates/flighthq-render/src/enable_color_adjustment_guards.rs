// @generated from upstream/packages/render/src/enableColorAdjustmentGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_log::log_once;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, LogData, LogDataProvider, LogLevel, Matrix, RenderState,
    Renderable, SceneGraphSyncPolicy,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/enableColorAdjustmentGuards.ts:8 (sha256:28ef4df81b7a23da7419762fa45a0a7357a25688f3618e2d9e0a8a96c1d049d3)
pub fn are_color_adjustment_guards_enabled(state: &RenderState) -> bool {
    return ((get_render_state_runtime(state).color_adjustment_channel_mixing_guard).clone())
        .is_some();
}

// Source: upstream/packages/render/src/enableColorAdjustmentGuards.ts:18 (sha256:f7e544072ba16489ffcd62c4095500229b4627339a00497df46fbb322123ac26)
pub fn enable_color_adjustment_guards(state: &RenderState) -> () {
    get_render_state_runtime(state).color_adjustment_channel_mixing_guard = Some(
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderState, __flight_argument_1: Renderable| -> () {
                warn_color_adjustment_channel_mixing_not_inlineable()
            },
        )
            as Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/render/src/enableColorAdjustmentGuards.ts:22 (sha256:91cdc82c242c7a7151c3d9427905893a9543c29b3a2d7bd5253b568f3c63c881)
#[derive(Clone, Default)]
struct WarnColorAdjustmentChannelMixingNotInlineableRecord2 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnColorAdjustmentChannelMixingNotInlineableRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_color_adjustment_channel_mixing_not_inlineable() -> () {
    log_once(
        "render:color-adjustment-channel-mixing-not-inlineable".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::OpaqueHostValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), crate::OpaqueHostValue::String("updateRenderProxyColorTransform: per-object channel-mixing color adjustment (saturation/hue/sepia/channelMixer) is not inline-able yet — the 4×5 fold is deferred, so only the affine part of the stack was applied. Use an Effect pass for the channel-mixing op.".to_owned())));
            __flight_record
        }))),
        Some(("render".to_owned()).clone()),
    );
}
