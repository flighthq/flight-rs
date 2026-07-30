// @generated from upstream/packages/types/src/ParticleObjectsState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RandomSource;

// Source: upstream/packages/types/src/ParticleObjectsState.ts:3 (sha256:e4464e1605aa4611f053b8b6fac5a0d4784aca308b7912611ad313c2168ce176)
#[derive(Clone)]
pub struct ParticleObjectsState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub burst_timer: f64,
    pub emitter_age: f64,
    pub lifetimes: Vec<f32>,
    pub prev_x: f64,
    pub prev_y: f64,
    pub random: RandomSource,
    pub rotation_speeds: Vec<f32>,
    pub scales: Vec<f32>,
    pub spawn_accumulator: f64,
    pub velocities: Vec<f32>,
}
impl PartialEq for ParticleObjectsState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
