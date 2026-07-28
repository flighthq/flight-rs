// @generated from upstream/packages/types/src/QuadBatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    QuadTransformType, Rectangle, Stage, TextureAtlas,
};

// Source: upstream/packages/types/src/QuadBatch.ts:7 (sha256:1eb0802de4e5a5b34e6e585aa05bc5147a491e7e455d4719ac072ac9577c9caa)
#[derive(Clone)]
pub struct QuadBatchData {
    pub atlas: Option<TextureAtlas>,
    pub ids: Vec<u16>,
    pub instance_count: f64,
    pub material_data: Option<Vec<Option<MaterialData>>>,
    pub transforms: Vec<f32>,
    pub transform_type: QuadTransformType,
}

// Source: upstream/packages/types/src/QuadBatch.ts:20 (sha256:75238ecb20b17ee00471a4ef63623e2e5b34bab0747cfd29e59d8662516e1942)
#[derive(Clone)]
pub struct QuadBatchRuntime {
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
    pub instance_velocities: Option<Vec<f32>>,
}

// Source: upstream/packages/types/src/QuadBatch.ts:31 (sha256:51f21dd1261747ad4e3ecb0223ce247c09417a3ad79113123cf7bde455ecfb17)
#[derive(Clone)]
pub struct QuadBatch {
    pub data: QuadBatchData,
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

// Source: upstream/packages/types/src/QuadBatch.ts:35 (sha256:4a42050cba3214e1f705bc48c73a663b704ae3d048f879dbbbff7cdc5066415c)
pub const QUAD_BATCH_KIND: &'static str = "QuadBatch";
