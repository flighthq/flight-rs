// @generated from upstream/packages/types/src/Tilemap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Rectangle, Stage, Tileset,
};

// Source: upstream/packages/types/src/Tilemap.ts:5 (sha256:566599ce0525cecc527b700162a16b1d22b913e37ba55d06083adf975852aa1c)
#[derive(Clone)]
pub struct TilemapData {
    pub tileset: Option<Tileset>,
    pub columns: f64,
    pub rows: f64,
    pub tiles: Vec<i16>,
    pub material_data: Option<Vec<Option<MaterialData>>>,
}

// Source: upstream/packages/types/src/Tilemap.ts:15 (sha256:f874667906594d578d67434836527cfbf0d2b6698b49f52f359e7fecee47af82)
#[derive(Clone)]
pub struct TilemapRuntime {
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

// Source: upstream/packages/types/src/Tilemap.ts:17 (sha256:07492065cf96f90ceaa42389cb8a5707074488d0ff08275fdf0a289f4ed61cf9)
#[derive(Clone)]
pub struct Tilemap {
    pub data: TilemapData,
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

// Source: upstream/packages/types/src/Tilemap.ts:21 (sha256:62481eb694aea171dfe962d6b52ba7db3a8a9d130002f7e9f0f7acc217dc7cd3)
pub const TILEMAP_KIND: &'static str = "Tilemap";
