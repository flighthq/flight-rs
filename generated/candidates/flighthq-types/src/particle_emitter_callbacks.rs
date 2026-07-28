// @generated from upstream/packages/types/src/ParticleEmitterCallbacks.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ParticleEmitterCallbacks.ts:1 (sha256:eeb4954eec290115eb7bff6d11084034fd083fd74cdf423b990a028800fd7a7e)
#[derive(Clone, Default)]
pub struct ParticleEmitterCallbacks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_death: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>>>,
    >,
    pub on_spawn: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for ParticleEmitterCallbacks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
