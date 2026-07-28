// @generated from upstream/packages/sprite/src/tilemap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_displayobject::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_signals::create_signal;
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Rectangle, Stage, TILEMAP_KIND as tilemap_kind_constant, Tilemap, TilemapData,
    TilemapRuntime, TilemapSignals, Tileset, Vector2Like,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub tileset: Option<Tileset>,
    pub columns: Option<f64>,
    pub rows: Option<f64>,
    pub tiles: Option<Vec<i16>>,
    pub material_data: Option<Vec<Option<MaterialData>>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
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
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub stage: Option<Stage>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_multiplier: Option<f64>,
    pub alpha_offset: Option<f64>,
    pub blue_multiplier: Option<f64>,
    pub blue_offset: Option<f64>,
    pub green_multiplier: Option<f64>,
    pub green_offset: Option<f64>,
    pub red_multiplier: Option<f64>,
    pub red_offset: Option<f64>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
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
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
}
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
    pub __flight_identity: std::sync::Arc<()>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotation: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/sprite/src/tilemap.ts:21 (sha256:5ac7db3d9c394f7ca11e767732eaeee96cd53e673331fd37e9bc49f9580d61b5)
pub fn clear_tilemap(tilemap: &mut Tilemap) -> () {
    tilemap.data.tiles.fill((-1.0_f64) as i16);
    let signals = get_tilemap_signals(tilemap);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_cleared.emit).clone())();
    }
}

// Source: upstream/packages/sprite/src/tilemap.ts:32 (sha256:d659f9ad78ac2a12f1d942a2a70daa9169fe82982931c2aaefdbaa30849db2f0)
pub fn clone_tilemap(source: &Tilemap) -> Tilemap {
    return create_tilemap(Some(Tilemap {
        __flight_identity: std::sync::Arc::new(()),
        data: TilemapData {
            __flight_identity: std::sync::Arc::new(()),
            columns: source.data.columns,
            material_data: if ((source.data.material_data).clone()).is_some() {
                Some(
                    ((source.data.material_data).clone())
                        .as_ref()
                        .unwrap()
                        .clone(),
                )
            } else {
                None
            },
            rows: source.data.rows,
            tiles: ((source.data.tiles).clone()).clone(),
            tileset: (source.data.tileset).clone(),
        },
    }));
}

// Source: upstream/packages/sprite/src/tilemap.ts:45 (sha256:07bb34bff9319d7d59f4e24e2c5512ed3a652bcc6047d9dc9b564d43e3260353)
pub fn compute_tilemap_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let tilemap = source;
    let tileset = (tilemap.data.tileset).clone();
    let columns = tilemap.data.columns;
    let rows = tilemap.data.rows;
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    out.width = if (tileset).is_some() {
        (columns * tileset.as_ref().unwrap().tile_width)
    } else {
        0.0_f64
    };
    out.height = if (tileset).is_some() {
        (rows * tileset.as_ref().unwrap().tile_height)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/sprite/src/tilemap.ts:54 (sha256:930c91dc30a707a09a1bee01e624382680cfd059ddbf851b7e0a7dfce9fbdbd9)
pub fn create_tilemap(obj: Option<Tilemap>) -> Tilemap {
    return create_display_object_generic(
        (tilemap_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_tilemap_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_tilemap_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/sprite/src/tilemap.ts:58 (sha256:49e8f9b267b9649105bdadc8dfa31c5f3b308db639b167445e8346a897457e5d)
pub fn create_tilemap_data(data: Option<FlightPartialRecord1>) -> TilemapData {
    let columns = (data.as_ref().and_then(|value| value.columns)).unwrap_or(0.0_f64);
    let rows = (data.as_ref().and_then(|value| value.rows)).unwrap_or(0.0_f64);
    return TilemapData {
        __flight_identity: std::sync::Arc::new(()),
        columns: columns,
        rows: rows,
        material_data: data
            .as_ref()
            .and_then(|value| (value.material_data).clone()),
        tiles: (data.as_ref().and_then(|value| (value.tiles).clone()))
            .unwrap_or(vec![0_i16; (columns * rows) as usize].fill((-1.0_f64) as i16)),
        tileset: data.as_ref().and_then(|value| (value.tileset).clone()),
    };
}

// Source: upstream/packages/sprite/src/tilemap.ts:70 (sha256:55e2f381660956029b468dce5633be7c2966cedcb73eb40e4b1043596901cd86)
pub fn create_tilemap_runtime() -> TilemapRuntime {
    return {
        let __flight_source =
            &(create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone())));
        TilemapRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            binding: (__flight_source.binding).clone(),
            appearance_id: __flight_source.appearance_id,
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            color_adjustments: (__flight_source.color_adjustments).clone(),
            resolved_color_transform: (__flight_source.resolved_color_transform).clone(),
            color_adjustments_channel_mixing: __flight_source.color_adjustments_channel_mixing,
            traits: (__flight_source.traits).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_using_local_bounds_id: __flight_source.local_bounds_using_local_bounds_id,
            local_content_id: __flight_source.local_content_id,
            local_transform_id: __flight_source.local_transform_id,
            local_transform_using_local_transform_id: __flight_source
                .local_transform_using_local_transform_id,
            node_signals: (__flight_source.node_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            parent: (__flight_source.parent).clone(),
            world_bounds_using_local_bounds_id: __flight_source.world_bounds_using_local_bounds_id,
            world_bounds_using_world_transform_id: __flight_source
                .world_bounds_using_world_transform_id,
            world_transform_id: __flight_source.world_transform_id,
            world_transform_using_local_transform_id: __flight_source
                .world_transform_using_local_transform_id,
            world_transform_using_parent_transform_id: __flight_source
                .world_transform_using_parent_transform_id,
            local_matrix: (__flight_source.local_matrix).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            world_matrix: (__flight_source.world_matrix).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            world_bounds_rectangle: (__flight_source.world_bounds_rectangle).clone(),
            stage: (__flight_source.stage).clone(),
        }
    };
}

// Source: upstream/packages/sprite/src/tilemap.ts:74 (sha256:02224a314cbdcd785ec18c3b46dc35ba472835fe9dfe78bfb30b910a8d033542)
pub fn create_tilemap_signals() -> TilemapSignals {
    return TilemapSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_cleared: create_signal(),
        on_tile_changed: create_signal(),
        on_tiles_changed: create_signal(),
    };
}

// Source: upstream/packages/sprite/src/tilemap.ts:87 (sha256:e0f834dfc57e99dcfac08b3b68a4976713661127cba245fe2077c5bb2ee3e5f5)
pub fn enable_tilemap_signals(target: &mut Tilemap) -> TilemapSignals {
    let mut s = {
        let __flight_source = &((*target).clone());
        TilemapWithSignals {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    };
    return {
        s[*TILEMAP_SIGNALS_SLOT as usize]?? = create_tilemap_signals();
        s[*TILEMAP_SIGNALS_SLOT as usize]
    };
}

// Source: upstream/packages/sprite/src/tilemap.ts:92 (sha256:26ceb80fdcc2e0b7166033bec20d9615c32ec320c1d452bae659bb91b24d7d49)
pub fn fill_tilemap_tiles(tilemap: &mut Tilemap, id: f64) -> () {
    tilemap.data.tiles.fill((id) as i16);
}

// Source: upstream/packages/sprite/src/tilemap.ts:100 (sha256:9ccb4c2b87a6817e07198f0e8d0ee96a5e839c9b301f839e30de062f97581bfb)
pub fn get_tilemap_column_at_x(source: &Tilemap, x: f64) -> f64 {
    let tileset = (source.data.tileset).clone();
    let columns = source.data.columns;
    if ((tileset).is_none()) || (tileset.as_ref().unwrap().tile_width <= 0.0_f64) {
        return (-1.0_f64);
    }
    let col = (x / tileset.as_ref().unwrap().tile_width).floor();
    if (col < 0.0_f64) || (col >= columns) {
        return (-1.0_f64);
    }
    return col;
}

// Source: upstream/packages/sprite/src/tilemap.ts:113 (sha256:be8e089a32c13454211928a1dc8b046ba476504475e359c294978a01f6283ca5)
pub fn get_tilemap_column_row_at_point(
    out: &mut Vector2Like,
    source: &Tilemap,
    x: f64,
    y: f64,
) -> bool {
    let col = get_tilemap_column_at_x(source, x);
    let row = get_tilemap_row_at_y(source, y);
    if (col < 0.0_f64) || (row < 0.0_f64) {
        return false;
    }
    out.x = col;
    out.y = row;
    return true;
}

// Source: upstream/packages/sprite/src/tilemap.ts:126 (sha256:f1392e0777ba09697c0f6367fe865854f81b8aa6114fc0ec53ba541342c9f857)
pub fn get_tilemap_row_at_y(source: &Tilemap, y: f64) -> f64 {
    let tileset = (source.data.tileset).clone();
    let rows = source.data.rows;
    if ((tileset).is_none()) || (tileset.as_ref().unwrap().tile_height <= 0.0_f64) {
        return (-1.0_f64);
    }
    let row = (y / tileset.as_ref().unwrap().tile_height).floor();
    if (row < 0.0_f64) || (row >= rows) {
        return (-1.0_f64);
    }
    return row;
}

// Source: upstream/packages/sprite/src/tilemap.ts:134 (sha256:eb0276a30d87e7cc575914fc45ab0141d148e1364cc2e13d6909ddf5c236dcce)
pub fn get_tilemap_runtime(source: &Tilemap) -> TilemapRuntime {
    return {
        let __flight_source = &(get_display_object_runtime(source));
        TilemapRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            binding: (__flight_source.binding).clone(),
            appearance_id: __flight_source.appearance_id,
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            color_adjustments: (__flight_source.color_adjustments).clone(),
            resolved_color_transform: (__flight_source.resolved_color_transform).clone(),
            color_adjustments_channel_mixing: __flight_source.color_adjustments_channel_mixing,
            traits: (__flight_source.traits).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_using_local_bounds_id: __flight_source.local_bounds_using_local_bounds_id,
            local_content_id: __flight_source.local_content_id,
            local_transform_id: __flight_source.local_transform_id,
            local_transform_using_local_transform_id: __flight_source
                .local_transform_using_local_transform_id,
            node_signals: (__flight_source.node_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            parent: (__flight_source.parent).clone(),
            world_bounds_using_local_bounds_id: __flight_source.world_bounds_using_local_bounds_id,
            world_bounds_using_world_transform_id: __flight_source
                .world_bounds_using_world_transform_id,
            world_transform_id: __flight_source.world_transform_id,
            world_transform_using_local_transform_id: __flight_source
                .world_transform_using_local_transform_id,
            world_transform_using_parent_transform_id: __flight_source
                .world_transform_using_parent_transform_id,
            local_matrix: (__flight_source.local_matrix).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            world_matrix: (__flight_source.world_matrix).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            world_bounds_rectangle: (__flight_source.world_bounds_rectangle).clone(),
            stage: (__flight_source.stage).clone(),
        }
    };
}

// Source: upstream/packages/sprite/src/tilemap.ts:139 (sha256:f85ee86cdf5343e5a259f6998071cecd4ba849294ffbe4580edcf996ee594cda)
pub fn get_tilemap_signals(source: &Tilemap) -> Option<TilemapSignals> {
    return Some(
        {
            let __flight_source = &((*source).clone());
            TilemapWithSignals {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            }
        }[*TILEMAP_SIGNALS_SLOT as usize]
            .clone(),
    );
}

// Source: upstream/packages/sprite/src/tilemap.ts:143 (sha256:858a6b7394fbe5f1a3f34239a8556ce7d3ee4604fe87f820af90883a1656256b)
pub fn get_tilemap_tile(tilemap: &Tilemap, column: f64, row: f64) -> f64 {
    let columns = tilemap.data.columns;
    let rows = tilemap.data.rows;
    if (((column < 0.0_f64) || (column >= columns)) || (row < 0.0_f64)) || (row >= rows) {
        return (-1.0_f64);
    }
    return (tilemap.data.tiles[((row * columns) + column) as usize] as f64);
}

// Source: upstream/packages/sprite/src/tilemap.ts:154 (sha256:2a7aa521234de0c10bf206426472adc2e06711788db8dcdad3ca603625c7dc81)
pub fn get_tilemap_tile_at_point(source: &Tilemap, point: &Vector2Like) -> f64 {
    return get_tilemap_tile_at_point_xy(source, point.x, point.y);
}

// Source: upstream/packages/sprite/src/tilemap.ts:161 (sha256:5a55bff34b3107105e2ad6e3d6a4868b4e2552247648d76eccef288b47b62df2)
pub fn get_tilemap_tile_at_point_xy(source: &Tilemap, x: f64, y: f64) -> f64 {
    let col = get_tilemap_column_at_x(source, x);
    let row = get_tilemap_row_at_y(source, y);
    if (col < 0.0_f64) || (row < 0.0_f64) {
        return (-1.0_f64);
    }
    return get_tilemap_tile(source, col, row);
}

// Source: upstream/packages/sprite/src/tilemap.ts:172 (sha256:5a51de3bac3b0b6a96f0951af658fef642ae1ed455e967f49c2be2892857a857)
pub fn get_tilemap_tile_rect(out: &mut Rectangle, source: &Tilemap, column: f64, row: f64) -> bool {
    let tileset = (source.data.tileset).clone();
    let columns = source.data.columns;
    let rows = source.data.rows;
    if (((((tileset).is_none()) || (column < 0.0_f64)) || (column >= columns)) || (row < 0.0_f64))
        || (row >= rows)
    {
        return false;
    }
    out.x = (column * tileset.as_ref().unwrap().tile_width);
    out.y = (row * tileset.as_ref().unwrap().tile_height);
    out.width = tileset.as_ref().unwrap().tile_width;
    out.height = tileset.as_ref().unwrap().tile_height;
    return true;
}

// Source: upstream/packages/sprite/src/tilemap.ts:182 (sha256:d3e5c7c22c4774a3ee43232e8ebae8d3086d3c8d2d7c5d886237871031d9f579)
pub fn resize_tilemap(tilemap: &mut Tilemap, columns: f64, rows: f64) -> () {
    let mut new_tiles = vec![0_i16; (columns * rows) as usize].fill((-1.0_f64) as i16);
    let copy_columns = (columns).min(tilemap.data.columns);
    let copy_rows = (rows).min(tilemap.data.rows);
    {
        let mut r = 0.0_f64;
        while (r < copy_rows) {
            {
                let mut c = 0.0_f64;
                while (c < copy_columns) {
                    new_tiles[((r * columns) + c) as usize] =
                        (tilemap.data.tiles[((r * tilemap.data.columns) + c) as usize] as f64);
                    {
                        c += 1.0;
                        c
                    };
                }
            }
            {
                r += 1.0;
                r
            };
        }
    }
    tilemap.data.columns = columns;
    tilemap.data.rows = rows;
    tilemap.data.tiles = new_tiles;
}

// Source: upstream/packages/sprite/src/tilemap.ts:198 (sha256:62cba131147cd1572947b7f05424962f57270e94382dec879e01040e935d5ea9)
pub fn set_tilemap_tile(tilemap: &mut Tilemap, column: f64, row: f64, id: f64) -> () {
    let columns = tilemap.data.columns;
    let rows = tilemap.data.rows;
    if (((column < 0.0_f64) || (column >= columns)) || (row < 0.0_f64)) || (row >= rows) {
        return;
    }
    tilemap.data.tiles[((row * columns) + column) as usize] = (id) as i16;
    let signals = get_tilemap_signals(tilemap);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_tile_changed.emit).clone())(column, row, id);
    }
}

// Source: upstream/packages/sprite/src/tilemap.ts:212 (sha256:8354424cc02151cc7c13c9b3bf02fec5ded21504c738e29960e4a35fa3d2fb34)
pub fn set_tilemap_tiles(
    tilemap: &mut Tilemap,
    ids: &Vec<f64>,
    offset_column: f64,
    offset_row: f64,
    width: f64,
    height: f64,
) -> () {
    let columns = tilemap.data.columns;
    let rows = tilemap.data.rows;
    {
        let mut r = 0.0_f64;
        while (r < height) {
            let target_row = (offset_row + r);
            if (target_row < 0.0_f64) || (target_row >= rows) {
                {
                    r += 1.0;
                    r
                };
                continue;
            }
            {
                let mut c = 0.0_f64;
                while (c < width) {
                    let target_col = (offset_column + c);
                    if (target_col < 0.0_f64) || (target_col >= columns) {
                        {
                            c += 1.0;
                            c
                        };
                        continue;
                    }
                    tilemap.data.tiles[((target_row * columns) + target_col) as usize] =
                        (ids[((r * width) + c) as usize].clone()) as i16;
                    {
                        c += 1.0;
                        c
                    };
                }
            }
            {
                r += 1.0;
                r
            };
        }
    }
    let signals = get_tilemap_signals(tilemap);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_tiles_changed.emit).clone())(
            offset_column,
            offset_row,
            width,
            height,
        );
    }
}

// Source: upstream/packages/sprite/src/tilemap.ts:234 (sha256:501b9fca163dc75799a15df3e0ad52fecc82b6e26a584723c13e6e1e2d3fe8c0)
static DEFAULT_METHODS: std::sync::LazyLock<FlightPartialRecord2> =
    std::sync::LazyLock::new(|| FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                compute_tilemap_local_bounds_rectangle(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                )
            },
        )
            as Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>))),
        binding: None,
        appearance_id: None,
        bounds_using_local_bounds_id: None,
        bounds_using_local_transform_id: None,
        can_add_child: None,
        children: None,
        color_adjustments: None,
        resolved_color_transform: None,
        color_adjustments_channel_mixing: None,
        traits: None,
        interaction_signals: None,
        local_bounds_id: None,
        local_bounds_using_local_bounds_id: None,
        local_content_id: None,
        local_transform_id: None,
        local_transform_using_local_transform_id: None,
        node_signals: None,
        interaction_state: None,
        parent: None,
        world_bounds_using_local_bounds_id: None,
        world_bounds_using_world_transform_id: None,
        world_transform_id: None,
        world_transform_using_local_transform_id: None,
        world_transform_using_parent_transform_id: None,
        local_matrix: None,
        rotation_angle: None,
        rotation_cosine: None,
        rotation_sine: None,
        world_matrix: None,
        bounds_rectangle: None,
        local_bounds_rectangle: None,
        world_bounds_rectangle: None,
        stage: None,
    });

// Source: upstream/packages/sprite/src/tilemap.ts:238 (sha256:b448b031099576eae638daa302bb5336d794bd3866167b48e8c70433e186bb1a)
static TILEMAP_SIGNALS_SLOT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/sprite/src/tilemap.ts:240 (sha256:ee06a7b14b1a3717ee9a9d8da8d7aa204039e15c528801f34ff531577d871707)
#[derive(Clone, Default)]
struct TilemapWithSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for TilemapWithSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
