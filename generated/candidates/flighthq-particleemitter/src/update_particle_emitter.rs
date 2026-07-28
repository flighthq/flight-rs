// @generated from upstream/packages/particleemitter/src/updateParticleEmitter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::reserve_particle_emitter;
use flighthq_node::{get_node_world_matrix, invalidate_node_local_bounds};
use flighthq_particles::{
    PARTICLE_VELOCITY_STRIDE as particle_velocity_stride_constant,
    ensure_particle_emitter_state_capacity, get_particle_emitter_signals,
    sample_particle_color_curve, sample_particle_curve,
};
pub use flighthq_types::ParticleEmitterCallbacks;
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, ParticleEmitter, ParticleEmitterConfig, ParticleEmitterState, Rectangle, Stage,
    TextureAtlas,
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
    pub alpha_multiplier: Option<f64>,
    pub alpha_offset: Option<f64>,
    pub blue_multiplier: Option<f64>,
    pub blue_offset: Option<f64>,
    pub green_multiplier: Option<f64>,
    pub green_offset: Option<f64>,
    pub red_multiplier: Option<f64>,
    pub red_offset: Option<f64>,
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
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
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

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:21 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:22 (sha256:eed66b9413dd1f3589bacb1e13051ec02a5deec60c911cd17bad429c276ac74b)
const TWO_PI: f64 = 6.283185307179586_f64;

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:27 (sha256:d9d22bb79120eae266ea7b93c05a892cd0851a612a150ce59a1cfc4b93bb290a)
pub fn is_particle_emitter_complete(
    emitter: &ParticleEmitter,
    state: &ParticleEmitterState,
    config: &ParticleEmitterConfig,
) -> bool {
    if (config.duration <= 0.0_f64) || (config.loop_) {
        return false;
    }
    return (state.emitter_age >= config.duration) && (emitter.data.particle_count == 0.0_f64);
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:38 (sha256:e0c225ef7b89615e590a9ca4c09617b0a80f425bf3c179ad015ea08eb69f83c0)
fn is_emitting(config: &ParticleEmitterConfig, emitter_age: f64) -> bool {
    return ((config.duration <= 0.0_f64) || (config.loop_)) || (emitter_age < config.duration);
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:42 (sha256:cba39fee10e8d9ac4aa211e8f4b4f6327c0bf6d4615de8d2ce6cae16af4fe213)
pub fn update_particle_emitter(
    emitter: &mut ParticleEmitter,
    state: &mut ParticleEmitterState,
    config: &ParticleEmitterConfig,
    delta_time: f64,
    callbacks: Option<ParticleEmitterCallbacks>,
) -> () {
    let world_transform = if config.world_space {
        Some(get_node_world_matrix(&emitter))
    } else {
        None
    };
    emitter.data.world_space = (world_transform).is_some();
    if (delta_time <= 0.0_f64) {
        return;
    }
    let track_x = if (world_transform).is_some() {
        world_transform.as_ref().unwrap().tx
    } else {
        emitter.x
    };
    let track_y = if (world_transform).is_some() {
        world_transform.as_ref().unwrap().ty
    } else {
        emitter.y
    };
    let has_vel_inherit = (config.velocity_inheritance != 0.0_f64);
    let mut emitter_vel_x = 0.0_f64;
    let mut emitter_vel_y = 0.0_f64;
    if (!crate::host_value::<()>("host.call")) {
        emitter_vel_x = ((track_x - state.prev_x) / delta_time);
        emitter_vel_y = ((track_y - state.prev_y) / delta_time);
    }
    let gx = (config.gravity_x * delta_time);
    let gy = (config.gravity_y * delta_time);
    let gz = (config.gravity_z * delta_time);
    let color_start_r = config.color_start_r;
    let color_start_g = config.color_start_g;
    let color_start_b = config.color_start_b;
    let color_end_r = config.color_end_r;
    let color_end_g = config.color_end_g;
    let color_end_b = config.color_end_b;
    let has_color_variance = (((((config.color_start_variance_r != 0.0_f64)
        || (config.color_start_variance_g != 0.0_f64))
        || (config.color_start_variance_b != 0.0_f64))
        || (config.color_end_variance_r != 0.0_f64))
        || (config.color_end_variance_g != 0.0_f64))
        || (config.color_end_variance_b != 0.0_f64);
    let has_color_gradient = (((has_color_variance) || (color_start_r != color_end_r))
        || (color_start_g != color_end_g))
        || (color_start_b != color_end_b);
    let alpha_curve = (config.alpha_curve).clone();
    let color_curve = (config.color_curve).clone();
    let scale_curve = (config.scale_curve).clone();
    let has_alpha_curve =
        ((alpha_curve).is_some()) && (alpha_curve.as_ref().unwrap().length > 0.0_f64);
    let has_color_curve =
        ((color_curve).is_some()) && (color_curve.as_ref().unwrap().length >= 3.0_f64);
    let has_scale_curve =
        ((scale_curve).is_some()) && (scale_curve.as_ref().unwrap().length > 0.0_f64);
    let has_scale_anim = (config.scale_end != 1.0_f64) || (has_scale_curve);
    let has_color_work = (has_color_curve) || (has_color_gradient);
    let has_rotation_speed =
        (config.rotation_speed_min != 0.0_f64) || (config.rotation_speed_max != 0.0_f64);
    let has_flipbook = (config.frame_count > 1.0_f64);
    let on_death = callbacks
        .as_ref()
        .and_then(|value| (value.on_death).clone());
    let on_spawn = callbacks
        .as_ref()
        .and_then(|value| (value.on_spawn).clone());
    let signals = get_particle_emitter_signals(state);
    let mut live_count = emitter.data.particle_count;
    let mut i = 0.0_f64;
    while (i < live_count) {
        let lt = (i * 2.0_f64);
        state.lifetimes[lt as usize] += (delta_time) as f32;
        if ((state.lifetimes[lt as usize] as f64)
            >= (state.lifetimes[(lt + 1.0_f64) as usize] as f64))
        {
            if ((on_death).is_some()) || ((signals).is_some()) {
                let tt = (i * PARTICLE_TRANSFORM_STRIDE);
                let dx = (emitter.data.transforms[tt as usize] as f64);
                let dy = (emitter.data.transforms[(tt + 1.0_f64) as usize] as f64);
                {
                    let __flight_callback = on_death;
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()(dx, dy, 0.0_f64))
                };
                (signals
                    .as_ref()
                    .unwrap()
                    .on_particle_death
                    .as_ref()
                    .map(|value| (value.emit).clone()))(dx, dy, 0.0_f64);
            }
            {
                live_count -= 1.0;
                live_count
            };
            if (i < live_count) {
                let lt2 = (live_count * 2.0_f64);
                state.lifetimes[lt as usize] = (state.lifetimes[lt2 as usize] as f64) as f32;
                state.lifetimes[(lt + 1.0_f64) as usize] =
                    (state.lifetimes[(lt2 + 1.0_f64) as usize] as f64) as f32;
                let vt = (i * particle_velocity_stride_constant);
                let vt2 = (live_count * particle_velocity_stride_constant);
                state.velocities[vt as usize] = (state.velocities[vt2 as usize] as f64) as f32;
                state.velocities[(vt + 1.0_f64) as usize] =
                    (state.velocities[(vt2 + 1.0_f64) as usize] as f64) as f32;
                state.velocities[(vt + 2.0_f64) as usize] =
                    (state.velocities[(vt2 + 2.0_f64) as usize] as f64) as f32;
                let tt = (i * PARTICLE_TRANSFORM_STRIDE);
                let tt2 = (live_count * PARTICLE_TRANSFORM_STRIDE);
                emitter.data.transforms[tt as usize] =
                    (emitter.data.transforms[tt2 as usize] as f64) as f32;
                emitter.data.transforms[(tt + 1.0_f64) as usize] =
                    (emitter.data.transforms[(tt2 + 1.0_f64) as usize] as f64) as f32;
                emitter.data.transforms[(tt + 2.0_f64) as usize] =
                    (emitter.data.transforms[(tt2 + 2.0_f64) as usize] as f64) as f32;
                emitter.data.transforms[(tt + 3.0_f64) as usize] =
                    (emitter.data.transforms[(tt2 + 3.0_f64) as usize] as f64) as f32;
                emitter.data.positions_z[i as usize] =
                    (emitter.data.positions_z[live_count as usize] as f64) as f32;
                emitter.data.alphas[i as usize] =
                    (emitter.data.alphas[live_count as usize] as f64) as f32;
                emitter.data.ids[i as usize] =
                    (emitter.data.ids[live_count as usize] as f64) as u16;
                let ct = (i * 3.0_f64);
                let ct2 = (live_count * 3.0_f64);
                emitter.data.colors[ct as usize] =
                    (emitter.data.colors[ct2 as usize] as f64) as f32;
                emitter.data.colors[(ct + 1.0_f64) as usize] =
                    (emitter.data.colors[(ct2 + 1.0_f64) as usize] as f64) as f32;
                emitter.data.colors[(ct + 2.0_f64) as usize] =
                    (emitter.data.colors[(ct2 + 2.0_f64) as usize] as f64) as f32;
                state.scales[i as usize] = (state.scales[live_count as usize] as f64) as f32;
                state.rotation_speeds[i as usize] =
                    (state.rotation_speeds[live_count as usize] as f64) as f32;
                if has_color_variance {
                    state.color_birth[ct as usize] =
                        (state.color_birth[ct2 as usize] as f64) as f32;
                    state.color_birth[(ct + 1.0_f64) as usize] =
                        (state.color_birth[(ct2 + 1.0_f64) as usize] as f64) as f32;
                    state.color_birth[(ct + 2.0_f64) as usize] =
                        (state.color_birth[(ct2 + 2.0_f64) as usize] as f64) as f32;
                    state.color_death[ct as usize] =
                        (state.color_death[ct2 as usize] as f64) as f32;
                    state.color_death[(ct + 1.0_f64) as usize] =
                        (state.color_death[(ct2 + 1.0_f64) as usize] as f64) as f32;
                    state.color_death[(ct + 2.0_f64) as usize] =
                        (state.color_death[(ct2 + 2.0_f64) as usize] as f64) as f32;
                }
            }
            continue;
        }
        let vt = (i * particle_velocity_stride_constant);
        state.velocities[vt as usize] += (gx) as f32;
        state.velocities[(vt + 1.0_f64) as usize] += (gy) as f32;
        state.velocities[(vt + 2.0_f64) as usize] += (gz) as f32;
        let tt = (i * PARTICLE_TRANSFORM_STRIDE);
        emitter.data.transforms[tt as usize] +=
            ((state.velocities[vt as usize] as f64) * delta_time) as f32;
        emitter.data.transforms[(tt + 1.0_f64) as usize] +=
            ((state.velocities[(vt + 1.0_f64) as usize] as f64) * delta_time) as f32;
        emitter.data.positions_z[i as usize] +=
            ((state.velocities[(vt + 2.0_f64) as usize] as f64) * delta_time) as f32;
        let life_fraction = ((state.lifetimes[lt as usize] as f64)
            / (state.lifetimes[(lt + 1.0_f64) as usize] as f64));
        emitter.data.alphas[i as usize] = if has_alpha_curve {
            (sample_particle_curve(alpha_curve.as_ref().unwrap(), life_fraction)) as f32
        } else {
            (config.alpha_start + ((config.alpha_end - config.alpha_start) * life_fraction)) as f32
        };
        if has_color_work {
            let ct = (i * 3.0_f64);
            if has_color_curve {
                sample_particle_color_curve(
                    &mut emitter.data.colors,
                    ct,
                    color_curve.as_ref().unwrap(),
                    life_fraction,
                );
            } else {
                if has_color_variance {
                    emitter.data.colors[ct as usize] = ((state.color_birth[ct as usize] as f64)
                        + (((state.color_death[ct as usize] as f64)
                            - (state.color_birth[ct as usize] as f64))
                            * life_fraction))
                        as f32;
                    emitter.data.colors[(ct + 1.0_f64) as usize] =
                        ((state.color_birth[(ct + 1.0_f64) as usize] as f64)
                            + (((state.color_death[(ct + 1.0_f64) as usize] as f64)
                                - (state.color_birth[(ct + 1.0_f64) as usize] as f64))
                                * life_fraction)) as f32;
                    emitter.data.colors[(ct + 2.0_f64) as usize] =
                        ((state.color_birth[(ct + 2.0_f64) as usize] as f64)
                            + (((state.color_death[(ct + 2.0_f64) as usize] as f64)
                                - (state.color_birth[(ct + 2.0_f64) as usize] as f64))
                                * life_fraction)) as f32;
                } else {
                    emitter.data.colors[ct as usize] =
                        (color_start_r + ((color_end_r - color_start_r) * life_fraction)) as f32;
                    emitter.data.colors[(ct + 1.0_f64) as usize] =
                        (color_start_g + ((color_end_g - color_start_g) * life_fraction)) as f32;
                    emitter.data.colors[(ct + 2.0_f64) as usize] =
                        (color_start_b + ((color_end_b - color_start_b) * life_fraction)) as f32;
                }
            }
        }
        if has_scale_anim {
            let scale_factor = if has_scale_curve {
                sample_particle_curve(scale_curve.as_ref().unwrap(), life_fraction)
            } else {
                (1.0_f64 + ((config.scale_end - 1.0_f64) * life_fraction))
            };
            emitter.data.transforms[(tt + 3.0_f64) as usize] =
                ((state.scales[i as usize] as f64) * scale_factor) as f32;
        }
        if has_rotation_speed {
            emitter.data.transforms[(tt + 2.0_f64) as usize] +=
                ((state.rotation_speeds[i as usize] as f64) * delta_time) as f32;
        }
        if has_flipbook {
            let frame = (((state.lifetimes[lt as usize] as f64) * config.frame_rate).floor()
                % config.frame_count);
            emitter.data.ids[i as usize] = (config.region_id_min + frame) as u16;
        }
        {
            i += 1.0;
            i
        };
    }
    emitter.data.particle_count = live_count;
    let emitting = is_emitting(config, state.emitter_age);
    if (config.duration > 0.0_f64) && (!config.loop_) {
        state.emitter_age += delta_time;
    }
    state.spawn_accumulator += if emitting {
        (config.spawn_rate * delta_time)
    } else {
        0.0_f64
    };
    let mut to_spawn = (state.spawn_accumulator).floor();
    state.spawn_accumulator -= to_spawn;
    if (emitting) && (config.burst_count > 0.0_f64) {
        state.burst_timer -= delta_time;
        if (state.burst_timer <= 0.0_f64) {
            to_spawn += config.burst_count;
            state.burst_timer = if (config.burst_interval > 0.0_f64) {
                config.burst_interval
            } else {
                f64::INFINITY
            };
        }
    }
    let max_new = (config.max_particles - live_count);
    if (to_spawn > max_new) {
        to_spawn = max_new;
    }
    if (to_spawn > 0.0_f64) {
        let new_count = (live_count + to_spawn);
        reserve_particle_emitter(emitter, new_count);
        ensure_particle_emitter_state_capacity(state, new_count, has_color_variance);
        let base_angle = (config.direction_y).atan2(config.direction_x);
        let region_range = (config.region_id_max - config.region_id_min);
        let region_id_min = config.region_id_min;
        let rot_speed_range = (config.rotation_speed_max - config.rotation_speed_min);
        let has_rot_speed =
            (config.rotation_speed_min != 0.0_f64) || (config.rotation_speed_max != 0.0_f64);
        let do_trail = ((world_transform).is_some()) && (!crate::host_value::<()>("host.call"));
        let prev_path_x = if do_trail { state.prev_x } else { track_x };
        let prev_path_y = if do_trail { state.prev_y } else { track_y };
        let dir_len = (((config.direction_x * config.direction_x)
            + (config.direction_y * config.direction_y))
            + (config.direction_z * config.direction_z))
            .sqrt();
        let dir_nx = if (dir_len > 0.000001_f64) {
            (config.direction_x / dir_len)
        } else {
            0.0_f64
        };
        let dir_ny = if (dir_len > 0.000001_f64) {
            (config.direction_y / dir_len)
        } else {
            (-1.0_f64)
        };
        let dir_nz = if (dir_len > 0.000001_f64) {
            (config.direction_z / dir_len)
        } else {
            0.0_f64
        };
        {
            let mut s_idx = 0.0_f64;
            while (s_idx < to_spawn) {
                let idx = (live_count + s_idx);
                let lifetime = (config.lifetime_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.lifetime_max - config.lifetime_min)));
                let lt = (idx * 2.0_f64);
                state.lifetimes[lt as usize] = (0.0_f64) as f32;
                state.lifetimes[(lt + 1.0_f64) as usize] = (lifetime) as f32;
                let speed = (config.speed_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.speed_max - config.speed_min)));
                let mut vx: f64;
                let mut vy: f64;
                let mut vz: f64;
                let mut spawn_x = 0.0_f64;
                let mut spawn_y = 0.0_f64;
                let mut spawn_z = 0.0_f64;
                let shape = (config.emitter_shape).clone();
                if (shape == "sphere") || (shape == "cone3d") {
                    let mut sx: f64;
                    let mut sy: f64;
                    let mut sz: f64;
                    if (shape == "cone3d") && (config.emitter_cone_angle > 0.0_f64) {
                        let cone_half = (config.emitter_cone_angle / 2.0_f64);
                        let cos_theta = (1.0_f64
                            - ({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } * (1.0_f64 - (cone_half).cos())));
                        let sin_theta = (1.0_f64 - (cos_theta * cos_theta)).sqrt();
                        let phi = ({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } * TWO_PI);
                        let lx = (sin_theta * (phi).cos());
                        let ly = (sin_theta * (phi).sin());
                        let lz = cos_theta;
                        let r_dir = rotate_to_direction(lx, ly, lz, dir_nx, dir_ny, dir_nz);
                        sx = r_dir[0.0_f64 as usize].clone();
                        sy = r_dir[1.0_f64 as usize].clone();
                        sz = r_dir[2.0_f64 as usize].clone();
                    } else {
                        let mut u: f64;
                        let mut v: f64;
                        let mut s2: f64;
                        loop {
                            {
                                u = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                v = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                s2 = ((u * u) + (v * v));
                            }
                            if !((s2 >= 1.0_f64) || (s2 == 0.0_f64)) {
                                break;
                            }
                        }
                        let f = (2.0_f64 * (1.0_f64 - s2).sqrt());
                        sx = (u * f);
                        sy = (v * f);
                        sz = (1.0_f64 - (2.0_f64 * s2));
                    }
                    vx = (sx * speed);
                    vy = (sy * speed);
                    vz = (sz * speed);
                    if (config.emitter_radius > 0.0_f64) {
                        let r = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        })
                        .cbrt()
                            * config.emitter_radius);
                        let mut pu: f64;
                        let mut pv: f64;
                        let mut ps2: f64;
                        loop {
                            {
                                pu = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                pv = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                ps2 = ((pu * pu) + (pv * pv));
                            }
                            if !((ps2 >= 1.0_f64) || (ps2 == 0.0_f64)) {
                                break;
                            }
                        }
                        let pf = (2.0_f64 * (1.0_f64 - ps2).sqrt());
                        spawn_x = ((pu * pf) * r);
                        spawn_y = ((pv * pf) * r);
                        spawn_z = ((1.0_f64 - (2.0_f64 * ps2)) * r);
                    }
                } else {
                    if (shape == "box") {
                        let angle = (base_angle
                            + ((({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } - 0.5_f64)
                                * 2.0_f64)
                                * config.spread));
                        vx = ((angle).cos() * speed);
                        vy = ((angle).sin() * speed);
                        vz = ((config.direction_z * speed)
                            / if (dir_len > 0.000001_f64) {
                                dir_len
                            } else {
                                1.0_f64
                            });
                        spawn_x = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_width);
                        spawn_y = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_height);
                        spawn_z = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_depth);
                    } else {
                        let angle = (base_angle
                            + ((({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } - 0.5_f64)
                                * 2.0_f64)
                                * config.spread));
                        vx = ((angle).cos() * speed);
                        vy = ((angle).sin() * speed);
                        vz = 0.0_f64;
                        if (shape == "circle") && (config.emitter_radius > 0.0_f64) {
                            let r = (({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            })
                            .sqrt()
                                * config.emitter_radius);
                            let a = ({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } * TWO_PI);
                            spawn_x = ((a).cos() * r);
                            spawn_y = ((a).sin() * r);
                        } else {
                            if (shape == "rect")
                                && ((config.emitter_width > 0.0_f64)
                                    || (config.emitter_height > 0.0_f64))
                            {
                                spawn_x = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * config.emitter_width);
                                spawn_y = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * config.emitter_height);
                            }
                        }
                    }
                }
                if (world_transform).is_some() {
                    let t = if (to_spawn > 1.0_f64) {
                        (s_idx / (to_spawn - 1.0_f64))
                    } else {
                        1.0_f64
                    };
                    let origin_x = (prev_path_x + ((track_x - prev_path_x) * t));
                    let origin_y = (prev_path_y + ((track_y - prev_path_y) * t));
                    let wx = (((world_transform.as_ref().unwrap().a * spawn_x)
                        + (world_transform.as_ref().unwrap().c * spawn_y))
                        + origin_x);
                    let wy = (((world_transform.as_ref().unwrap().b * spawn_x)
                        + (world_transform.as_ref().unwrap().d * spawn_y))
                        + origin_y);
                    spawn_x = wx;
                    spawn_y = wy;
                    let wvx = ((world_transform.as_ref().unwrap().a * vx)
                        + (world_transform.as_ref().unwrap().c * vy));
                    let wvy = ((world_transform.as_ref().unwrap().b * vx)
                        + (world_transform.as_ref().unwrap().d * vy));
                    vx = wvx;
                    vy = wvy;
                }
                if (has_vel_inherit) && (!crate::host_value::<()>("host.call")) {
                    vx += (emitter_vel_x * config.velocity_inheritance);
                    vy += (emitter_vel_y * config.velocity_inheritance);
                }
                let vt = (idx * particle_velocity_stride_constant);
                state.velocities[vt as usize] = (vx) as f32;
                state.velocities[(vt + 1.0_f64) as usize] = (vy) as f32;
                state.velocities[(vt + 2.0_f64) as usize] = (vz) as f32;
                let spawn_scale = (config.scale_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.scale_max - config.scale_min)));
                state.scales[idx as usize] = (spawn_scale) as f32;
                let tt = (idx * PARTICLE_TRANSFORM_STRIDE);
                emitter.data.transforms[tt as usize] = (spawn_x) as f32;
                emitter.data.transforms[(tt + 1.0_f64) as usize] = (spawn_y) as f32;
                let spawn_angle = if (shape == "sphere") || (shape == "cone3d") {
                    base_angle
                } else {
                    (base_angle
                        + ((({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * 2.0_f64)
                            * config.spread))
                };
                emitter.data.transforms[(tt + 2.0_f64) as usize] =
                    if ((shape == "sphere") || (shape == "cone3d")) || (shape == "box") {
                        ((vy).atan2(vx)) as f32
                    } else {
                        (spawn_angle) as f32
                    };
                emitter.data.transforms[(tt + 3.0_f64) as usize] = if has_scale_curve {
                    (spawn_scale * sample_particle_curve(scale_curve.as_ref().unwrap(), 0.0_f64))
                        as f32
                } else {
                    (spawn_scale) as f32
                };
                emitter.data.positions_z[idx as usize] = (spawn_z) as f32;
                emitter.data.alphas[idx as usize] = if has_alpha_curve {
                    (sample_particle_curve(alpha_curve.as_ref().unwrap(), 0.0_f64)) as f32
                } else {
                    (config.alpha_start) as f32
                };
                let ct = (idx * 3.0_f64);
                if has_color_curve {
                    sample_particle_color_curve(
                        &mut emitter.data.colors,
                        ct,
                        color_curve.as_ref().unwrap(),
                        0.0_f64,
                    );
                } else {
                    if has_color_variance {
                        let r0 = clamp01(
                            (color_start_r
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_start_variance_r)),
                        );
                        let g0 = clamp01(
                            (color_start_g
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_start_variance_g)),
                        );
                        let b0 = clamp01(
                            (color_start_b
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_start_variance_b)),
                        );
                        let r1 = clamp01(
                            (color_end_r
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_end_variance_r)),
                        );
                        let g1 = clamp01(
                            (color_end_g
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_end_variance_g)),
                        );
                        let b1 = clamp01(
                            (color_end_b
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_end_variance_b)),
                        );
                        state.color_birth[ct as usize] = (r0) as f32;
                        state.color_birth[(ct + 1.0_f64) as usize] = (g0) as f32;
                        state.color_birth[(ct + 2.0_f64) as usize] = (b0) as f32;
                        state.color_death[ct as usize] = (r1) as f32;
                        state.color_death[(ct + 1.0_f64) as usize] = (g1) as f32;
                        state.color_death[(ct + 2.0_f64) as usize] = (b1) as f32;
                        emitter.data.colors[ct as usize] = (r0) as f32;
                        emitter.data.colors[(ct + 1.0_f64) as usize] = (g0) as f32;
                        emitter.data.colors[(ct + 2.0_f64) as usize] = (b0) as f32;
                    } else {
                        emitter.data.colors[ct as usize] = (color_start_r) as f32;
                        emitter.data.colors[(ct + 1.0_f64) as usize] = (color_start_g) as f32;
                        emitter.data.colors[(ct + 2.0_f64) as usize] = (color_start_b) as f32;
                    }
                }
                emitter.data.ids[idx as usize] = (region_id_min
                    + if (config.frame_count > 1.0_f64) {
                        0.0_f64
                    } else {
                        if (region_range > 0.0_f64) {
                            (__flight_js_to_i32(
                                ({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * region_range),
                            ) | __flight_js_to_i32(0.0_f64)) as f64
                        } else {
                            0.0_f64
                        }
                    }) as u16;
                state.rotation_speeds[idx as usize] = if has_rot_speed {
                    (config.rotation_speed_min
                        + ({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } * rot_speed_range)) as f32
                } else {
                    (0.0_f64) as f32
                };
                {
                    let __flight_callback = on_spawn;
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()(spawn_x, spawn_y, 0.0_f64))
                };
                if (signals).is_some() {
                    ((signals.as_ref().unwrap().on_particle_spawn.emit).clone())(
                        spawn_x,
                        spawn_y,
                        0.0_f64,
                        (state.velocities[vt as usize] as f64),
                        (state.velocities[(vt + 1.0_f64) as usize] as f64),
                        0.0_f64,
                    );
                }
                {
                    s_idx += 1.0;
                    s_idx
                };
            }
        }
        emitter.data.particle_count = new_count;
    }
    state.prev_x = track_x;
    state.prev_y = track_y;
    let live_render_velocity_count = (emitter.data.particle_count * 2.0_f64);
    if ((emitter.data.velocities.len() as f64) >= live_render_velocity_count) {
        {
            let mut vi = 0.0_f64;
            while (vi < emitter.data.particle_count) {
                let src = (vi * particle_velocity_stride_constant);
                let dst = (vi * 2.0_f64);
                emitter.data.velocities[dst as usize] =
                    (state.velocities[src as usize] as f64) as f32;
                emitter.data.velocities[(dst + 1.0_f64) as usize] =
                    (state.velocities[(src + 1.0_f64) as usize] as f64) as f32;
                {
                    vi += 1.0;
                    vi
                };
            }
        }
    }
    if ((signals).is_some()) && (is_particle_emitter_complete(emitter, state, config)) {
        ((signals.as_ref().unwrap().on_emitter_complete.emit).clone())();
    }
    invalidate_node_local_bounds(emitter);
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:481 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:487 (sha256:50a57123f6b45907eaef3ae1022afb661b7f38f7a0e7fcbd096e0aa2999baa2f)
static _ROT: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/particleemitter/src/updateParticleEmitter.ts:488 (sha256:f857c56076aa46a3b58101517d531a456e4a66af57ded4cca0d02ea8d7f3c2de)
fn rotate_to_direction(lx: f64, ly: f64, lz: f64, dx: f64, dy: f64, dz: f64) -> Vec<f64> {
    let kx = (-dy);
    let ky = dx;
    let sin_angle = ((kx * kx) + (ky * ky)).sqrt();
    let cos_angle = dz;
    if (sin_angle < 0.000001_f64) {
        if (cos_angle > 0.0_f64) {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = lx;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = ly;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = lz;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
        } else {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = lx;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = (-ly);
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = (-lz);
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
        }
        return (_ROT).clone();
    }
    let inv_sin = (1.0_f64 / sin_angle);
    let ax = (kx * inv_sin);
    let ay = (ky * inv_sin);
    let kdotv = ((ax * lx) + (ay * ly));
    let cross_x = (ay * lz);
    let cross_y = ((-ax) * lz);
    let cross_z = ((ax * ly) - (ay * lx));
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value =
            (((lx * cos_angle) + (cross_x * sin_angle)) + ((ax * kdotv) * (1.0_f64 - cos_angle)));
        if __flight_index == _ROT.lock().unwrap().len() {
            _ROT.lock().unwrap().push(__flight_value);
        } else {
            _ROT.lock().unwrap()[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value =
            (((ly * cos_angle) + (cross_y * sin_angle)) + ((ay * kdotv) * (1.0_f64 - cos_angle)));
        if __flight_index == _ROT.lock().unwrap().len() {
            _ROT.lock().unwrap().push(__flight_value);
        } else {
            _ROT.lock().unwrap()[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (((lz * cos_angle) + (cross_z * sin_angle)) + 0.0_f64);
        if __flight_index == _ROT.lock().unwrap().len() {
            _ROT.lock().unwrap().push(__flight_value);
        } else {
            _ROT.lock().unwrap()[__flight_index] = __flight_value;
        }
    };
    return (_ROT).clone();
}
