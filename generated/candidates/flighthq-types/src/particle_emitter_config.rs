// @generated from upstream/packages/types/src/ParticleEmitterConfig.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ParticleCurve;

// Source: upstream/packages/types/src/ParticleEmitterConfig.ts:3 (sha256:e864c5a7607ed38331bac348ec0f16af4014e8d8c8f1d74c772449c048cbbfb8)
pub type ParticleEmitterShape = String;

// Source: upstream/packages/types/src/ParticleEmitterConfig.ts:8 (sha256:d74fd935c98480e89ab1c4ce86e82a0a53176feb88d6cf069d384f41a5f355c5)
pub type ParticleBlendMode = String;

// Source: upstream/packages/types/src/ParticleEmitterConfig.ts:10 (sha256:cf4bc7901c43bc748e5284248a651f745819fc095bdc6b09ccf7134d7f42f41f)
#[derive(Clone)]
pub struct ParticleEmitterConfig {
    pub alpha_end: f64,
    pub alpha_start: f64,
    pub blend_mode: Option<ParticleBlendMode>,
    pub color_end_b: f64,
    pub color_end_g: f64,
    pub color_end_r: f64,
    pub color_end_variance_b: f64,
    pub color_end_variance_g: f64,
    pub color_end_variance_r: f64,
    pub color_start_b: f64,
    pub color_start_g: f64,
    pub color_start_r: f64,
    pub color_start_variance_b: f64,
    pub color_start_variance_g: f64,
    pub color_start_variance_r: f64,
    pub direction_x: f64,
    pub direction_y: f64,
    pub direction_z: f64,
    pub gravity_x: f64,
    pub gravity_y: f64,
    pub gravity_z: f64,
    pub emitter_cone_angle: f64,
    pub emitter_depth: f64,
    pub emitter_height: f64,
    pub emitter_radius: f64,
    pub emitter_shape: ParticleEmitterShape,
    pub emitter_width: f64,
    pub burst_count: f64,
    pub burst_interval: f64,
    pub duration: f64,
    pub loop_: bool,
    pub frame_count: f64,
    pub frame_rate: f64,
    pub lifetime_max: f64,
    pub lifetime_min: f64,
    pub max_particles: f64,
    pub region_id_max: f64,
    pub region_id_min: f64,
    pub scale_end: f64,
    pub scale_max: f64,
    pub scale_min: f64,
    pub speed_max: f64,
    pub speed_min: f64,
    pub spawn_rate: f64,
    pub spread: f64,
    pub rotation_speed_max: f64,
    pub rotation_speed_min: f64,
    pub velocity_inheritance: f64,
    pub alpha_curve: Option<ParticleCurve>,
    pub color_curve: Option<ParticleCurve>,
    pub scale_curve: Option<ParticleCurve>,
    pub world_space: bool,
}
