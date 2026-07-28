// @generated from upstream/packages/particleemitter/src/particleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_particle_emitter_data;
use flighthq_geometry::{reserve_float32_array, reserve_uint16_array};
use flighthq_scene::{create_scene_node, get_scene_node_runtime};
use flighthq_types::{
    AabbLike, Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InteractionSignals, Kind, Material, MaterialData, Matrix, Node, NodeData, NodeInteractionState,
    NodeSignals, NodeTraitsKey, PARTICLE_EMITTER3_D_KIND as particle_emitter3_d_kind_constant,
    ParticleEmitter3D, ParticleEmitter3DRuntime, ParticleEmitterData, Quaternion, Rectangle, Stage,
    TextureAtlas, Vector3, Vector3Like,
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
    pub alpha_multiplier: Option<f64>,
    pub alpha_offset: Option<f64>,
    pub blue_multiplier: Option<f64>,
    pub blue_offset: Option<f64>,
    pub green_multiplier: Option<f64>,
    pub green_offset: Option<f64>,
    pub red_multiplier: Option<f64>,
    pub red_offset: Option<f64>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
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
    pub data: Option<NodeData>,
    pub enabled: Option<bool>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FlightPartialRecord15 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord15 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:16 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:17 (sha256:a68bb9efd032a89d93f67112fefca6fd2cfff5d832357b4e7a07ee6a442870a2)
const PARTICLE_COLOR_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:20 (sha256:585a7221ae5fdce81f34d23b2eddffa6eddb0882b479fae7fe335b6f12057b5d)
const PARTICLE_VELOCITY_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:22 (sha256:4cf7c9a26c52e92bf2e2a17e5e34c043832d23d843a350de94e0ecbfe30f0cb0)
pub const PARTICLE_EMITTER_3_D_DELETED_ID: f64 = 65535.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:24 (sha256:382baec1573441aa2594ebb9b91f9da090d9b94c67decea9be81b22acbf89f9b)
pub fn append_particle_emitter3_d_particle(
    target: &mut ParticleEmitter3D,
    id: f64,
    x: f64,
    y: f64,
    z: f64,
    rotation: f64,
    scale: f64,
) -> f64 {
    let index = target.data.particle_count;
    let needed = (index + 1.0_f64);
    if (get_particle_emitter3_d_capacity(target) < needed) {
        let new_capacity = (needed).max(if (target.data.particle_count * 2.0_f64) != 0.0_f64 {
            (target.data.particle_count * 2.0_f64)
        } else {
            8.0_f64
        });
        reserve_particle_emitter3_d(target, new_capacity);
    }
    target.data.particle_count = needed;
    target.data.ids[index as usize] = (id) as u16;
    let tt = (index * PARTICLE_TRANSFORM_STRIDE);
    target.data.transforms[tt as usize] = (x) as f32;
    target.data.transforms[(tt + 1.0_f64) as usize] = (y) as f32;
    target.data.transforms[(tt + 2.0_f64) as usize] = (rotation) as f32;
    target.data.transforms[(tt + 3.0_f64) as usize] = (scale) as f32;
    target.data.positions_z[index as usize] = (z) as f32;
    target.data.alphas[index as usize] = (1.0_f64) as f32;
    let ct = (index * PARTICLE_COLOR_STRIDE);
    target.data.colors[ct as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 1.0_f64) as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 2.0_f64) as usize] = (1.0_f64) as f32;
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (0.0_f64) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (0.0_f64) as f32;
    target.data.velocities[(vt + 2.0_f64) as usize] = (0.0_f64) as f32;
    return index;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:59 (sha256:83f11c03e3602fc761037e0b5d4f772eeba8fdbe20577ef302752e16557cf660)
pub fn clear_particle_emitter3_d(target: &mut ParticleEmitter3D) -> () {
    target.data.particle_count = 0.0_f64;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:63 (sha256:628cd7614df48156f182128798e9fd5bf25988b8cfb9ddcd4883c0bf30a7fc1d)
pub fn clone_particle_emitter3_d(source: &ParticleEmitter3D) -> ParticleEmitter3D {
    return create_particle_emitter3_d(Some(ParticleEmitter3D {
        __flight_identity: std::sync::Arc::new(()),
        blend_mode: (source.blend_mode).clone(),
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:81 (sha256:dbd0cebd3fb2aba2a3ff0343cd713cebe18cc5b035bc1c3babb9576352bf0d71)
pub fn compact_particle_emitter3_d(target: &mut ParticleEmitter3D) -> () {
    if (target.data.particle_count == 0.0_f64) {
        return;
    }
    let mut write = 0.0_f64;
    {
        let mut read = 0.0_f64;
        while (read < target.data.particle_count) {
            if ((target.data.ids[read as usize] as f64) == PARTICLE_EMITTER_3_D_DELETED_ID) {
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
                target.data.velocities[(vt + 2.0_f64) as usize] =
                    (target.data.velocities[(vts + 2.0_f64) as usize] as f64) as f32;
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:113 (sha256:1983bcc1bef2efb6120b8c96a2f74fe5ad849552db249755b28612eeab5c3d61)
pub fn compute_particle_emitter3_d_local_bounds_aabb(
    out: &mut AabbLike,
    source: &ParticleEmitter3D,
) -> () {
    let particle_count = source.data.particle_count;
    if (particle_count == 0.0_f64) {
        out.min.x = 0.0_f64;
        out.min.y = 0.0_f64;
        out.min.z = 0.0_f64;
        out.max.x = 0.0_f64;
        out.max.y = 0.0_f64;
        out.max.z = 0.0_f64;
        return;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    let mut max_z = (-f64::INFINITY);
    {
        let mut i = 0.0_f64;
        while (i < particle_count) {
            let tt = (i * PARTICLE_TRANSFORM_STRIDE);
            let px = (source.data.transforms[tt as usize] as f64);
            let py = (source.data.transforms[(tt + 1.0_f64) as usize] as f64);
            let pz = (source.data.positions_z[i as usize] as f64);
            let scale = (source.data.transforms[(tt + 3.0_f64) as usize] as f64);
            let r = (math.sqrt1_2 * if (scale < 0.0_f64) { (-scale) } else { scale });
            if ((px - r) < min_x) {
                min_x = (px - r);
            }
            if ((py - r) < min_y) {
                min_y = (py - r);
            }
            if ((pz - r) < min_z) {
                min_z = (pz - r);
            }
            if ((px + r) > max_x) {
                max_x = (px + r);
            }
            if ((py + r) > max_y) {
                max_y = (py + r);
            }
            if ((pz + r) > max_z) {
                max_z = (pz + r);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:154 (sha256:a941d5d8ab34bc166825cf8dde3c5777d059b861ceb5f606abf9b87f1cc8a371)
pub fn create_particle_emitter3_d(obj: Option<ParticleEmitter3D>) -> ParticleEmitter3D {
    let mut node = create_scene_node(
        Some((particle_emitter3_d_kind_constant).to_owned()),
        Some(((obj).clone().unwrap()).clone()),
    );
    node.data = create_particle_emitter_data(Some(
        (obj.as_ref().map(|value| (value.data).clone())).clone(),
    ));
    node.blend_mode =
        (obj.as_ref().map(|value| (value.blend_mode).clone())).unwrap_or("normal".to_owned());
    return node;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:161 (sha256:8783a1a2e035fb9a5c262ea4c2c232593c834c7167912e71631fad21491a7e91)
pub fn get_particle_emitter3_d_capacity(source: &ParticleEmitter3D) -> f64 {
    let transform_capacity =
        (__flight_js_to_i32(((source.data.transforms.len() as f64) / PARTICLE_TRANSFORM_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64;
    return ((source.data.ids.len() as f64).min((source.data.alphas.len() as f64)))
        .min(transform_capacity);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:167 (sha256:34696172d9b8786516c98aa37ec593ad466020a6791015d1b1a764af168d7473)
pub fn get_particle_emitter3_d_particle_alpha(source: &ParticleEmitter3D, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.alphas[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:172 (sha256:6da9448545e5e496112104e2f1adfd5aaf4f4d7a7e9b150a673fe50b6fd5f416)
pub fn get_particle_emitter3_d_particle_id(source: &ParticleEmitter3D, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.ids[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:177 (sha256:1d55e8ba919ffb0343bd02111a9578682929d91d162d1a72fbd11bca458f6e3b)
pub fn get_particle_emitter3_d_particle_velocity(
    out: &mut Vector3Like,
    source: &ParticleEmitter3D,
    index: f64,
) -> bool {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return false;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    out.x = (source.data.velocities[vt as usize] as f64);
    out.y = (source.data.velocities[(vt + 1.0_f64) as usize] as f64);
    out.z = (source.data.velocities[(vt + 2.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:190 (sha256:923d3618825db83f1dcdc5260dd880381eba99696cd133d09d42a4558ef84590)
pub fn get_particle_emitter3_d_runtime(source: &ParticleEmitter3D) -> ParticleEmitter3DRuntime {
    return get_scene_node_runtime(&source);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:194 (sha256:53ccdc048bf6625edc739e17e698d25ee6b76aeb8f9ba9256e6a43e004d85399)
#[derive(Clone)]
struct IsParticleEmitter3DRecord16 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for IsParticleEmitter3DRecord16 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_particle_emitter3_d(node: &IsParticleEmitter3DRecord16) -> bool {
    return ((node.kind).clone() == particle_emitter3_d_kind_constant);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:198 (sha256:3094d60232aefa4d76ff37c4195068cea23d41e3f31cab39040e980ebbbaaf5f)
pub fn remove_particle_emitter3_d_particle(target: &mut ParticleEmitter3D, index: f64) -> () {
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
        target.data.velocities[(vt + 2.0_f64) as usize] =
            (target.data.velocities[(vts + 2.0_f64) as usize] as f64) as f32;
        target.data.positions_z[index as usize] =
            (target.data.positions_z[last as usize] as f64) as f32;
    }
    target.data.particle_count = last;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:226 (sha256:fb7fee2176d02be2c677f1884edc603d4dcb7bc83c40c5b9221b5418d4f30506)
pub fn reserve_particle_emitter3_d(target: &mut ParticleEmitter3D, capacity: f64) -> () {
    if (get_particle_emitter3_d_capacity(target) >= capacity) {
        return;
    }
    target.data.alphas = reserve_float32_array(&target.data.alphas, capacity);
    target.data.colors =
        reserve_float32_array(&target.data.colors, (capacity * PARTICLE_COLOR_STRIDE));
    target.data.ids = reserve_uint16_array(&target.data.ids, capacity);
    target.data.positions_z = reserve_float32_array(&target.data.positions_z, capacity);
    target.data.transforms = reserve_float32_array(
        &target.data.transforms,
        (capacity * PARTICLE_TRANSFORM_STRIDE),
    );
    target.data.velocities = reserve_float32_array(
        &target.data.velocities,
        (capacity * PARTICLE_VELOCITY_STRIDE),
    );
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:237 (sha256:a3cf1d15cc63dfc25e76dfe9fb9caf5392eba4b0a2add18f6e4a26c727d40c0d)
pub fn set_particle_emitter3_d_particle(
    target: &mut ParticleEmitter3D,
    index: f64,
    id: f64,
    x: f64,
    y: f64,
    z: f64,
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
    target.data.positions_z[index as usize] = (z) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:258 (sha256:bb24af2612562f03b8bf5c95c95897383aa7f425270b71479eb20793a9dee670)
pub fn set_particle_emitter3_d_particle_alpha(
    target: &mut ParticleEmitter3D,
    index: f64,
    alpha: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    target.data.alphas[index as usize] = (alpha) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:263 (sha256:a25ff808d8a466b32bd7d9391a881c4f79980472a0782e31f6046a73a03eada9)
pub fn set_particle_emitter3_d_particle_color(
    target: &mut ParticleEmitter3D,
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:277 (sha256:0d35ba999cfaff1d04815ceaa1a7f00561cd41c570a8e47473061d16aeeccd2a)
pub fn set_particle_emitter3_d_particle_velocity(
    target: &mut ParticleEmitter3D,
    index: f64,
    vx: f64,
    vy: f64,
    vz: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (vx) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (vy) as f32;
    target.data.velocities[(vt + 2.0_f64) as usize] = (vz) as f32;
}
