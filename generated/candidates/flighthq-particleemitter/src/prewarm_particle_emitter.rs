// @generated from upstream/packages/particleemitter/src/prewarmParticleEmitter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ParticleEmitterCallbacks, update_particle_emitter};
use flighthq_types::{
    ParticleEmitter, ParticleEmitterCallbacks, ParticleEmitterConfig, ParticleEmitterState,
};

// Source: upstream/packages/particleemitter/src/prewarmParticleEmitter.ts:7 (sha256:9d1cee92bb1281ac15c00cdd5a332172f84404e374a4519c09f0132333c2d774)
pub fn prewarm_particle_emitter(
    emitter: &mut ParticleEmitter,
    state: &mut ParticleEmitterState,
    config: &ParticleEmitterConfig,
    duration: f64,
    step_delta_time: Option<crate::OpaqueHostValue>,
    callbacks: Option<ParticleEmitterCallbacks>,
) -> () {
    let step_delta_time = step_delta_time.unwrap_or((1.0_f64 / 60.0_f64));
    let step = if (step_delta_time > 0.0_f64) {
        step_delta_time
    } else {
        duration
    };
    let mut elapsed = 0.0_f64;
    while (elapsed < duration) {
        let delta_time = (step).min((duration - elapsed));
        update_particle_emitter(
            emitter,
            state,
            config,
            delta_time,
            Some(((callbacks).clone().unwrap()).clone()),
        );
        elapsed += delta_time;
    }
}
