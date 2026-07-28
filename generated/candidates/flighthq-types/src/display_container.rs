// @generated from upstream/packages/types/src/DisplayContainer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, DisplayObjectData,
    InteractionSignals, Kind, Material, MaterialData, Matrix, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, Rectangle, Stage,
};

// Source: upstream/packages/types/src/DisplayContainer.ts:3 (sha256:0bf73da11233b3ad1be3ed370d0fda3abc30c492e26b44f74f96b593cbd8aaf6)
#[derive(Clone)]
pub struct DisplayContainerRuntime {
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

// Source: upstream/packages/types/src/DisplayContainer.ts:5 (sha256:28a273c427a0476a0f2672ce0c0cd835f529dbd3dfed712a62ab4addfc130b5f)
#[derive(Clone)]
pub struct DisplayContainer {
    pub data: Option<DisplayObjectData>,
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
