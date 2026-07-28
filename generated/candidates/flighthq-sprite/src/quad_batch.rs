// @generated from upstream/packages/sprite/src/quadBatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_displayobject::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_geometry::{
    copy_rectangle, create_rectangle, reserve_float32_array, reserve_uint16_array,
};
use flighthq_node::invalidate_node_local_bounds;
use flighthq_signals::create_signal;
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, QUAD_BATCH_KIND as quad_batch_kind_constant, QuadBatch, QuadBatchData,
    QuadBatchRuntime, QuadBatchSignals, QuadTransformType, Rectangle, RectangleLike, Stage,
    TextureAtlas, Vector2Like,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub ids: Option<Vec<u16>>,
    pub instance_count: Option<f64>,
    pub material_data: Option<Vec<Option<MaterialData>>>,
    pub transforms: Option<Vec<f32>>,
    pub transform_type: Option<QuadTransformType>,
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
    pub instance_velocities: Option<Vec<f32>>,
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
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
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
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
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
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
}
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord14 {
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
impl PartialEq for FlightPartialRecord14 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:25 (sha256:11ac8c2d22177f9e5d228dd84ce31cd3dfb75e33a76743fd5bcf560978a399b9)
const QUAD_VECTOR2_STRIDE: f64 = 2.0_f64;

// Source: upstream/packages/sprite/src/quadBatch.ts:26 (sha256:6bcbc0de0baf8f48aaa34ac1b1d3e106aced604c075599da9495396ea1e3bc2a)
const QUAD_MATRIX3_X2_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/sprite/src/quadBatch.ts:29 (sha256:351c314da04e951bf76448a880b34071a90aa4f691038b690fff14d843a4a3a3)
pub const QUAD_BATCH_DELETED_ID: f64 = 65535.0_f64;

// Source: upstream/packages/sprite/src/quadBatch.ts:37 (sha256:78d3f61d7196b9230343d37c43cbd3fdd3e7b425845734223e75d33fefae1b98)
pub fn append_quad_batch_instance(target: &mut QuadBatch, id: f64, x: f64, y: f64) -> f64 {
    let index = target.data.instance_count;
    resize_quad_batch(target, (index + 1.0_f64));
    target.data.ids[index as usize] = (id) as u16;
    let o = (index * QUAD_VECTOR2_STRIDE);
    target.data.transforms[o as usize] = (x) as f32;
    target.data.transforms[(o + 1.0_f64) as usize] = (y) as f32;
    let signals = get_quad_batch_signals(target);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_instance_appended.emit).clone())(index);
    }
    return index;
}

// Source: upstream/packages/sprite/src/quadBatch.ts:50 (sha256:c7365eadd499c68716814610364743e97aacabf0963f266cb6611c06a2cb78d9)
pub fn clear_quad_batch(target: &mut QuadBatch) -> () {
    target.data.instance_count = 0.0_f64;
    let signals = get_quad_batch_signals(target);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_cleared.emit).clone())();
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:61 (sha256:32a80b0e08f4381b18f2517e8d3ebd96a9847b643491757aad0deb63d6121d90)
pub fn clone_quad_batch(source: &QuadBatch) -> QuadBatch {
    return create_quad_batch(Some(QuadBatch {
        __flight_identity: std::sync::Arc::new(()),
        data: QuadBatchData {
            __flight_identity: std::sync::Arc::new(()),
            atlas: (source.data.atlas).clone(),
            ids: ((source.data.ids).clone()).clone(),
            instance_count: source.data.instance_count,
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
            transforms: ((source.data.transforms).clone()).clone(),
            transform_type: (source.data.transform_type).clone(),
        },
    }));
}

// Source: upstream/packages/sprite/src/quadBatch.ts:75 (sha256:3a1c25ae0914a827c1ad4b4cefc8d436f87f8b9a446519023ab102ac85b1c5a2)
fn copy_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let runtime = get_display_object_runtime(&source);
    if ((runtime.local_bounds_rectangle).clone()).is_some() {
        copy_rectangle(out, runtime.local_bounds_rectangle.as_ref().unwrap());
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:91 (sha256:3e48b2395aae4e7b383b67bebfe85014cbd7e2abfa2920733e3f0a927826757b)
pub fn compact_quad_batch(target: &mut QuadBatch) -> () {
    if (target.data.instance_count == 0.0_f64) {
        return;
    }
    let stride = get_quad_batch_transform_stride((target.data.transform_type).clone());
    let mut write = 0.0_f64;
    {
        let mut read = 0.0_f64;
        while (read < target.data.instance_count) {
            if ((target.data.ids[read as usize] as f64) == QUAD_BATCH_DELETED_ID) {
                {
                    read += 1.0;
                    read
                };
                continue;
            }
            if (write != read) {
                target.data.ids[write as usize] = (target.data.ids[read as usize] as f64) as u16;
                let dst = (write * stride);
                let src = (read * stride);
                {
                    let mut k = 0.0_f64;
                    while (k < stride) {
                        target.data.transforms[(dst + k) as usize] =
                            (target.data.transforms[(src + k) as usize] as f64) as f32;
                        {
                            k += 1.0;
                            k
                        };
                    }
                }
                if ((target.data.material_data).clone()).is_some() {
                    target.data.material_data.as_mut().unwrap()[write as usize] =
                        target.data.material_data.as_mut().unwrap()[read as usize].clone();
                }
            }
            {
                write += 1.0;
                write
            };
            {
                read += 1.0;
                read
            };
        }
    }
    target.data.instance_count = write;
}

// Source: upstream/packages/sprite/src/quadBatch.ts:117 (sha256:6a836f4fd41c109a36fa22e3ac6379a30bb0802b6b04882e59e7e78fd095b91f)
pub fn compute_quad_batch_local_bounds_rectangle(out: &mut Rectangle, source: &QuadBatch) -> () {
    let atlas = (source.data.atlas).clone();
    let instance_count = source.data.instance_count;
    let transform_type = (source.data.transform_type).clone();
    if ((atlas).is_none()) || (instance_count == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return;
    }
    let num_regions = (atlas.as_ref().unwrap().regions.len() as f64);
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    if (transform_type == "vector2") {
        {
            let mut i = 0.0_f64;
            while (i < instance_count) {
                let id = (source.data.ids[i as usize] as f64);
                if (id < 0.0_f64) || (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = atlas.as_ref().unwrap().regions[id as usize].clone();
                if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let dx = (source.data.transforms[(i * QUAD_VECTOR2_STRIDE) as usize] as f64);
                let dy =
                    (source.data.transforms[((i * QUAD_VECTOR2_STRIDE) + 1.0_f64) as usize] as f64);
                if (dx < min_x) {
                    min_x = dx;
                }
                if (dy < min_y) {
                    min_y = dy;
                }
                let rx = (dx + region.width);
                let ry = (dy + region.height);
                if (rx > max_x) {
                    max_x = rx;
                }
                if (ry > max_y) {
                    max_y = ry;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    } else {
        {
            let mut i = 0.0_f64;
            while (i < instance_count) {
                let id = (source.data.ids[i as usize] as f64);
                if (id < 0.0_f64) || (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = atlas.as_ref().unwrap().regions[id as usize].clone();
                if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let o = (i * QUAD_MATRIX3_X2_STRIDE);
                let a = (source.data.transforms[o as usize] as f64);
                let b = (source.data.transforms[(o + 1.0_f64) as usize] as f64);
                let c = (source.data.transforms[(o + 2.0_f64) as usize] as f64);
                let d = (source.data.transforms[(o + 3.0_f64) as usize] as f64);
                let tx = (source.data.transforms[(o + 4.0_f64) as usize] as f64);
                let ty = (source.data.transforms[(o + 5.0_f64) as usize] as f64);
                let w = region.width;
                let h = region.height;
                let x0 = tx;
                let y0 = ty;
                let x1 = ((a * w) + tx);
                let y1 = ((b * w) + ty);
                let x2 = ((c * h) + tx);
                let y2 = ((d * h) + ty);
                let x3 = (((a * w) + (c * h)) + tx);
                let y3 = (((b * w) + (d * h)) + ty);
                let q_min_x = (((x0).min(x1)).min(x2)).min(x3);
                let q_min_y = (((y0).min(y1)).min(y2)).min(y3);
                let q_max_x = (((x0).max(x1)).max(x2)).max(x3);
                let q_max_y = (((y0).max(y1)).max(y2)).max(y3);
                if (q_min_x < min_x) {
                    min_x = q_min_x;
                }
                if (q_min_y < min_y) {
                    min_y = q_min_y;
                }
                if (q_max_x > max_x) {
                    max_x = q_max_x;
                }
                if (q_max_y > max_y) {
                    max_y = q_max_y;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    if (min_x == f64::INFINITY) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
    } else {
        out.x = min_x;
        out.y = min_y;
        out.width = (max_x - min_x);
        out.height = (max_y - min_y);
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:193 (sha256:1c800b4a3898d508262531dded33e2104e3aab48221910d04c9a6b3bb9337774)
pub fn create_quad_batch(obj: Option<QuadBatch>) -> QuadBatch {
    return create_display_object_generic(
        (quad_batch_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_quad_batch_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_quad_batch_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/sprite/src/quadBatch.ts:197 (sha256:d8f8cdb62e22227010d70dd56ef4cf8f0cdba6c401de0f4f4aa0ef0d108156df)
pub fn create_quad_batch_data(data: Option<FlightPartialRecord1>) -> QuadBatchData {
    return QuadBatchData {
        __flight_identity: std::sync::Arc::new(()),
        atlas: data.as_ref().and_then(|value| (value.atlas).clone()),
        ids: (data.as_ref().and_then(|value| (value.ids).clone())).unwrap_or(vec![
            0_u16;
            (0.0_f64)
                as usize
        ]),
        instance_count: (data.as_ref().and_then(|value| value.instance_count)).unwrap_or(0.0_f64),
        material_data: data
            .as_ref()
            .and_then(|value| (value.material_data).clone()),
        transforms: (data.as_ref().and_then(|value| (value.transforms).clone()))
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        transform_type: (data
            .as_ref()
            .and_then(|value| (value.transform_type).clone()))
        .unwrap_or("vector2".to_owned()),
    };
}

// Source: upstream/packages/sprite/src/quadBatch.ts:208 (sha256:7104396156c3615e82d045294620ea11a4d0a1775d8608efed4eae6c8f758c8a)
pub fn create_quad_batch_runtime() -> QuadBatchRuntime {
    let mut runtime = create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
    runtime.local_bounds_rectangle = None;
    runtime.instance_velocities = None;
    return runtime;
}

// Source: upstream/packages/sprite/src/quadBatch.ts:215 (sha256:a5e5bc8636d351898b9b43f2af798138b6a79864561fd3879b3ba9d82d7637e0)
pub fn create_quad_batch_signals() -> QuadBatchSignals {
    return QuadBatchSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_cleared: create_signal(),
        on_instance_appended: create_signal(),
        on_instance_removed: create_signal(),
    };
}

// Source: upstream/packages/sprite/src/quadBatch.ts:228 (sha256:3656710a8318a9f521d167f964ae723d7cfb0869fe97796ca8a60bd8e8b7ae93)
pub fn enable_quad_batch_signals(target: &mut QuadBatch) -> QuadBatchSignals {
    let mut s = {
        let __flight_source = &((*target).clone());
        QuadBatchWithSignals {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    };
    return {
        s[*QUAD_BATCH_SIGNALS_SLOT as usize]?? = create_quad_batch_signals();
        s[*QUAD_BATCH_SIGNALS_SLOT as usize]
    };
}

// Source: upstream/packages/sprite/src/quadBatch.ts:233 (sha256:3e0c4c6e4762344f55f6c7861d8effb1ade1f806174032a15189307e20ae0f72)
pub fn get_quad_batch_capacity(source: &QuadBatch) -> f64 {
    let stride = get_quad_batch_transform_stride((source.data.transform_type).clone());
    let transform_capacity = (__flight_js_to_i32(((source.data.transforms.len() as f64) / stride))
        | __flight_js_to_i32(0.0_f64)) as f64;
    return (source.data.ids.len() as f64).min(transform_capacity);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:244 (sha256:ae3bb672ab02dc0fc6d5343042b87f3e85c63bb1bbdfb8195f48e04810a5e505)
pub fn get_quad_batch_instance_id(source: &QuadBatch, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.instance_count) {
        return (-1.0_f64);
    }
    return (source.data.ids[index as usize] as f64);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:255 (sha256:d8a3f5c48ecdae2bd48c400406fe2974e632fe320e1926b8b14188b5d34b196d)
pub fn get_quad_batch_instance_transform(
    out: &mut Vector2Like,
    source: &QuadBatch,
    index: f64,
) -> bool {
    let instance_count = source.data.instance_count;
    let transform_type = (source.data.transform_type).clone();
    if (index < 0.0_f64) || (index >= instance_count) {
        return false;
    }
    if (transform_type == "vector2") {
        let o = (index * QUAD_VECTOR2_STRIDE);
        out.x = (source.data.transforms[o as usize] as f64);
        out.y = (source.data.transforms[(o + 1.0_f64) as usize] as f64);
    } else {
        let o = (index * QUAD_MATRIX3_X2_STRIDE);
        out.x = (source.data.transforms[(o + 4.0_f64) as usize] as f64);
        out.y = (source.data.transforms[(o + 5.0_f64) as usize] as f64);
    }
    return true;
}

// Source: upstream/packages/sprite/src/quadBatch.ts:270 (sha256:dbac87f02408f85738620de8ee285cd420835b1a77137ac72dfe43723b82a129)
pub fn get_quad_batch_runtime(source: &QuadBatch) -> QuadBatchRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:275 (sha256:5ac7812183ca0fd3bee59ca974dbd2f7661a1f35484e0140c3c184ed4eb4f506)
pub fn get_quad_batch_signals(source: &QuadBatch) -> Option<QuadBatchSignals> {
    return Some(
        {
            let __flight_source = &((*source).clone());
            QuadBatchWithSignals {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            }
        }[*QUAD_BATCH_SIGNALS_SLOT as usize]
            .clone(),
    );
}

// Source: upstream/packages/sprite/src/quadBatch.ts:279 (sha256:1688ee6fb6c74e4d710d23d8e3095921c2f9e15fc653e6593270fb133bcbda29)
pub fn get_quad_batch_transform_stride(transform_type: QuadTransformType) -> f64 {
    return QUAD_TRANSFORM_STRIDE[transform_type as usize].clone();
}

// Source: upstream/packages/sprite/src/quadBatch.ts:283 (sha256:a1ba55edb310da71c25c0e937e014ce71ac40b167d6a2853b8142d2ada8be270)
pub fn hit_test_quad_batch_point(source: &QuadBatch, point: &Vector2Like) -> f64 {
    return hit_test_quad_batch_point_xy(source, point.x, point.y);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:296 (sha256:5e4a950c168b8aae381db0507c74d99f74d70dd7d92788e20fa9c04fc83e741a)
pub fn hit_test_quad_batch_point_exact(source: &QuadBatch, point: &Vector2Like) -> f64 {
    return hit_test_quad_batch_point_exact_xy(source, point.x, point.y);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:303 (sha256:574ae6d546af345c8fd9d566b7c6eb6e36b03e3d342fd506f5d5f9ef3372143a)
pub fn hit_test_quad_batch_point_exact_xy(source: &QuadBatch, x: f64, y: f64) -> f64 {
    let atlas = (source.data.atlas).clone();
    let instance_count = source.data.instance_count;
    let transform_type = (source.data.transform_type).clone();
    if ((atlas).is_none()) || (instance_count == 0.0_f64) {
        return (-1.0_f64);
    }
    let num_regions = (atlas.as_ref().unwrap().regions.len() as f64);
    if (transform_type == "vector2") {
        {
            let mut i = 0.0_f64;
            while (i < instance_count) {
                let id = (source.data.ids[i as usize] as f64);
                if (id < 0.0_f64) || (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = atlas.as_ref().unwrap().regions[id as usize].clone();
                let dx = (source.data.transforms[(i * QUAD_VECTOR2_STRIDE) as usize] as f64);
                let dy =
                    (source.data.transforms[((i * QUAD_VECTOR2_STRIDE) + 1.0_f64) as usize] as f64);
                if (((x >= dx) && (x < (dx + region.width))) && (y >= dy))
                    && (y < (dy + region.height))
                {
                    return i;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    } else {
        {
            let mut i = 0.0_f64;
            while (i < instance_count) {
                let id = (source.data.ids[i as usize] as f64);
                if (id < 0.0_f64) || (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = atlas.as_ref().unwrap().regions[id as usize].clone();
                if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let o = (i * QUAD_MATRIX3_X2_STRIDE);
                let a = (source.data.transforms[o as usize] as f64);
                let b = (source.data.transforms[(o + 1.0_f64) as usize] as f64);
                let c = (source.data.transforms[(o + 2.0_f64) as usize] as f64);
                let d = (source.data.transforms[(o + 3.0_f64) as usize] as f64);
                let tx = (source.data.transforms[(o + 4.0_f64) as usize] as f64);
                let ty = (source.data.transforms[(o + 5.0_f64) as usize] as f64);
                let w = region.width;
                let h = region.height;
                let x0 = tx;
                let y0 = ty;
                let x1 = ((a * w) + tx);
                let y1 = ((b * w) + ty);
                let x2 = (((a * w) + (c * h)) + tx);
                let y2 = (((b * w) + (d * h)) + ty);
                let x3 = ((c * h) + tx);
                let y3 = ((d * h) + ty);
                if (((cross_sign((x0).clone(), (y0).clone(), x1, y1, x, y))
                    && (cross_sign(x1, y1, x2, y2, x, y)))
                    && (cross_sign(x2, y2, x3, y3, x, y)))
                    && (cross_sign(x3, y3, (x0).clone(), (y0).clone(), x, y))
                {
                    return i;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:356 (sha256:6f314fcecf96d87e37ea8c8271437573facfae5f195453e291f1c9b7def5b4a7)
pub fn hit_test_quad_batch_point_xy(source: &QuadBatch, x: f64, y: f64) -> f64 {
    let atlas = (source.data.atlas).clone();
    let instance_count = source.data.instance_count;
    let transform_type = (source.data.transform_type).clone();
    if ((atlas).is_none()) || (instance_count == 0.0_f64) {
        return (-1.0_f64);
    }
    let num_regions = (atlas.as_ref().unwrap().regions.len() as f64);
    if (transform_type == "vector2") {
        {
            let mut i = 0.0_f64;
            while (i < instance_count) {
                let id = (source.data.ids[i as usize] as f64);
                if (id < 0.0_f64) || (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = atlas.as_ref().unwrap().regions[id as usize].clone();
                let dx = (source.data.transforms[(i * QUAD_VECTOR2_STRIDE) as usize] as f64);
                let dy =
                    (source.data.transforms[((i * QUAD_VECTOR2_STRIDE) + 1.0_f64) as usize] as f64);
                if (((x >= dx) && (x < (dx + region.width))) && (y >= dy))
                    && (y < (dy + region.height))
                {
                    return i;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    } else {
        {
            let mut i = 0.0_f64;
            while (i < instance_count) {
                let id = (source.data.ids[i as usize] as f64);
                if (id < 0.0_f64) || (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = atlas.as_ref().unwrap().regions[id as usize].clone();
                if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let o = (i * QUAD_MATRIX3_X2_STRIDE);
                let a = (source.data.transforms[o as usize] as f64);
                let b = (source.data.transforms[(o + 1.0_f64) as usize] as f64);
                let c = (source.data.transforms[(o + 2.0_f64) as usize] as f64);
                let d = (source.data.transforms[(o + 3.0_f64) as usize] as f64);
                let tx = (source.data.transforms[(o + 4.0_f64) as usize] as f64);
                let ty = (source.data.transforms[(o + 5.0_f64) as usize] as f64);
                let w = region.width;
                let h = region.height;
                let x0 = tx;
                let y0 = ty;
                let x1 = ((a * w) + tx);
                let y1 = ((b * w) + ty);
                let x2 = ((c * h) + tx);
                let y2 = ((d * h) + ty);
                let x3 = (((a * w) + (c * h)) + tx);
                let y3 = (((b * w) + (d * h)) + ty);
                let min_x = (((x0).min(x1)).min(x2)).min(x3);
                let min_y = (((y0).min(y1)).min(y2)).min(y3);
                let max_x = (((x0).max(x1)).max(x2)).max(x3);
                let max_y = (((y0).max(y1)).max(y2)).max(y3);
                if (((x >= min_x) && (x < max_x)) && (y >= min_y)) && (y < max_y) {
                    return i;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:408 (sha256:4e915fe5ed23b14a5bca554624749791bea46eb36de7a3f9857bb9925fc5f63d)
pub fn iterate_quad_batch_instances(
    source: &QuadBatch,
    visitor: &mut impl FnMut(f64, f64, Vec<f32>) -> (),
) -> () {
    let instance_count = source.data.instance_count;
    let transform_type = (source.data.transform_type).clone();
    let stride = get_quad_batch_transform_stride((transform_type).clone());
    {
        let mut i = 0.0_f64;
        while (i < instance_count) {
            visitor(
                i,
                (source.data.ids[i as usize] as f64),
                source.data.transforms[(i * stride) as usize..((i * stride) + stride) as usize]
                    .to_vec(),
            );
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:425 (sha256:010ec5daf281df50acca9b7dea5e58f5834375093e7e3ec02f2011a8fea556cd)
pub fn remove_quad_batch_instance(target: &mut QuadBatch, index: f64) -> () {
    let last = (target.data.instance_count - 1.0_f64);
    if (index < 0.0_f64) || (index > last) {
        return;
    }
    let swap_source = if (index < last) { last } else { (-1.0_f64) };
    if (index < last) {
        target.data.ids[index as usize] = (target.data.ids[last as usize] as f64) as u16;
        if ((target.data.transform_type).clone() == "vector2") {
            let dst = (index * QUAD_VECTOR2_STRIDE);
            let src = (last * QUAD_VECTOR2_STRIDE);
            target.data.transforms[dst as usize] =
                (target.data.transforms[src as usize] as f64) as f32;
            target.data.transforms[(dst + 1.0_f64) as usize] =
                (target.data.transforms[(src + 1.0_f64) as usize] as f64) as f32;
        } else {
            let dst = (index * QUAD_MATRIX3_X2_STRIDE);
            let src = (last * QUAD_MATRIX3_X2_STRIDE);
            {
                let mut k = 0.0_f64;
                while (k < QUAD_MATRIX3_X2_STRIDE) {
                    target.data.transforms[(dst + k) as usize] =
                        (target.data.transforms[(src + k) as usize] as f64) as f32;
                    {
                        k += 1.0;
                        k
                    };
                }
            }
        }
        if ((target.data.material_data).clone()).is_some() {
            target.data.material_data.as_mut().unwrap()[index as usize] =
                target.data.material_data.as_mut().unwrap()[last as usize].clone();
        }
    }
    target.data.instance_count = last;
    let signals = get_quad_batch_signals(target);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_instance_removed.emit).clone())(index, swap_source);
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:451 (sha256:2bcb793531aee5cf26280877b4a6850fb06ebe4bd96ab6334c20899b41c3a711)
pub fn reserve_quad_batch(target: &mut QuadBatch, capacity: f64) -> () {
    let current_capacity = get_quad_batch_capacity(target);
    if (current_capacity >= capacity) {
        return;
    }
    target.data.ids = reserve_uint16_array(&target.data.ids, capacity);
    target.data.transforms = reserve_float32_array(
        &target.data.transforms,
        (capacity * get_quad_batch_transform_stride((target.data.transform_type).clone())),
    );
}

// Source: upstream/packages/sprite/src/quadBatch.ts:459 (sha256:e1a222eb89123c495a0d1a6d63b4b4692809bdcb149a87c49d4410177d7d0aa2)
pub fn resize_quad_batch(target: &mut QuadBatch, instance_count: f64) -> () {
    let old_instance_count = target.data.instance_count;
    target.data.instance_count = instance_count;
    if (old_instance_count >= instance_count) {
        return;
    }
    let capacity = get_quad_batch_capacity(target);
    if (capacity < instance_count) {
        let new_capacity = (instance_count).max((capacity * 2.0_f64));
        reserve_quad_batch(target, new_capacity);
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:476 (sha256:12fe99dbe97e0ed4c5d706719bdbd77126c0eb4094a36ac7881782b49b195353)
pub fn set_quad_batch_instance(target: &mut QuadBatch, index: f64, id: f64, x: f64, y: f64) -> () {
    if (index < 0.0_f64) || (index >= target.data.instance_count) {
        return;
    }
    target.data.ids[index as usize] = (id) as u16;
    let o = (index * QUAD_VECTOR2_STRIDE);
    target.data.transforms[o as usize] = (x) as f32;
    target.data.transforms[(o + 1.0_f64) as usize] = (y) as f32;
}

// Source: upstream/packages/sprite/src/quadBatch.ts:491 (sha256:5c83e050cd0d879246602b637ce93717b26f5694d2e22d6734ac8a25fd7a2d82)
pub fn set_quad_batch_instance_matrix(
    target: &mut QuadBatch,
    index: f64,
    id: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    tx: f64,
    ty: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.instance_count) {
        return;
    }
    target.data.ids[index as usize] = (id) as u16;
    let o = (index * QUAD_MATRIX3_X2_STRIDE);
    target.data.transforms[o as usize] = (a) as f32;
    target.data.transforms[(o + 1.0_f64) as usize] = (b) as f32;
    target.data.transforms[(o + 2.0_f64) as usize] = (c) as f32;
    target.data.transforms[(o + 3.0_f64) as usize] = (d) as f32;
    target.data.transforms[(o + 4.0_f64) as usize] = (tx) as f32;
    target.data.transforms[(o + 5.0_f64) as usize] = (ty) as f32;
}

// Source: upstream/packages/sprite/src/quadBatch.ts:520 (sha256:7c63002d3396429f2ca4b4921ff71af8c4dbfab26ea683881297449b82770125)
pub fn set_quad_batch_instance_range(
    target: &mut QuadBatch,
    start_index: f64,
    count: f64,
    source: &Vec<f32>,
) -> () {
    if ((start_index < 0.0_f64) || (count <= 0.0_f64))
        || ((start_index + count) > target.data.instance_count)
    {
        return;
    }
    let stride = get_quad_batch_transform_stride((target.data.transform_type).clone());
    let dst = (start_index * stride);
    let len = (count * stride);
    {
        let mut k = 0.0_f64;
        while (k < len) {
            target.data.transforms[(dst + k) as usize] = (source[k as usize] as f64) as f32;
            {
                k += 1.0;
                k
            };
        }
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:534 (sha256:cf2317d720ecdcf6af7e5995614d5cbe7b680b29f8867a301f9736af7430179e)
pub fn set_quad_batch_local_bounds_rectangle(target: &QuadBatch, rect: &Rectangle) -> () {
    let mut runtime = get_display_object_runtime(target);
    if ((runtime.local_bounds_rectangle).clone()).is_none() {
        runtime.local_bounds_rectangle = Some(create_rectangle(None, None, None, None));
    }
    copy_rectangle(runtime.local_bounds_rectangle.as_mut().unwrap(), &{
        let __flight_source = &(rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            height: __flight_source.height,
            width: __flight_source.width,
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    invalidate_node_local_bounds(target);
}

// Source: upstream/packages/sprite/src/quadBatch.ts:547 (sha256:51dd81e839460522be7d0c0185b498c4c55c18d53290acca203e0387b869c946)
pub fn set_quad_batch_transform_type(target: &mut QuadBatch, new_type: QuadTransformType) -> () {
    if ((target.data.transform_type).clone() == new_type) {
        return;
    }
    let count = target.data.instance_count;
    if (new_type == "matrix3x2") {
        let mut new_transforms: Vec<f32> = vec![
            0.0_f32;
            (((target.data.transforms.len() as f64) / QUAD_VECTOR2_STRIDE).max(count)
                * QUAD_MATRIX3_X2_STRIDE) as usize
        ];
        {
            let mut i = (count - 1.0_f64);
            while (i >= 0.0_f64) {
                let src = (i * QUAD_VECTOR2_STRIDE);
                let dst = (i * QUAD_MATRIX3_X2_STRIDE);
                let x = (target.data.transforms[src as usize] as f64);
                let y = (target.data.transforms[(src + 1.0_f64) as usize] as f64);
                new_transforms[dst as usize] = (1.0_f64) as f32;
                new_transforms[(dst + 1.0_f64) as usize] = (0.0_f64) as f32;
                new_transforms[(dst + 2.0_f64) as usize] = (0.0_f64) as f32;
                new_transforms[(dst + 3.0_f64) as usize] = (1.0_f64) as f32;
                new_transforms[(dst + 4.0_f64) as usize] = ((x).clone()) as f32;
                new_transforms[(dst + 5.0_f64) as usize] = ((y).clone()) as f32;
                {
                    i -= 1.0;
                    i
                };
            }
        }
        target.data.transforms = (new_transforms).clone();
    } else {
        {
            let mut i = 0.0_f64;
            while (i < count) {
                let src = (i * QUAD_MATRIX3_X2_STRIDE);
                let dst = (i * QUAD_VECTOR2_STRIDE);
                target.data.transforms[dst as usize] =
                    (target.data.transforms[(src + 4.0_f64) as usize] as f64) as f32;
                target.data.transforms[(dst + 1.0_f64) as usize] =
                    (target.data.transforms[(src + 5.0_f64) as usize] as f64) as f32;
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    target.data.transform_type = (new_type).clone();
}

// Source: upstream/packages/sprite/src/quadBatch.ts:581 (sha256:a107c7d7b5b8fb80ae7ff5e80fd9088d840b7655dcea90422f65cc256f7d3279)
static DEFAULT_METHODS: std::sync::LazyLock<FlightPartialRecord2> =
    std::sync::LazyLock::new(|| FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                copy_local_bounds_rectangle(&mut __flight_argument_0, &__flight_argument_1)
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
        instance_velocities: None,
    });

// Source: upstream/packages/sprite/src/quadBatch.ts:585 (sha256:238a0f0f99917ec7b8730f572110369690cb68566fc84116df3553cdfcf6783a)
static QUAD_BATCH_SIGNALS_SLOT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/sprite/src/quadBatch.ts:587 (sha256:d16672a42737f57500938890eb91408d7e0a5c60205b13d1b8a7e268ad5b846b)
#[derive(Clone, Default)]
struct QuadBatchWithSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for QuadBatchWithSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/sprite/src/quadBatch.ts:591 (sha256:b8dcad1528fa3e80019397998b4a14d1787977e992f28e2314611a54bc6d7659)
#[derive(Clone, Default)]
struct QuadTransformStride {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vector2: f64,
    pub matrix3x2: f64,
}
impl PartialEq for QuadTransformStride {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

static QUAD_TRANSFORM_STRIDE: std::sync::LazyLock<QuadTransformStride> =
    std::sync::LazyLock::new(|| QuadTransformStride {
        __flight_identity: std::sync::Arc::new(()),
        vector2: 2.0_f64,
        matrix3x2: 6.0_f64,
    });

// Source: upstream/packages/sprite/src/quadBatch.ts:597 (sha256:3930812f9a6af5c8e54a67d87e38cceaed2ad9d1da940fe279ed936d8bb366da)
fn cross_sign(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> bool {
    return ((((bx - ax) * (py - ay)) - ((by - ay) * (px - ax))) >= 0.0_f64);
}
