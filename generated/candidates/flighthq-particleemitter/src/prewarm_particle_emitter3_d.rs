// @generated from upstream/packages/particleemitter/src/prewarmParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::update_particle_emitter3_d;
use flighthq_types::{
    ParticleEmitter3D, ParticleEmitterCallbacks, ParticleEmitterConfig, ParticleEmitterState,
};

// Source: upstream/packages/particleemitter/src/prewarmParticleEmitter3D.ts:6 (sha256:1bb704aae99e152b82157f2adc7e7af35845c38e50bcf648737695a95ed7d49d)
pub fn prewarm_particle_emitter3_d(
    emitter: &mut ParticleEmitter3D,
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
        update_particle_emitter3_d(
            emitter,
            state,
            config,
            delta_time,
            Some(((callbacks).clone().unwrap()).clone()),
        );
        elapsed += delta_time;
    }
}
