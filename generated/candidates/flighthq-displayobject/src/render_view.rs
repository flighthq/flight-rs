// @generated from upstream/packages/displayobject/src/renderView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_node::invalidate_node_local_bounds;
use flighthq_types::{
    Node, RENDER_VIEW_KIND as render_view_kind_constant, Rectangle, RenderView, RenderViewData,
    RenderViewRuntime,
};

// Source: upstream/packages/displayobject/src/renderView.ts:15 (sha256:c339ddd5fd94fc9e8538242e15b5e021059d970dbf188729c7b2fc964087f7a9)
pub fn compute_render_view_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    out.width = source.data.width;
    out.height = source.data.height;
}

// Source: upstream/packages/displayobject/src/renderView.ts:21 (sha256:439f741ca982f24a8900c0b43c151482b2636b2b51468b5f99199251bbdd1cbe)
pub fn create_render_view(obj: Option<RenderView>) -> RenderView {
    return create_display_object_generic(
        render_view_kind_constant,
        Some(((obj).clone().unwrap()).clone()),
        Some(create_render_view_data),
        Some(create_render_view_runtime),
    );
}

// Source: upstream/packages/displayobject/src/renderView.ts:25 (sha256:5f7ac885df1ed5daad6174a3713a5fa878ed3790600ff4033504d7997968aba9)
pub fn create_render_view_data(data: Option<RenderViewData>) -> RenderViewData {
    return RenderViewData {
        __flight_identity: std::sync::Arc::new(()),
        height: (data.as_ref().map(|value| value.height)).unwrap_or(0.0_f64),
        renderer: data.as_ref().and_then(|value| (value.renderer).clone()),
        width: (data.as_ref().map(|value| value.width)).unwrap_or(0.0_f64),
    };
}

// Source: upstream/packages/displayobject/src/renderView.ts:33 (sha256:4b5509403b0e261699c920c20f1915036f8eaedc1cdb3832d61bc41ca3297895)
pub fn create_render_view_runtime() -> RenderViewRuntime {
    return create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
}

// Source: upstream/packages/displayobject/src/renderView.ts:37 (sha256:57f6a6c0549d601eb1496de07cff49430570fb9c027b83dc7ad320e40a9ead33)
pub fn get_render_view_runtime(source: &RenderView) -> RenderViewRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/displayobject/src/renderView.ts:41 (sha256:da0463a37167f4971b6f42ac6beec2a90a5987d89e6528065634f5df16a4f489)
pub fn set_render_view_size(source: &mut RenderView, width: f64, height: f64) -> () {
    if ((source.data.width == width) && (source.data.height == height)) {
        return;
    }
    source.data.width = width;
    source.data.height = height;
    invalidate_node_local_bounds(source);
}

// Source: upstream/packages/displayobject/src/renderView.ts:48 (sha256:e0331c119ced96488bcb2ab50d6f959807ca212fef8a52887acc649835a3e6a6)
static DEFAULT_METHODS: std::sync::LazyLock<RenderViewRuntime> =
    std::sync::LazyLock::new(|| RenderViewRuntime {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: compute_render_view_local_bounds_rectangle,
    });
