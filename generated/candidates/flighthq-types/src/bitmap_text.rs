// @generated from upstream/packages/types/src/BitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, GlyphSource,
    InteractionSignals, Kind, Material, MaterialData, Matrix, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, QuadBatch, Rectangle, Stage,
};

// Source: upstream/packages/types/src/BitmapText.ts:9 (sha256:fb814b59647f58d51fc3eb28845ee76447c5c5b70f0cd7733f4325a6e519913f)
pub type BitmapTextAlign = String;

// Source: upstream/packages/types/src/BitmapText.ts:20 (sha256:1ce347f6ccf8f3976756b26f27a7aaf632a2a078faea3f71b7a73b0305ab6ccc)
#[derive(Clone, Default)]
pub struct BitmapTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: BitmapTextAlign,
    pub color: f64,
    pub glyph_source: Option<GlyphSource>,
    pub letter_spacing: f64,
    pub line_height: f64,
    pub text: String,
    pub wrap_width: Option<f64>,
}
impl PartialEq for BitmapTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapText.ts:40 (sha256:750a82db9e8a5c3f07be88e895ce992c3c0093c2036fda39680353adfa4613f9)
#[derive(Clone)]
pub struct BitmapTextRuntime {
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
    pub quad_batches: Vec<QuadBatch>,
}
impl PartialEq for BitmapTextRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapText.ts:52 (sha256:58a2b715038e0386ac8f038f18f536356df7c840c61bd24fd4846f1d8cc04048)
#[derive(Clone, Default)]
pub struct BitmapText {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: BitmapTextData,
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
impl PartialEq for BitmapText {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapText.ts:58 (sha256:30b3f8bb3e2d7fc885abd15048874ed6359a1c0c62dca87dbb12219b721bfea0)
#[derive(Clone, Default)]
pub struct BitmapTextOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<BitmapTextAlign>,
    pub color: Option<f64>,
    pub letter_spacing: Option<f64>,
    pub line_height: Option<f64>,
    pub text: Option<String>,
    pub wrap_width: Option<f64>,
}
impl PartialEq for BitmapTextOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapText.ts:67 (sha256:0407460c41722f239ace0d39d723460b0ccd154dcc53002ed4852bda8f9a8fcb)
pub const BITMAP_TEXT_KIND: &'static str = "BitmapText";
