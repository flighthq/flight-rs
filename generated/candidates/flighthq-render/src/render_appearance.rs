// @generated from upstream/packages/render/src/renderAppearance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_node::get_node_appearance_revision;
use flighthq_types::{RenderProxy, RenderState};

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
#[derive(Clone)]
struct RecalculateAppearanceRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecalculateAppearanceRecord1 {
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
