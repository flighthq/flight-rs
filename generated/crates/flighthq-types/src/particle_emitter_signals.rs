// @generated from upstream/packages/types/src/ParticleEmitterSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/ParticleEmitterSignals.ts:4 (sha256:19efeccee40067090237f52143119b800c54932858ed5009e95ce5f3adb98ee1)
#[derive(Clone)]
pub struct ParticleEmitterSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_particle_spawn: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(f64, f64, f64, f64, f64, f64) -> () + Send + 'static>>,
        >,
    >,
    pub on_particle_death: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>>>,
    >,
    pub on_emitter_complete:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for ParticleEmitterSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ParticleEmitterSignals {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
