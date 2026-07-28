// @generated from upstream/packages/types/src/NativeText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Rectangle, Stage, TextAutoSize, TextFormatAlign, TextVerticalAlign,
};

// Source: upstream/packages/types/src/NativeText.ts:14 (sha256:f078ff7b5a1fba1d9be2cabd7ecea006f95793b8270319a759825bed6f622e85)
#[derive(Clone, Default)]
pub struct NativeTextStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<TextFormatAlign>,
    pub bold: Option<bool>,
    pub color: Option<f64>,
    pub font: Option<String>,
    pub italic: Option<bool>,
    pub leading: Option<f64>,
    pub size: Option<f64>,
}
impl PartialEq for NativeTextStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NativeText.ts:24 (sha256:323707e2b26e1934faec2551d07b78cd93edc36d497ac9e4c967aaa9fe11128b)
#[derive(Clone, Default)]
pub struct NativeTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub style: NativeTextStyle,
    pub text: String,
    pub vertical_align: TextVerticalAlign,
    pub width: f64,
}
impl PartialEq for NativeTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NativeText.ts:37 (sha256:ffa5268de8737c98ecb432ac7edb206546da586c3017bc731709b5889c8ee2b7)
#[derive(Clone)]
pub struct NativeTextRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: f64,
    pub bounds_using_local_bounds_id: f64,
    pub bounds_using_local_transform_id: f64,
    pub can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
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
    pub compute_local_bounds_rectangle: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
    >,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub stage: Option<Stage>,
    pub element: Option<crate::OpaqueHostValue>,
    pub measured_height: f64,
    pub measured_width: f64,
}
impl PartialEq for NativeTextRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NativeText.ts:49 (sha256:63fb798185b5c8be09651c9ed151fffceaa74a6e90ced42503e237507fe47ca0)
#[derive(Clone, Default)]
pub struct NativeText {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: NativeTextData,
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
impl PartialEq for NativeText {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NativeText.ts:53 (sha256:4e663a6e6e8006b1ba071a4432421e8b640bdf963a2bbc72776e4af3a563159e)
pub const NATIVE_TEXT_KIND: &'static str = "NativeText";
