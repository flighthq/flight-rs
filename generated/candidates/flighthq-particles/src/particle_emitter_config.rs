// @generated from upstream/packages/particles/src/particleEmitterConfig.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ParticleBlendMode, ParticleCurve, ParticleEmitterConfig, ParticleEmitterShape,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord603063107 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_end: Option<f64>,
    pub alpha_start: Option<f64>,
    pub blend_mode: Option<ParticleBlendMode>,
    pub color_end_b: Option<f64>,
    pub color_end_g: Option<f64>,
    pub color_end_r: Option<f64>,
    pub color_end_variance_b: Option<f64>,
    pub color_end_variance_g: Option<f64>,
    pub color_end_variance_r: Option<f64>,
    pub color_start_b: Option<f64>,
    pub color_start_g: Option<f64>,
    pub color_start_r: Option<f64>,
    pub color_start_variance_b: Option<f64>,
    pub color_start_variance_g: Option<f64>,
    pub color_start_variance_r: Option<f64>,
    pub direction_x: Option<f64>,
    pub direction_y: Option<f64>,
    pub direction_z: Option<f64>,
    pub gravity_x: Option<f64>,
    pub gravity_y: Option<f64>,
    pub gravity_z: Option<f64>,
    pub emitter_cone_angle: Option<f64>,
    pub emitter_depth: Option<f64>,
    pub emitter_height: Option<f64>,
    pub emitter_radius: Option<f64>,
    pub emitter_shape: Option<ParticleEmitterShape>,
    pub emitter_width: Option<f64>,
    pub burst_count: Option<f64>,
    pub burst_interval: Option<f64>,
    pub duration: Option<f64>,
    pub loop_: Option<bool>,
    pub frame_count: Option<f64>,
    pub frame_rate: Option<f64>,
    pub lifetime_max: Option<f64>,
    pub lifetime_min: Option<f64>,
    pub max_particles: Option<f64>,
    pub region_id_max: Option<f64>,
    pub region_id_min: Option<f64>,
    pub scale_end: Option<f64>,
    pub scale_max: Option<f64>,
    pub scale_min: Option<f64>,
    pub speed_max: Option<f64>,
    pub speed_min: Option<f64>,
    pub spawn_rate: Option<f64>,
    pub spread: Option<f64>,
    pub rotation_speed_max: Option<f64>,
    pub rotation_speed_min: Option<f64>,
    pub velocity_inheritance: Option<f64>,
    pub alpha_curve: Option<ParticleCurve>,
    pub color_curve: Option<ParticleCurve>,
    pub scale_curve: Option<ParticleCurve>,
    pub world_space: Option<bool>,
}
impl PartialEq for FlightPartialRecord603063107 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particles/src/particleEmitterConfig.ts:3 (sha256:abb2ffb0f022c9535292c6b263f485ec1255b9f3d59984a190504d18e5894724)
pub fn create_particle_emitter_config(
    config: Option<FlightPartialRecord603063107>,
) -> ParticleEmitterConfig {
    return ParticleEmitterConfig {
        __flight_identity: std::sync::Arc::new(()),
        alpha_curve: config
            .as_ref()
            .and_then(|value| (value.alpha_curve).clone()),
        alpha_end: (config.as_ref().and_then(|value| value.alpha_end))
            .clone()
            .unwrap_or(0.0_f64),
        alpha_start: (config.as_ref().and_then(|value| value.alpha_start))
            .clone()
            .unwrap_or(1.0_f64),
        blend_mode: config.as_ref().and_then(|value| (value.blend_mode).clone()),
        burst_count: (config.as_ref().and_then(|value| value.burst_count))
            .clone()
            .unwrap_or(0.0_f64),
        burst_interval: (config.as_ref().and_then(|value| value.burst_interval))
            .clone()
            .unwrap_or(0.0_f64),
        color_curve: config
            .as_ref()
            .and_then(|value| (value.color_curve).clone()),
        color_end_b: (config.as_ref().and_then(|value| value.color_end_b))
            .clone()
            .unwrap_or(1.0_f64),
        color_end_g: (config.as_ref().and_then(|value| value.color_end_g))
            .clone()
            .unwrap_or(1.0_f64),
        color_end_r: (config.as_ref().and_then(|value| value.color_end_r))
            .clone()
            .unwrap_or(1.0_f64),
        color_end_variance_b: (config.as_ref().and_then(|value| value.color_end_variance_b))
            .clone()
            .unwrap_or(0.0_f64),
        color_end_variance_g: (config.as_ref().and_then(|value| value.color_end_variance_g))
            .clone()
            .unwrap_or(0.0_f64),
        color_end_variance_r: (config.as_ref().and_then(|value| value.color_end_variance_r))
            .clone()
            .unwrap_or(0.0_f64),
        color_start_b: (config.as_ref().and_then(|value| value.color_start_b))
            .clone()
            .unwrap_or(1.0_f64),
        color_start_g: (config.as_ref().and_then(|value| value.color_start_g))
            .clone()
            .unwrap_or(1.0_f64),
        color_start_r: (config.as_ref().and_then(|value| value.color_start_r))
            .clone()
            .unwrap_or(1.0_f64),
        color_start_variance_b: (config
            .as_ref()
            .and_then(|value| value.color_start_variance_b))
        .clone()
        .unwrap_or(0.0_f64),
        color_start_variance_g: (config
            .as_ref()
            .and_then(|value| value.color_start_variance_g))
        .clone()
        .unwrap_or(0.0_f64),
        color_start_variance_r: (config
            .as_ref()
            .and_then(|value| value.color_start_variance_r))
        .clone()
        .unwrap_or(0.0_f64),
        direction_x: (config.as_ref().and_then(|value| value.direction_x))
            .clone()
            .unwrap_or(0.0_f64),
        direction_y: (config.as_ref().and_then(|value| value.direction_y))
            .clone()
            .unwrap_or((-1.0_f64)),
        direction_z: (config.as_ref().and_then(|value| value.direction_z))
            .clone()
            .unwrap_or(0.0_f64),
        duration: (config.as_ref().and_then(|value| value.duration))
            .clone()
            .unwrap_or(0.0_f64),
        emitter_cone_angle: (config.as_ref().and_then(|value| value.emitter_cone_angle))
            .clone()
            .unwrap_or(0.0_f64),
        emitter_depth: (config.as_ref().and_then(|value| value.emitter_depth))
            .clone()
            .unwrap_or(0.0_f64),
        emitter_height: (config.as_ref().and_then(|value| value.emitter_height))
            .clone()
            .unwrap_or(0.0_f64),
        emitter_radius: (config.as_ref().and_then(|value| value.emitter_radius))
            .clone()
            .unwrap_or(0.0_f64),
        emitter_shape: (config
            .as_ref()
            .and_then(|value| (value.emitter_shape).clone()))
        .clone()
        .unwrap_or("point".to_owned()),
        emitter_width: (config.as_ref().and_then(|value| value.emitter_width))
            .clone()
            .unwrap_or(0.0_f64),
        frame_count: (config.as_ref().and_then(|value| value.frame_count))
            .clone()
            .unwrap_or(1.0_f64),
        frame_rate: (config.as_ref().and_then(|value| value.frame_rate))
            .clone()
            .unwrap_or(12.0_f64),
        gravity_x: (config.as_ref().and_then(|value| value.gravity_x))
            .clone()
            .unwrap_or(0.0_f64),
        gravity_y: (config.as_ref().and_then(|value| value.gravity_y))
            .clone()
            .unwrap_or(0.0_f64),
        gravity_z: (config.as_ref().and_then(|value| value.gravity_z))
            .clone()
            .unwrap_or(0.0_f64),
        lifetime_max: (config.as_ref().and_then(|value| value.lifetime_max))
            .clone()
            .unwrap_or(1.0_f64),
        lifetime_min: (config.as_ref().and_then(|value| value.lifetime_min))
            .clone()
            .unwrap_or(0.5_f64),
        loop_: (config.as_ref().and_then(|value| value.loop_))
            .clone()
            .unwrap_or(true),
        max_particles: (config.as_ref().and_then(|value| value.max_particles))
            .clone()
            .unwrap_or(1000.0_f64),
        region_id_max: (config.as_ref().and_then(|value| value.region_id_max))
            .clone()
            .unwrap_or(1.0_f64),
        region_id_min: (config.as_ref().and_then(|value| value.region_id_min))
            .clone()
            .unwrap_or(0.0_f64),
        rotation_speed_max: (config.as_ref().and_then(|value| value.rotation_speed_max))
            .clone()
            .unwrap_or(0.0_f64),
        rotation_speed_min: (config.as_ref().and_then(|value| value.rotation_speed_min))
            .clone()
            .unwrap_or(0.0_f64),
        scale_curve: config
            .as_ref()
            .and_then(|value| (value.scale_curve).clone()),
        scale_end: (config.as_ref().and_then(|value| value.scale_end))
            .clone()
            .unwrap_or(1.0_f64),
        scale_max: (config.as_ref().and_then(|value| value.scale_max))
            .clone()
            .unwrap_or(1.0_f64),
        scale_min: (config.as_ref().and_then(|value| value.scale_min))
            .clone()
            .unwrap_or(1.0_f64),
        spawn_rate: (config.as_ref().and_then(|value| value.spawn_rate))
            .clone()
            .unwrap_or(10.0_f64),
        speed_max: (config.as_ref().and_then(|value| value.speed_max))
            .clone()
            .unwrap_or(100.0_f64),
        speed_min: (config.as_ref().and_then(|value| value.speed_min))
            .clone()
            .unwrap_or(50.0_f64),
        spread: (config.as_ref().and_then(|value| value.spread))
            .clone()
            .unwrap_or(std::f64::consts::PI),
        velocity_inheritance: (config.as_ref().and_then(|value| value.velocity_inheritance))
            .clone()
            .unwrap_or(0.0_f64),
        world_space: (config.as_ref().and_then(|value| value.world_space))
            .clone()
            .unwrap_or(false),
    };
}
