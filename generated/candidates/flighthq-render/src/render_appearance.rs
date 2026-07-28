// @generated from upstream/packages/render/src/renderAppearance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_node::get_node_appearance_revision;
use flighthq_types::{
    Adjustment, BlendMode, ColorTransform, DisplayObjectClipHooks, InteractionSignals, Matrix,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, RenderProxy, RenderState,
    SceneGraphSyncPolicy,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
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
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/renderAppearance.ts:6 (sha256:8e81d74b857fdf2dae44389936e1469ade3a88796a1a82af576b14f455af68b7)
pub fn update_render_proxy_appearance(
    state: &RenderState,
    data: &mut RenderProxy,
    parent_data: Option<RenderProxy>,
) -> bool {
    let appearance_id = get_node_appearance_revision(&(data.source).clone());
    if (((parent_data).is_some())
        && (parent_data.as_ref().unwrap().appearance_frame_id
            == get_render_state_runtime(state).current_frame_id))
        || (data.last_appearance_id != appearance_id)
    {
        recalculate_appearance(state, data, Some(((parent_data).clone().unwrap()).clone()));
        data.last_appearance_id = appearance_id;
        return true;
    }
    return false;
}

// Source: upstream/packages/render/src/renderAppearance.ts:19 (sha256:41880c43d52808bd3cb08a187578239dd58381cce79622f6e310bb3441fd1209)
#[derive(Clone, Default)]
struct RecalculateAppearanceRecord3 {
    __flight_identity: std::sync::Arc<()>,
    alpha: f64,
    visible: bool,
    blend_mode: Option<BlendMode>,
}
impl PartialEq for RecalculateAppearanceRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recalculate_appearance(
    state: &RenderState,
    data: &mut RenderProxy,
    parent_data: Option<RenderProxy>,
) -> () {
    let source = (data.source).clone();
    if (parent_data).is_some() {
        data.visible = (source.visible) && (parent_data.as_ref().unwrap().visible);
        if (!data.visible) {
            return;
        }
        data.alpha = (source.alpha * parent_data.as_ref().unwrap().alpha);
        if (data.alpha <= 0.0_f64) {
            return;
        }
        data.blend_mode = (source.blend_mode).clone();
    } else {
        data.visible = source.visible;
        if (!data.visible) {
            return;
        }
        data.alpha = (source.alpha * state.render_alpha);
        if (data.alpha <= 0.0_f64) {
            return;
        }
        data.blend_mode = if ((state.render_blend_mode).clone()).is_some() {
            (state.render_blend_mode).clone()
        } else {
            (source.blend_mode).clone()
        };
    }
    data.appearance_frame_id = get_render_state_runtime(state).current_frame_id;
}
