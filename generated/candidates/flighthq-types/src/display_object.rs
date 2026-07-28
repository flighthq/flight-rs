// @generated from upstream/packages/types/src/DisplayObject.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeDataFactory, NodeInteractionState,
    NodeRuntimeFactory, NodeSignals, NodeTraitsKey, Rectangle, Stage,
};

// Source: upstream/packages/types/src/DisplayObject.ts:9 (sha256:6a8c06bc00834ebd8b57775a5f59f472af7b5ecf7f98b1939235b0cca1a66dc4)
#[derive(Clone)]
pub struct DisplayObject {
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

// Source: upstream/packages/types/src/DisplayObject.ts:13 (sha256:ede92710ddf9f1e1a1e8a29eaca2c42e699bd220631531bdcb4363f81ef9a95b)
#[derive(Clone)]
pub struct DisplayObjectTraits {
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

// Source: upstream/packages/types/src/DisplayObject.ts:17 (sha256:af6b3e6bcc2bd2d24e7d294305d34cfb623314545885e763f3205e2be1eabe46)
#[derive(Clone)]
pub struct DisplayObjectData {}

// Source: upstream/packages/types/src/DisplayObject.ts:18 (sha256:c916f06b9b3fbab4190d2fe99dcf38db092f10037c67a399dbb9c0b5a8343a6e)
pub const DISPLAY_OBJECT_KIND: &'static str = "DisplayObject";

// Source: upstream/packages/types/src/DisplayObject.ts:19 (sha256:b410fd498bf7cb937ae2ccefbf1693a8bc26b731832cbac2eff9ecce1aff3ea5)
pub const DISPLAY_OBJECT_TRAITS_KEY: &'static str = "DisplayObjectTraits";

// Source: upstream/packages/types/src/DisplayObject.ts:22 (sha256:d48b6602d9ab7bac3e71da19493106c092f4a67899e296ec88436aaa4a539220)
#[derive(Clone)]
pub struct DisplayObjectRuntime {
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

// Source: upstream/packages/types/src/DisplayObject.ts:25 (sha256:ae86d59cbfc9737e919b7cb4873d1df87027e4321704d93c75ed2acaa8700149)
pub type DisplayObjectDataFactory = NodeDataFactory;

// Source: upstream/packages/types/src/DisplayObject.ts:26 (sha256:bba4e0042ba61259518104d9ab6eea77fc1d7a233dc336980b70241c5ab0beee)
pub type DisplayObjectRuntimeFactory = NodeRuntimeFactory;
