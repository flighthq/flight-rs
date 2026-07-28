// @generated from upstream/packages/render/src/renderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{create_entity, create_entity_runtime};
use flighthq_types::{BLEND_MODE as blend_mode_constant, RenderState, RenderStateRuntime};

// Source: upstream/packages/render/src/renderState.ts:5 (sha256:77d919408262f698375721533f66dec71fec3d5cd6234cfbb80066475558968e)
pub fn create_render_state(obj: Option<RenderState>) -> RenderState {
    let mut state = create_entity(Some(RenderState {
        __flight_identity: std::sync::Arc::new(()),
        allow_smoothing: (obj.as_ref().map(|value| value.allow_smoothing)).unwrap_or(true),
        background_color: (obj.as_ref().map(|value| value.background_color)).unwrap_or(0.0_f64),
        background_color_rgba: (obj
            .as_ref()
            .map(|value| (value.background_color_rgba).clone()))
        .unwrap_or(vec![]),
        background_color_string: (obj
            .as_ref()
            .map(|value| (value.background_color_string).clone()))
        .unwrap_or("".to_owned()),
        current_clip_depth: (obj.as_ref().map(|value| value.current_clip_depth)).unwrap_or(0.0_f64),
        display_object_clip_hooks: obj
            .as_ref()
            .and_then(|value| (value.display_object_clip_hooks).clone()),
        pixel_ratio: (obj.as_ref().map(|value| value.pixel_ratio)).unwrap_or(1.0_f64),
        render_alpha: (obj.as_ref().map(|value| value.render_alpha)).unwrap_or(1.0_f64),
        render_blend_mode: Some(
            (obj.as_ref()
                .and_then(|value| (value.render_blend_mode).clone()))
            .unwrap_or((blend_mode_constant.normal).clone()),
        ),
        render_transform2_d: obj
            .as_ref()
            .and_then(|value| (value.render_transform2_d).clone()),
        round_pixels: (obj.as_ref().map(|value| value.round_pixels)).unwrap_or(false),
        scene_graph_sync_policy: (obj
            .as_ref()
            .map(|value| (value.scene_graph_sync_policy).clone()))
        .unwrap_or("refreshDerivedState".to_owned()),
    }));
    ();
    return state;
}

// Source: upstream/packages/render/src/renderState.ts:29 (sha256:d19fed0d3e8e0d8208b2d8c78b34375c1cecd269b1b4e893f72ea7350c9f85d9)
pub fn create_render_state_runtime() -> RenderStateRuntime {
    let mut runtime = create_entity_runtime();
    runtime.color_adjustment_channel_mixing_guard = None;
    runtime.current_frame_id = 0.0_f64;
    runtime.render_adapt_hook = None;
    runtime.render_proxy_adapter_map = Vec::new();
    runtime.render_proxy_map = Vec::new();
    runtime.renderer_map = Vec::new();
    runtime.renderer_map_id = 0.0_f64;
    runtime.temp_stack = vec![];
    return runtime;
}

// Source: upstream/packages/render/src/renderState.ts:44 (sha256:fe128b76ac43af999788bb3f3c74ba5dab813e0ad282d0c8458e5d1570dde3fb)
pub fn get_render_state_runtime(state: &RenderState) -> RenderStateRuntime {
    return panic!("entity runtime storage requires the generated native entity trait");
}
