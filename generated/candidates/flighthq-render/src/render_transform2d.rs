// @generated from upstream/packages/render/src/renderTransform2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_geometry::{copy_matrix, multiply_matrix};
use flighthq_node::{get_node_local_matrix, get_node_local_transform_revision};
use flighthq_types::{
    Adjustment, BlendMode, ColorTransform, DisplayObjectClipHooks, InteractionSignals, Matrix,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, RenderProxy2D, RenderState,
    SceneGraphSyncPolicy, Transform2DNode,
};

#[derive(Clone)]
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

#[derive(Clone)]
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

// Source: upstream/packages/render/src/renderTransform2d.ts:7 (sha256:51400b322d1bac6809bf2dc4dfc3c3410ad78310ca75f1c5a4646a46479aa9e4)
pub fn update_render_proxy2_d_transform(
    state: &RenderState,
    data: &mut RenderProxy2D,
    parent_data: Option<RenderProxy2D>,
) -> bool {
    let local_transform_id = get_node_local_transform_revision(&(data.source).clone());
    let parent_dirty = ((parent_data).is_some())
        && (parent_data.as_ref().unwrap().transform_frame_id
            == get_render_state_runtime(state).current_frame_id);
    let local_dirty = (data.last_local_transform_id != local_transform_id);
    if (parent_dirty) || (local_dirty) {
        recalculate_render_transform2_d(
            state,
            data,
            Some(((parent_data).clone().unwrap()).clone()),
        );
        data.last_local_transform_id = local_transform_id;
        return true;
    }
    return false;
}

// Source: upstream/packages/render/src/renderTransform2d.ts:25 (sha256:6a2c29ffed0ca2316d918a121df2248ae3ba3aa74d183b796ce597c8caecb455)
#[derive(Clone)]
struct RecalculateRenderTransform2DRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecalculateRenderTransform2DRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recalculate_render_transform2_d(
    state: &RenderState,
    data: &mut RenderProxy2D,
    parent_data: Option<RenderProxy2D>,
) -> () {
    let transform2_d = get_node_local_matrix(&Transform2DNode {
        __flight_identity: std::sync::Arc::clone(&((data.source).clone()).__flight_identity),
        data: (((data.source).clone()).data).clone(),
        enabled: ((data.source).clone()).enabled,
        kind: (((data.source).clone()).kind).clone(),
        name: (((data.source).clone()).name).clone(),
        pivot_x: ((data.source).clone()).pivot_x,
        pivot_y: ((data.source).clone()).pivot_y,
        rotation: ((data.source).clone()).rotation,
        scale_x: ((data.source).clone()).scale_x,
        scale_y: ((data.source).clone()).scale_y,
        skew_x: ((data.source).clone()).skew_x,
        skew_y: ((data.source).clone()).skew_y,
        x: ((data.source).clone()).x,
        y: ((data.source).clone()).y,
    });
    let parent_transform2_d = if (parent_data).is_some() {
        (parent_data.as_ref().unwrap().transform2_d).clone()
    } else {
        ((state.render_transform2_d).clone()).unwrap()
    };
    if (parent_transform2_d).is_some() {
        multiply_matrix(&mut data.transform2_d, &parent_transform2_d, &transform2_d);
    } else {
        copy_matrix(&mut data.transform2_d, &transform2_d);
    }
    data.transform_frame_id = get_render_state_runtime(state).current_frame_id;
}
