// @generated from upstream/packages/render/src/renderColorTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_node::get_node_runtime;
use flighthq_types::{RenderProxy, RenderState};

// Source: upstream/packages/render/src/renderColorTransform.ts:17 (sha256:32040be95ef34c3cbd1e8a96752fde09a792b64217d3d4b888e293bc3c72b408)
pub fn update_render_proxy_color_transform(
    state: &RenderState,
    data: &mut RenderProxy,
    _parent_data: Option<RenderProxy>,
) -> () {
    let runtime = get_node_runtime(&(data.source).clone());
    data.color_transform = (runtime.resolved_color_transform).clone();
    if runtime.color_adjustments_channel_mixing {
        {
            let __flight_callback =
                (get_render_state_runtime(state).color_adjustment_channel_mixing_guard).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()((*state).clone(), (data.source).clone()))
        };
    }
}
