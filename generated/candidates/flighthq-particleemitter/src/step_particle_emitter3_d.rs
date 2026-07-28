// @generated from upstream/packages/particleemitter/src/stepParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::update_particle_emitter3_d;
use flighthq_particles::{apply_particle_collisions, apply_particle_forces};
use flighthq_types::{
    ParticleCollider, ParticleEmitter3D, ParticleEmitterCallbacks, ParticleEmitterConfig,
    ParticleEmitterState, ParticleForce,
};

// Source: upstream/packages/particleemitter/src/stepParticleEmitter3D.ts:14 (sha256:656f24a6ba2da83f96c49c0d6ae5057b6ae7dd7578a8544d53e73a252c272515)
pub fn step_particle_emitter3_d(
    emitter: &mut ParticleEmitter3D,
    state: &mut ParticleEmitterState,
    config: &ParticleEmitterConfig,
    delta_time: f64,
    forces: Option<Vec<ParticleForce>>,
    colliders: Option<Vec<ParticleCollider>>,
    callbacks: Option<ParticleEmitterCallbacks>,
) -> () {
    let mut as_emitter = emitter;
    if ((forces).is_some() && ((forces.as_ref().unwrap().len() as f64) > 0.0_f64)) {
        apply_particle_forces(&as_emitter, state, forces.as_ref().unwrap(), delta_time);
    }
    update_particle_emitter3_d(
        emitter,
        state,
        config,
        delta_time,
        Some(((callbacks).clone().unwrap()).clone()),
    );
    if ((colliders).is_some() && ((colliders.as_ref().unwrap().len() as f64) > 0.0_f64)) {
        apply_particle_collisions(&mut as_emitter, state, colliders.as_ref().unwrap());
    }
}
