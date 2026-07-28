// @generated from upstream/packages/types/src/ParticleEmitterState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RandomSource;

// Source: upstream/packages/types/src/ParticleEmitterState.ts:3 (sha256:d13698591a3233b07f3ae68c34fdbfbec041a1db7189a04e62551a146fb7d05b)
#[derive(Clone)]
pub struct ParticleEmitterState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub burst_timer: f64,
    pub color_birth: Vec<f32>,
    pub color_death: Vec<f32>,
    pub emitter_age: f64,
    pub lifetimes: Vec<f32>,
    pub prev_x: f64,
    pub prev_y: f64,
    pub prev_z: f64,
    pub random: RandomSource,
    pub rotation_speeds: Vec<f32>,
    pub scales: Vec<f32>,
    pub spawn_accumulator: f64,
    pub velocities: Vec<f32>,
}
impl PartialEq for ParticleEmitterState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
