// @generated from upstream/packages/particleemitter/src/stepParticleEmitter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::update_particle_emitter;
use flighthq_particles::{apply_particle_collisions, apply_particle_forces};
use flighthq_types::{
    ParticleCollider, ParticleEmitter, ParticleEmitterCallbacks, ParticleEmitterConfig,
    ParticleEmitterState, ParticleForce,
};

// Source: upstream/packages/particleemitter/src/stepParticleEmitter.ts:30 (sha256:2552ace02df0016ebf49c031de303632f7185011d04a73d8dc87ecae765d4a3d)
pub fn step_particle_emitter(
    emitter: &mut ParticleEmitter,
    state: &mut ParticleEmitterState,
    config: &ParticleEmitterConfig,
    delta_time: f64,
    forces: Option<Vec<ParticleForce>>,
    colliders: Option<Vec<ParticleCollider>>,
    callbacks: Option<ParticleEmitterCallbacks>,
) -> () {
    if ((forces).is_some()) && ((forces.as_ref().unwrap().len() as f64) > 0.0_f64) {
        apply_particle_forces(emitter, state, forces.as_ref().unwrap(), delta_time);
    }
    update_particle_emitter(
        emitter,
        state,
        config,
        delta_time,
        Some(((callbacks).clone().unwrap()).clone()),
    );
    if ((colliders).is_some()) && ((colliders.as_ref().unwrap().len() as f64) > 0.0_f64) {
        apply_particle_collisions(emitter, state, colliders.as_ref().unwrap());
    }
}
