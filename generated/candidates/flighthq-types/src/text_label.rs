// @generated from upstream/packages/types/src/TextLabel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Rectangle, Stage, TextAutoSize, TextFormat, TextLayoutParams, TextLayoutResult,
    TextMeasureFunction, TextVerticalAlign,
};

// Source: upstream/packages/types/src/TextLabel.ts:7 (sha256:3f6682b750d0898585b3453f5d4b9d32cacb0721f0e3e4ce087020840c0ef768)
#[derive(Clone)]
pub struct TextLabelData {
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub text: String,
    pub text_format: TextFormat,
    pub vertical_align: TextVerticalAlign,
    pub width: f64,
}

// Source: upstream/packages/types/src/TextLabel.ts:18 (sha256:6795acda2aa36d88e003498810c2d72cf123d5046fbeab982770ec4810321cc6)
#[derive(Clone)]
pub struct TextLabelRuntime {
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
    pub build_text_layout_params: std::sync::Arc<
        dyn Fn(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + Sync + 'static,
    >,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: f64,
}

// Source: upstream/packages/types/src/TextLabel.ts:31 (sha256:6e6f193698f819105eae685200bdded28eb20248352584c28b2b560109d65e09)
#[derive(Clone)]
pub struct TextLabel {
    pub data: TextLabelData,
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

// Source: upstream/packages/types/src/TextLabel.ts:35 (sha256:800f6cacad6f11058247fb3a2fe6ad16ab7f1ea94d9d9a986dfb4599742ef0e2)
pub const TEXT_LABEL_KIND: &'static str = "TextLabel";
