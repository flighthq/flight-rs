// @generated from upstream/packages/types/src/ParticleEmitterSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ParticleEmitterSignals.ts:3 (sha256:3c768de70494ffbe169bc60a6f38e149044a01f8f96c7742f74424a768bc6834)
#[derive(Clone)]
pub struct ParticleEmitterSignals {
    pub on_particle_spawn: Signal,
    pub on_particle_death: Signal,
    pub on_emitter_complete: Signal,
}
