// @generated from upstream/packages/types/src/ParticleEmitterCallbacks.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ParticleEmitterCallbacks.ts:1 (sha256:eeb4954eec290115eb7bff6d11084034fd083fd74cdf423b990a028800fd7a7e)
#[derive(Clone)]
pub struct ParticleEmitterCallbacks {
    pub on_death: Option<std::sync::Arc<dyn Fn(f64, f64, f64) -> () + Send + Sync + 'static>>,
    pub on_spawn: Option<std::sync::Arc<dyn Fn(f64, f64, f64) -> () + Send + Sync + 'static>>,
}
