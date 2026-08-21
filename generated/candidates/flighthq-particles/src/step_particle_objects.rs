// @generated from upstream/packages/particles/src/stepParticleObjects.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    apply_particle_object_collisions, apply_particle_object_forces, update_particle_objects,
};
use flighthq_types::{
    ParticleCollider, ParticleEmitterConfig, ParticleForce, ParticleObject, ParticleObjectsState,
    ParticleObjectsUpdateOptions,
};

// Source: upstream/packages/particles/src/stepParticleObjects.ts:30 (sha256:85e37ca99f593e787b716691e0414d2e8c81b4846657fc7f824989879a9a613f)
pub fn step_particle_objects(
    objects: &mut Vec<ParticleObject>,
    state: &mut ParticleObjectsState,
    config: &ParticleEmitterConfig,
    delta_time: f64,
    forces: Option<Vec<ParticleForce>>,
    colliders: Option<Vec<ParticleCollider>>,
    update_options: Option<ParticleObjectsUpdateOptions>,
) -> () {
    if ((forces).is_some()) && ((forces.as_ref().unwrap().len() as f64) > 0.0_f64) {
        apply_particle_object_forces(objects, state, &forces.as_ref().unwrap(), delta_time);
    }
    update_particle_objects(
        objects,
        state,
        config,
        delta_time,
        ((update_options).clone()).clone(),
    );
    if ((colliders).is_some()) && ((colliders.as_ref().unwrap().len() as f64) > 0.0_f64) {
        apply_particle_object_collisions(objects, state, &colliders.as_ref().unwrap());
    }
}
