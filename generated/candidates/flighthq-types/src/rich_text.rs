// @generated from upstream/packages/types/src/RichText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Rectangle, RichTextContent, Stage, TextAutoSize, TextFieldSignals, TextFormat, TextFormatRange,
    TextInputState, TextLabel, TextLayoutParams, TextLayoutResult, TextMeasureFunction,
    TextVerticalAlign,
};

// Source: upstream/packages/types/src/RichText.ts:9 (sha256:fa82e08e1863fcc75e3ed9619dc8585f19565703bc84971444398c1df93031eb)
#[derive(Clone)]
pub struct RichTextData {
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub text: String,
    pub text_format: TextFormat,
    pub vertical_align: TextVerticalAlign,
    pub width: f64,
    pub background: bool,
    pub background_color: f64,
    pub border: bool,
    pub border_color: f64,
    pub condense_white: bool,
    pub default_text_format: TextFormat,
    pub max_chars: f64,
    pub mouse_wheel_enabled: bool,
    pub multiline: bool,
    pub scroll_h: f64,
    pub scroll_v: f64,
    pub selectable: bool,
    pub text_color: f64,
    pub text_format_ranges: Vec<TextFormatRange>,
    pub word_wrap: bool,
}

// Source: upstream/packages/types/src/RichText.ts:42 (sha256:8366b22af6581d9b3d860205d8d5245e7bb40398342313332aa3c7da2e420aa1)
#[derive(Clone)]
pub struct RichTextRuntime {
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
    pub input: Option<TextInputState>,
    pub rich_text_content: Option<RichTextContent>,
    pub selection_begin_index: f64,
    pub selection_end_index: f64,
    pub text_field_signals: Option<TextFieldSignals>,
}

// Source: upstream/packages/types/src/RichText.ts:57 (sha256:ede1beea3240687757ee8455992b246d3497476a47de43d9b8e5d02d8b73abe7)
#[derive(Clone)]
pub struct RichText {
    pub data: RichTextData,
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

// Source: upstream/packages/types/src/RichText.ts:61 (sha256:596b8a1b265ecce1ee0865dbb2e71192fc576e385865362468b050f38fe00952)
pub const RICH_TEXT_KIND: &'static str = "RichText";
