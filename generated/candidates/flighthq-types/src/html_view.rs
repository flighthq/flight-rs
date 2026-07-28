// @generated from upstream/packages/types/src/HtmlView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Rectangle, Stage,
};

// Source: upstream/packages/types/src/HtmlView.ts:3 (sha256:bba194f0689423bf8577d59419ce7b6d25107d76feeea43c14bd296df846fd93)
#[derive(Clone)]
pub struct HtmlViewData {
    pub element: Option<crate::OpaqueHostValue>,
    pub height: f64,
    pub width: f64,
}

// Source: upstream/packages/types/src/HtmlView.ts:9 (sha256:56cbcce7ac4a7d3949a91bbd7c6ff19e5a5db0a8763036515002d9b3cbc9c661)
#[derive(Clone)]
pub struct HtmlViewRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: f64,
    pub bounds_using_local_bounds_id: f64,
    pub bounds_using_local_transform_id: f64,
    pub can_add_child: std::sync::Arc<dyn Fn(Node, Node) -> bool + Send + Sync + 'static>,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: bool,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: f64,
    pub local_bounds_using_local_bounds_id: f64,
    pub local_content_id: f64,
    pub local_transform_id: f64,
    pub local_transform_using_local_transform_id: f64,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: f64,
    pub world_bounds_using_world_transform_id: f64,
    pub world_transform_id: f64,
    pub world_transform_using_local_transform_id: f64,
    pub world_transform_using_parent_transform_id: f64,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: f64,
    pub rotation_cosine: f64,
    pub rotation_sine: f64,
    pub world_matrix: Option<Matrix>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle:
        std::sync::Arc<dyn Fn(Rectangle, BoundsNodeAny) -> () + Send + Sync + 'static>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub stage: Option<Stage>,
}

// Source: upstream/packages/types/src/HtmlView.ts:11 (sha256:384fc590d337ee16124dcab9b2c796a36cf6ebd5f72e6ecc6530a8d0851df17f)
#[derive(Clone)]
pub struct HtmlView {
    pub data: HtmlViewData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub blend_mode: Option<BlendMode>,
    pub clip: Option<ClipRegion>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/HtmlView.ts:15 (sha256:1b73ef42e7b0700ee6fae93abed7ad989557e38ed0fcffe229addd777e635b6f)
pub const HTML_VIEW_KIND: &'static str = "HtmlView";
