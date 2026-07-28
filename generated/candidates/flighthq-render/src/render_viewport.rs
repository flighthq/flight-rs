// @generated from upstream/packages/render/src/renderViewport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_rectangle, matrix_transform_rectangle};
use flighthq_node::get_node_world_bounds_rectangle;
use flighthq_types::{Matrix, Rectangle, RenderProxy2D, RenderViewport2D};

// Source: upstream/packages/render/src/renderViewport.ts:8 (sha256:42679f7ed8e98a08a69ba7734455bff95406edd81a9bdad4d60d84a5974eadc8)
pub fn compute_render_proxy_world_bounds(
    out: &mut Rectangle,
    source: crate::OpaqueHostValue,
) -> bool {
    if (!is_spatial2_d_node((source).clone())) {
        return false;
    }
    let world_bounds = get_node_world_bounds_rectangle(&source);
    out.x = world_bounds.x;
    out.y = world_bounds.y;
    out.width = world_bounds.width;
    out.height = world_bounds.height;
    return true;
}

// Source: upstream/packages/render/src/renderViewport.ts:22 (sha256:4b2af2e5ad3e3b4c44426dde010fa5cc371aefbc0fad4fc2a84c0badce871bd1)
pub fn create_render_viewport2_d(x: f64, y: f64, width: f64, height: f64) -> RenderViewport2D {
    return RenderViewport2D {
        __flight_identity: std::sync::Arc::new(()),
        height: height,
        width: width,
        x: x,
        y: y,
    };
}

// Source: upstream/packages/render/src/renderViewport.ts:31 (sha256:477d3921ff7a5d3bbc4ee2835309eabd8a8674fc61652dc88ef86977a7139810)
pub fn is_renderable_in_viewport(
    source: crate::OpaqueHostValue,
    viewport: &RenderViewport2D,
    render_transform2_d: Option<Matrix>,
) -> bool {
    if (!compute_render_proxy_world_bounds(
        &mut (*_SCRATCH_BOUNDS.lock().unwrap()),
        (source).clone(),
    )) {
        return true;
    }
    if (render_transform2_d).is_some() {
        matrix_transform_rectangle(
            &mut (*_SCRATCH_TRANSFORMED.lock().unwrap()),
            &render_transform2_d.as_ref().unwrap(),
            &(*_SCRATCH_BOUNDS.lock().unwrap()),
        );
        (*_SCRATCH_BOUNDS.lock().unwrap()) = (*_SCRATCH_TRANSFORMED.lock().unwrap()).clone();
    }
    let obj_min_x = (*_SCRATCH_BOUNDS.lock().unwrap()).x;
    let obj_min_y = (*_SCRATCH_BOUNDS.lock().unwrap()).y;
    let obj_max_x =
        ((*_SCRATCH_BOUNDS.lock().unwrap()).x + (*_SCRATCH_BOUNDS.lock().unwrap()).width);
    let obj_max_y =
        ((*_SCRATCH_BOUNDS.lock().unwrap()).y + (*_SCRATCH_BOUNDS.lock().unwrap()).height);
    let vp_min_x = viewport.x;
    let vp_min_y = viewport.y;
    let vp_max_x = (viewport.x + viewport.width);
    let vp_max_y = (viewport.y + viewport.height);
    return (!(((obj_max_x < vp_min_x) || (obj_min_x > vp_max_x)) || (obj_max_y < vp_min_y))
        || (obj_min_y > vp_max_y));
}

// Source: upstream/packages/render/src/renderViewport.ts:59 (sha256:5296a3d8d8ddfa5f1fb1ed6ee071836ec7e027ef722494a8c3fc4b2d73857767)
pub fn is_render_proxy_in_viewport(
    proxy: &RenderProxy2D,
    viewport: &RenderViewport2D,
    render_transform2_d: Option<Matrix>,
) -> bool {
    return is_renderable_in_viewport(
        (proxy.source).clone(),
        viewport,
        ((render_transform2_d).clone()).clone(),
    );
}

// Source: upstream/packages/render/src/renderViewport.ts:70 (sha256:e4c07771d9dd001e5435ebb1141b00de6a69b5cde2855265ca0daf6c6d21b7fe)
fn is_spatial2_d_node(source: crate::OpaqueHostValue) -> bool {
    return (((source).is_some())
        && (match &(source) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "object"))
        && (false);
}

// Source: upstream/packages/render/src/renderViewport.ts:75 (sha256:9fff48d19fed43cbeb5a4cd248af698d2084de74b47889d361721625829a5f38)
static _SCRATCH_BOUNDS: std::sync::LazyLock<std::sync::Mutex<Rectangle>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_rectangle(None, None, None, None)));

// Source: upstream/packages/render/src/renderViewport.ts:76 (sha256:6127f5469f539feee807cee1c1efb279c5dffc4acc5a22d751d03d4119a9b390)
static _SCRATCH_TRANSFORMED: std::sync::LazyLock<std::sync::Mutex<Rectangle>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_rectangle(None, None, None, None)));
