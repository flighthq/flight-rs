// @generated from upstream/packages/particles/src/particleEmitterConfig.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use flighthq_types::{ParticleBlendMode, ParticleEmitterConfig, ParticleEmitterShape};

// Source: upstream/packages/particles/src/particleEmitterConfig.ts:5 (sha256:abb2ffb0f022c9535292c6b263f485ec1255b9f3d59984a190504d18e5894724)
pub fn create_particle_emitter_config(
    config: Option<ParticleEmitterConfig>,
) -> ParticleEmitterConfig {
    return ParticleEmitterConfig {
        __flight_identity: std::sync::Arc::new(()),
        alpha_curve: config
            .as_ref()
            .and_then(|value| (value.alpha_curve).clone()),
        alpha_end: (config.as_ref().map(|value| value.alpha_end)).unwrap_or(0.0_f64),
        alpha_start: (config.as_ref().map(|value| value.alpha_start)).unwrap_or(1.0_f64),
        blend_mode: config.as_ref().and_then(|value| (value.blend_mode).clone()),
        burst_count: (config.as_ref().map(|value| value.burst_count)).unwrap_or(0.0_f64),
        burst_interval: (config.as_ref().map(|value| value.burst_interval)).unwrap_or(0.0_f64),
        color_curve: config
            .as_ref()
            .and_then(|value| (value.color_curve).clone()),
        color_end_b: (config.as_ref().map(|value| value.color_end_b)).unwrap_or(1.0_f64),
        color_end_g: (config.as_ref().map(|value| value.color_end_g)).unwrap_or(1.0_f64),
        color_end_r: (config.as_ref().map(|value| value.color_end_r)).unwrap_or(1.0_f64),
        color_end_variance_b: (config.as_ref().map(|value| value.color_end_variance_b))
            .unwrap_or(0.0_f64),
        color_end_variance_g: (config.as_ref().map(|value| value.color_end_variance_g))
            .unwrap_or(0.0_f64),
        color_end_variance_r: (config.as_ref().map(|value| value.color_end_variance_r))
            .unwrap_or(0.0_f64),
        color_start_b: (config.as_ref().map(|value| value.color_start_b)).unwrap_or(1.0_f64),
        color_start_g: (config.as_ref().map(|value| value.color_start_g)).unwrap_or(1.0_f64),
        color_start_r: (config.as_ref().map(|value| value.color_start_r)).unwrap_or(1.0_f64),
        color_start_variance_b: (config.as_ref().map(|value| value.color_start_variance_b))
            .unwrap_or(0.0_f64),
        color_start_variance_g: (config.as_ref().map(|value| value.color_start_variance_g))
            .unwrap_or(0.0_f64),
        color_start_variance_r: (config.as_ref().map(|value| value.color_start_variance_r))
            .unwrap_or(0.0_f64),
        direction_x: (config.as_ref().map(|value| value.direction_x)).unwrap_or(0.0_f64),
        direction_y: (config.as_ref().map(|value| value.direction_y)).unwrap_or((-1.0_f64)),
        direction_z: (config.as_ref().map(|value| value.direction_z)).unwrap_or(0.0_f64),
        duration: (config.as_ref().map(|value| value.duration)).unwrap_or(0.0_f64),
        emitter_cone_angle: (config.as_ref().map(|value| value.emitter_cone_angle))
            .unwrap_or(0.0_f64),
        emitter_depth: (config.as_ref().map(|value| value.emitter_depth)).unwrap_or(0.0_f64),
        emitter_height: (config.as_ref().map(|value| value.emitter_height)).unwrap_or(0.0_f64),
        emitter_radius: (config.as_ref().map(|value| value.emitter_radius)).unwrap_or(0.0_f64),
        emitter_shape: (config.as_ref().map(|value| (value.emitter_shape).clone()))
            .unwrap_or("point".to_owned()),
        emitter_width: (config.as_ref().map(|value| value.emitter_width)).unwrap_or(0.0_f64),
        frame_count: (config.as_ref().map(|value| value.frame_count)).unwrap_or(1.0_f64),
        frame_rate: (config.as_ref().map(|value| value.frame_rate)).unwrap_or(12.0_f64),
        gravity_x: (config.as_ref().map(|value| value.gravity_x)).unwrap_or(0.0_f64),
        gravity_y: (config.as_ref().map(|value| value.gravity_y)).unwrap_or(0.0_f64),
        gravity_z: (config.as_ref().map(|value| value.gravity_z)).unwrap_or(0.0_f64),
        lifetime_max: (config.as_ref().map(|value| value.lifetime_max)).unwrap_or(1.0_f64),
        lifetime_min: (config.as_ref().map(|value| value.lifetime_min)).unwrap_or(0.5_f64),
        loop_: (config.as_ref().map(|value| value.loop_)).unwrap_or(true),
        max_particles: (config.as_ref().map(|value| value.max_particles)).unwrap_or(1000.0_f64),
        region_id_max: (config.as_ref().map(|value| value.region_id_max)).unwrap_or(1.0_f64),
        region_id_min: (config.as_ref().map(|value| value.region_id_min)).unwrap_or(0.0_f64),
        rotation_speed_max: (config.as_ref().map(|value| value.rotation_speed_max))
            .unwrap_or(0.0_f64),
        rotation_speed_min: (config.as_ref().map(|value| value.rotation_speed_min))
            .unwrap_or(0.0_f64),
        scale_curve: config
            .as_ref()
            .and_then(|value| (value.scale_curve).clone()),
        scale_end: (config.as_ref().map(|value| value.scale_end)).unwrap_or(1.0_f64),
        scale_max: (config.as_ref().map(|value| value.scale_max)).unwrap_or(1.0_f64),
        scale_min: (config.as_ref().map(|value| value.scale_min)).unwrap_or(1.0_f64),
        spawn_rate: (config.as_ref().map(|value| value.spawn_rate)).unwrap_or(10.0_f64),
        speed_max: (config.as_ref().map(|value| value.speed_max)).unwrap_or(100.0_f64),
        speed_min: (config.as_ref().map(|value| value.speed_min)).unwrap_or(50.0_f64),
        spread: (config.as_ref().map(|value| value.spread)).unwrap_or(std::f64::consts::PI),
        velocity_inheritance: (config.as_ref().map(|value| value.velocity_inheritance))
            .unwrap_or(0.0_f64),
        world_space: (config.as_ref().map(|value| value.world_space)).unwrap_or(false),
    };
}
