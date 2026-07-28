// @generated from upstream/packages/particleemitter/src/particleEmitter.ts; do not edit.
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
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, PARTICLE_EMITTER_KIND as particle_emitter_kind_constant, ParticleEmitter,
    ParticleEmitterData, ParticleEmitterRuntime, Rectangle, Stage, TextureAtlas, Vector2Like,
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

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alphas: Option<Vec<f32>>,
    pub atlas: Option<TextureAtlas>,
    pub colors: Option<Vec<f32>>,
    pub ids: Option<Vec<u16>>,
    pub particle_count: Option<f64>,
    pub positions_z: Option<Vec<f32>>,
    pub transforms: Option<Vec<f32>>,
    pub velocities: Option<Vec<f32>>,
    pub world_space: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:22 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:23 (sha256:a68bb9efd032a89d93f67112fefca6fd2cfff5d832357b4e7a07ee6a442870a2)
const PARTICLE_COLOR_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:24 (sha256:eb764aa733e7360185c6e3e6b75a3ea1c9dd0be629332a9f56e32be51bdeedb3)
const PARTICLE_VELOCITY_STRIDE: f64 = 2.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:27 (sha256:71d79f0d93e5989dbbf39feb93b7f7dcf8d21f62a250477d3e0611c854b652b7)
pub const PARTICLE_EMITTER_DELETED_ID: f64 = 65535.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:29 (sha256:13efde0037b4577c1339b505ad1bcbb838ba27bd6e1fdcc910238f45f78fd854)
fn copy_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let runtime = get_display_object_runtime(&source);
    if ((runtime.local_bounds_rectangle).clone()).is_some() {
        copy_rectangle(out, runtime.local_bounds_rectangle.as_ref().unwrap());
    }
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:39 (sha256:0e5adde2fbbcfcc985e8d283c4189c0f82e4fe53d1e65a33a8574088135b7362)
pub fn append_particle_emitter_particle(
    target: &mut ParticleEmitter,
    id: f64,
    x: f64,
    y: f64,
    rotation: f64,
    scale: f64,
) -> f64 {
    let index = target.data.particle_count;
    let needed = (index + 1.0_f64);
    if (get_particle_emitter_capacity(target) < needed) {
        let new_capacity = (needed).max(if (target.data.particle_count * 2.0_f64) != 0.0_f64 {
            (target.data.particle_count * 2.0_f64)
        } else {
            8.0_f64
        });
        reserve_particle_emitter(target, new_capacity);
    }
    target.data.particle_count = needed;
    target.data.ids[index as usize] = (id) as u16;
    let tt = (index * PARTICLE_TRANSFORM_STRIDE);
    target.data.transforms[tt as usize] = (x) as f32;
    target.data.transforms[(tt + 1.0_f64) as usize] = (y) as f32;
    target.data.transforms[(tt + 2.0_f64) as usize] = (rotation) as f32;
    target.data.transforms[(tt + 3.0_f64) as usize] = (scale) as f32;
    target.data.alphas[index as usize] = (1.0_f64) as f32;
    let ct = (index * PARTICLE_COLOR_STRIDE);
    target.data.colors[ct as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 1.0_f64) as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 2.0_f64) as usize] = (1.0_f64) as f32;
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (0.0_f64) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (0.0_f64) as f32;
    target.data.positions_z[index as usize] = (0.0_f64) as f32;
    return index;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:73 (sha256:729287f8b846f17e6b473e6ca9befe53befc7ae75094780aae6e3413b2f858a1)
pub fn clear_particle_emitter(target: &mut ParticleEmitter) -> () {
    target.data.particle_count = 0.0_f64;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:82 (sha256:48c5a8f462081622640f21b6562a037e8548e24ef57f3f240e6cd8ae4fa12306)
pub fn clone_particle_emitter(source: &ParticleEmitter) -> ParticleEmitter {
    return create_particle_emitter(Some(ParticleEmitter {
        __flight_identity: std::sync::Arc::new(()),
        data: ParticleEmitterData {
            __flight_identity: std::sync::Arc::new(()),
            alphas: ((source.data.alphas).clone()).clone(),
            atlas: (source.data.atlas).clone(),
            colors: ((source.data.colors).clone()).clone(),
            ids: ((source.data.ids).clone()).clone(),
            particle_count: source.data.particle_count,
            positions_z: ((source.data.positions_z).clone()).clone(),
            transforms: ((source.data.transforms).clone()).clone(),
            velocities: ((source.data.velocities).clone()).clone(),
            world_space: source.data.world_space,
        },
    }));
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:107 (sha256:ef4ecddad3f69400a7788caa10bd92a6f3b4233817ded18c16aec13ca1e1157e)
pub fn compact_particle_emitter(target: &mut ParticleEmitter) -> () {
    if (target.data.particle_count == 0.0_f64) {
        return;
    }
    let mut write = 0.0_f64;
    {
        let mut read = 0.0_f64;
        while (read < target.data.particle_count) {
            if ((target.data.ids[read as usize] as f64) == PARTICLE_EMITTER_DELETED_ID) {
                {
                    read += 1.0;
                    read
                };
                continue;
            }
            if (write != read) {
                target.data.ids[write as usize] = (target.data.ids[read as usize] as f64) as u16;
                let tt = (write * PARTICLE_TRANSFORM_STRIDE);
                let tts = (read * PARTICLE_TRANSFORM_STRIDE);
                target.data.transforms[tt as usize] =
                    (target.data.transforms[tts as usize] as f64) as f32;
                target.data.transforms[(tt + 1.0_f64) as usize] =
                    (target.data.transforms[(tts + 1.0_f64) as usize] as f64) as f32;
                target.data.transforms[(tt + 2.0_f64) as usize] =
                    (target.data.transforms[(tts + 2.0_f64) as usize] as f64) as f32;
                target.data.transforms[(tt + 3.0_f64) as usize] =
                    (target.data.transforms[(tts + 3.0_f64) as usize] as f64) as f32;
                target.data.alphas[write as usize] =
                    (target.data.alphas[read as usize] as f64) as f32;
                let ct = (write * PARTICLE_COLOR_STRIDE);
                let cts = (read * PARTICLE_COLOR_STRIDE);
                target.data.colors[ct as usize] = (target.data.colors[cts as usize] as f64) as f32;
                target.data.colors[(ct + 1.0_f64) as usize] =
                    (target.data.colors[(cts + 1.0_f64) as usize] as f64) as f32;
                target.data.colors[(ct + 2.0_f64) as usize] =
                    (target.data.colors[(cts + 2.0_f64) as usize] as f64) as f32;
                let vt = (write * PARTICLE_VELOCITY_STRIDE);
                let vts = (read * PARTICLE_VELOCITY_STRIDE);
                target.data.velocities[vt as usize] =
                    (target.data.velocities[vts as usize] as f64) as f32;
                target.data.velocities[(vt + 1.0_f64) as usize] =
                    (target.data.velocities[(vts + 1.0_f64) as usize] as f64) as f32;
                target.data.positions_z[write as usize] =
                    (target.data.positions_z[read as usize] as f64) as f32;
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
    target.data.particle_count = write;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:138 (sha256:eed64ce4fdc36ab0dc7770687345e1fac6d151fcd12ab923aa3590c43bde5462)
pub fn compute_particle_emitter_local_bounds_rectangle(
    out: &mut Rectangle,
    source: &ParticleEmitter,
) -> () {
    let atlas = (source.data.atlas).clone();
    let particle_count = source.data.particle_count;
    if ((atlas).is_none()) || (particle_count == 0.0_f64) {
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
    {
        let mut i = 0.0_f64;
        while (i < particle_count) {
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
            let tt = (i * PARTICLE_TRANSFORM_STRIDE);
            let px = (source.data.transforms[tt as usize] as f64);
            let py = (source.data.transforms[(tt + 1.0_f64) as usize] as f64);
            let rotation = (source.data.transforms[(tt + 2.0_f64) as usize] as f64);
            let scale = (source.data.transforms[(tt + 3.0_f64) as usize] as f64);
            let cos_r = ((rotation).cos() * scale);
            let sin_r = ((rotation).sin() * scale);
            let w = region.width;
            let h = region.height;
            let x0 = px;
            let y0 = py;
            let x1 = ((cos_r * w) + px);
            let y1 = ((sin_r * w) + py);
            let x2 = (((cos_r * w) - (sin_r * h)) + px);
            let y2 = (((sin_r * w) + (cos_r * h)) + py);
            let x3 = (((-sin_r) * h) + px);
            let y3 = ((cos_r * h) + py);
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

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:198 (sha256:ee49d2e98c5b645c6dd723f40f805537f25d473924f4f4624740b26c70d7b012)
pub fn create_particle_emitter(obj: Option<ParticleEmitter>) -> ParticleEmitter {
    return create_display_object_generic(
        (particle_emitter_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_particle_emitter_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_particle_emitter_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:207 (sha256:45f7f66c56c708593de9490b6aa1113bc1bdf2a0dbcc4e19357d6b7b403d3829)
pub fn create_particle_emitter_data(data: Option<FlightPartialRecord1>) -> ParticleEmitterData {
    return ParticleEmitterData {
        __flight_identity: std::sync::Arc::new(()),
        alphas: (data.as_ref().and_then(|value| (value.alphas).clone()))
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        atlas: data.as_ref().and_then(|value| (value.atlas).clone()),
        colors: (data.as_ref().and_then(|value| (value.colors).clone()))
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        ids: (data.as_ref().and_then(|value| (value.ids).clone())).unwrap_or(vec![
            0_u16;
            (0.0_f64)
                as usize
        ]),
        particle_count: (data.as_ref().and_then(|value| value.particle_count)).unwrap_or(0.0_f64),
        positions_z: (data.as_ref().and_then(|value| (value.positions_z).clone()))
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        transforms: (data.as_ref().and_then(|value| (value.transforms).clone()))
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        velocities: (data.as_ref().and_then(|value| (value.velocities).clone()))
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        world_space: (data.as_ref().and_then(|value| value.world_space)).unwrap_or(false),
    };
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:221 (sha256:987b0c6ff40fc28b62281a970bc58d8f8862117c42537f3031c8f9e94e047c35)
pub fn create_particle_emitter_runtime() -> ParticleEmitterRuntime {
    let mut runtime = create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
    runtime.local_bounds_rectangle = None;
    return runtime;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:227 (sha256:d8d451c8d17568725cb656523093484ae2fec348a0aceb5c10fdd36876263e72)
pub fn get_particle_emitter_capacity(source: &ParticleEmitter) -> f64 {
    let transform_capacity =
        (__flight_js_to_i32(((source.data.transforms.len() as f64) / PARTICLE_TRANSFORM_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64;
    return ((source.data.ids.len() as f64).min((source.data.alphas.len() as f64)))
        .min(transform_capacity);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:237 (sha256:a98ad2fcc3f015497b346664e0001be1ae1cae16fb208464c269976e0e696ee3)
pub fn get_particle_emitter_particle_alpha(source: &ParticleEmitter, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.alphas[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:246 (sha256:1a9c7fd18a5403f11bf48f1f3464523225d3e57d617e45a20886008405b34192)
pub fn get_particle_emitter_particle_id(source: &ParticleEmitter, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.ids[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:255 (sha256:a695a2ff63415093c2947163f9803ef25a2fcc75de4b42104a6c09c7f3441511)
pub fn get_particle_emitter_particle_velocity(
    out: &mut Vector2Like,
    source: &ParticleEmitter,
    index: f64,
) -> bool {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return false;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    out.x = (source.data.velocities[vt as usize] as f64);
    out.y = (source.data.velocities[(vt + 1.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:267 (sha256:bfbf41a6ed0a04605b33e12c5565c26e88bdfec2fe76102691dc7eb865ecc66f)
pub fn get_particle_emitter_runtime(source: &ParticleEmitter) -> ParticleEmitterRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:276 (sha256:c7f267eb197bdb2f30c11fc9831fd9f3a54aef58e648688efe13b1cbdef877d4)
pub fn remove_particle_emitter_particle(target: &mut ParticleEmitter, index: f64) -> () {
    let last = (target.data.particle_count - 1.0_f64);
    if (index < 0.0_f64) || (index > last) {
        return;
    }
    if (index < last) {
        target.data.ids[index as usize] = (target.data.ids[last as usize] as f64) as u16;
        let tt = (index * PARTICLE_TRANSFORM_STRIDE);
        let tts = (last * PARTICLE_TRANSFORM_STRIDE);
        target.data.transforms[tt as usize] = (target.data.transforms[tts as usize] as f64) as f32;
        target.data.transforms[(tt + 1.0_f64) as usize] =
            (target.data.transforms[(tts + 1.0_f64) as usize] as f64) as f32;
        target.data.transforms[(tt + 2.0_f64) as usize] =
            (target.data.transforms[(tts + 2.0_f64) as usize] as f64) as f32;
        target.data.transforms[(tt + 3.0_f64) as usize] =
            (target.data.transforms[(tts + 3.0_f64) as usize] as f64) as f32;
        target.data.alphas[index as usize] = (target.data.alphas[last as usize] as f64) as f32;
        let ct = (index * PARTICLE_COLOR_STRIDE);
        let cts = (last * PARTICLE_COLOR_STRIDE);
        target.data.colors[ct as usize] = (target.data.colors[cts as usize] as f64) as f32;
        target.data.colors[(ct + 1.0_f64) as usize] =
            (target.data.colors[(cts + 1.0_f64) as usize] as f64) as f32;
        target.data.colors[(ct + 2.0_f64) as usize] =
            (target.data.colors[(cts + 2.0_f64) as usize] as f64) as f32;
        let vt = (index * PARTICLE_VELOCITY_STRIDE);
        let vts = (last * PARTICLE_VELOCITY_STRIDE);
        target.data.velocities[vt as usize] = (target.data.velocities[vts as usize] as f64) as f32;
        target.data.velocities[(vt + 1.0_f64) as usize] =
            (target.data.velocities[(vts + 1.0_f64) as usize] as f64) as f32;
        target.data.positions_z[index as usize] =
            (target.data.positions_z[last as usize] as f64) as f32;
    }
    target.data.particle_count = last;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:304 (sha256:10ee6a056a777ebc21d5fc37bcb16af22758b14d8ac7112495d1a9168d9c4b14)
pub fn reserve_particle_emitter(target: &mut ParticleEmitter, capacity: f64) -> () {
    if (get_particle_emitter_capacity(target) >= capacity) {
        return;
    }
    target.data.alphas = reserve_float32_array(&target.data.alphas, capacity);
    target.data.colors = reserve_float32_array(&target.data.colors, (capacity * 3.0_f64));
    target.data.ids = reserve_uint16_array(&target.data.ids, capacity);
    target.data.positions_z = reserve_float32_array(&target.data.positions_z, capacity);
    target.data.transforms = reserve_float32_array(
        &target.data.transforms,
        (capacity * PARTICLE_TRANSFORM_STRIDE),
    );
    target.data.velocities = reserve_float32_array(&target.data.velocities, (capacity * 2.0_f64));
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:315 (sha256:a84e0bd362b67906947d1ac9c72e84c82cd3d564570be76cd30b01cc609399ce)
pub fn set_particle_emitter_local_bounds_rectangle(
    target: &ParticleEmitter,
    rect: &Rectangle,
) -> () {
    let mut runtime = get_display_object_runtime(target);
    if ((runtime.local_bounds_rectangle).clone()).is_none() {
        runtime.local_bounds_rectangle = Some(create_rectangle(None, None, None, None));
    }
    copy_rectangle(runtime.local_bounds_rectangle.as_mut().unwrap(), rect);
    invalidate_node_local_bounds(target);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:326 (sha256:dc42b2d3124dd3d767a4598c78b4e0c3bc0edbcc44146a62c4b8270d75035349)
pub fn set_particle_emitter_particle(
    target: &mut ParticleEmitter,
    index: f64,
    id: f64,
    x: f64,
    y: f64,
    rotation: f64,
    scale: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    target.data.ids[index as usize] = (id) as u16;
    let tt = (index * PARTICLE_TRANSFORM_STRIDE);
    target.data.transforms[tt as usize] = (x) as f32;
    target.data.transforms[(tt + 1.0_f64) as usize] = (y) as f32;
    target.data.transforms[(tt + 2.0_f64) as usize] = (rotation) as f32;
    target.data.transforms[(tt + 3.0_f64) as usize] = (scale) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:348 (sha256:b400873f25ea0b24ff0de4f98294542107d056bc78d23d43c4fca5734cef9d8a)
pub fn set_particle_emitter_particle_alpha(
    target: &mut ParticleEmitter,
    index: f64,
    alpha: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    target.data.alphas[index as usize] = (alpha) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:356 (sha256:c194caf80bcb9bbc3968e2de0aec93d833fe551bc145093ba444ce04e656d3a4)
pub fn set_particle_emitter_particle_color(
    target: &mut ParticleEmitter,
    index: f64,
    r: f64,
    g: f64,
    b: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    let ct = (index * PARTICLE_COLOR_STRIDE);
    target.data.colors[ct as usize] = (r) as f32;
    target.data.colors[(ct + 1.0_f64) as usize] = (g) as f32;
    target.data.colors[(ct + 2.0_f64) as usize] = (b) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:373 (sha256:c6c07678c60877eff9d8fcc1a9df9b713ef75ed8f00a2a3351b4ee7b27549350)
pub fn set_particle_emitter_particle_velocity(
    target: &mut ParticleEmitter,
    index: f64,
    vx: f64,
    vy: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (vx) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (vy) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:385 (sha256:83e00828dd4cdc7bc4c2516d6ca0246a8719205d375d518c416dac38ad3af9c3)
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
    });
