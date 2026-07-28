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
use flighthq_types::{RenderProxy2D, RenderState};

// Source: upstream/packages/render/src/renderTransform2d.ts:7 (sha256:51400b322d1bac6809bf2dc4dfc3c3410ad78310ca75f1c5a4646a46479aa9e4)
pub fn update_render_proxy2_d_transform(
    state: &RenderState,
    data: &mut RenderProxy2D,
    parent_data: Option<RenderProxy2D>,
) -> bool {
    let local_transform_id = get_node_local_transform_revision(&(data.source).clone());
    let parent_dirty = ((parent_data).is_some()
        && (parent_data.as_ref().unwrap().transform_frame_id
            == get_render_state_runtime(state).current_frame_id));
    let local_dirty = (data.last_local_transform_id != local_transform_id);
    if (parent_dirty || local_dirty) {
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
struct RecalculateRenderTransform2DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecalculateRenderTransform2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recalculate_render_transform2_d(
    state: &RenderState,
    data: &mut RenderProxy2D,
    parent_data: Option<RenderProxy2D>,
) -> () {
    let transform2_d = get_node_local_matrix(&(data.source).clone());
    let parent_transform2_d = if (parent_data).is_some() {
        (parent_data.as_ref().unwrap().transform2_d).clone()
    } else {
        (state.render_transform2_d).clone()
    };
    if (parent_transform2_d).is_some() {
        multiply_matrix(&mut data.transform2_d, &parent_transform2_d, &transform2_d);
    } else {
        copy_matrix(&mut data.transform2_d, &transform2_d);
    }
    data.transform_frame_id = get_render_state_runtime(state).current_frame_id;
}
