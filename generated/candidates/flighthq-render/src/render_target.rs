// @generated from upstream/packages/render/src/renderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_matrix, inverse_matrix, multiply_matrix};
use flighthq_node::get_node_local_matrix;
use flighthq_types::{
    Adjustment, ColorTransform, DisplayObject, InteractionSignals, Matrix, MatrixLike, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, RectangleLike,
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

// Source: upstream/packages/render/src/renderTarget.ts:5 (sha256:d096576df44fb3f2e8c6e8407060ad62e0000a87855d01af617a915673f40ae7)
#[derive(Clone)]
pub struct RenderTargetSizeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
}
impl PartialEq for RenderTargetSizeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/renderTarget.ts:15 (sha256:0e5248393bd9b52b3eded41ace6a29aa07272535ab3e4c7c17783601ccd9ade0)
pub fn compute_display_object_render_target_transform(
    out_render_transform: &mut MatrixLike,
    source: &DisplayObject,
    bounds: &RectangleLike,
    content_x: Option<f64>,
    content_y: Option<f64>,
) -> () {
    let content_x = content_x.unwrap_or(0.0_f64);
    let content_y = content_y.unwrap_or(0.0_f64);
    let local_transform = get_node_local_matrix(source);
    inverse_matrix(&mut (*_TEMP_INV_LOCAL.lock().unwrap()), &local_transform);
    (*_TEMP_TRANSLATION.lock().unwrap()).a = 1.0_f64;
    (*_TEMP_TRANSLATION.lock().unwrap()).b = 0.0_f64;
    (*_TEMP_TRANSLATION.lock().unwrap()).c = 0.0_f64;
    (*_TEMP_TRANSLATION.lock().unwrap()).d = 1.0_f64;
    (*_TEMP_TRANSLATION.lock().unwrap()).tx = (content_x - bounds.x);
    (*_TEMP_TRANSLATION.lock().unwrap()).ty = (content_y - bounds.y);
    multiply_matrix(
        out_render_transform,
        &(*_TEMP_TRANSLATION.lock().unwrap()),
        &(*_TEMP_INV_LOCAL.lock().unwrap()),
    );
}

// Source: upstream/packages/render/src/renderTarget.ts:37 (sha256:c8cebf7c9e1b2540b1de810528301357d5840c20b2f39e3fc0a978a55a7da08c)
pub fn compute_render_cache_transform(
    out_cache_transform: &mut MatrixLike,
    bounds: &RectangleLike,
    content_x: Option<f64>,
    content_y: Option<f64>,
) -> () {
    let content_x = content_x.unwrap_or(0.0_f64);
    let content_y = content_y.unwrap_or(0.0_f64);
    out_cache_transform.a = 1.0_f64;
    out_cache_transform.b = 0.0_f64;
    out_cache_transform.c = 0.0_f64;
    out_cache_transform.d = 1.0_f64;
    out_cache_transform.tx = (bounds.x - content_x);
    out_cache_transform.ty = (bounds.y - content_y);
}

// Source: upstream/packages/render/src/renderTarget.ts:51 (sha256:38b43bf331e204278fcbaaa4fefe1ed84a8bdeb2d5bed1ee469edd88cd801eaa)
#[derive(Clone)]
struct ComputeRenderTargetSizeRecord2 {
    __flight_identity: std::sync::Arc<()>,
    width: f64,
    height: f64,
}
impl PartialEq for ComputeRenderTargetSizeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn compute_render_target_size(
    bounds: &RectangleLike,
    padding: Option<f64>,
    min_width: Option<f64>,
    min_height: Option<f64>,
) -> ComputeRenderTargetSizeRecord2 {
    let padding = padding.unwrap_or(0.0_f64);
    let min_width = min_width.unwrap_or(1.0_f64);
    let min_height = min_height.unwrap_or(1.0_f64);
    return ComputeRenderTargetSizeRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        width: (min_width).max(((bounds.width).ceil() + (padding * 2.0_f64))),
        height: (min_height).max(((bounds.height).ceil() + (padding * 2.0_f64))),
    };
}

// Source: upstream/packages/render/src/renderTarget.ts:63 (sha256:a4e07af32a997adceac418cb978b10ba3defff678c3a72d4f27db07d01053d79)
static _TEMP_INV_LOCAL: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });

// Source: upstream/packages/render/src/renderTarget.ts:64 (sha256:3354b26ee33922d785cfe5d613d805b01aa08c63f65cd8de57cd254b05d9e7a2)
static _TEMP_TRANSLATION: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });
