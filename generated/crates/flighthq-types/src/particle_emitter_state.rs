// @generated from upstream/packages/types/src/ParticleEmitterState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, RandomSource};

// Source: upstream/packages/types/src/ParticleEmitterState.ts:4 (sha256:9df8bc05d6faa8049cd472435ee402c41d55a81d33b78939aaf9df009baf15bf)
#[derive(Clone)]
pub struct ParticleEmitterState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub burst_timer: f64,
    pub color_birth: Vec<f32>,
    pub color_death: Vec<f32>,
    pub emitter_age: f64,
    pub lifetimes: Vec<f32>,
    pub prev_x: f64,
    pub prev_y: f64,
    pub prev_z: f64,
    pub random: RandomSource,
    pub rotation_speeds: Vec<f32>,
    pub scales: Vec<f32>,
    pub spawn_accumulator: f64,
    pub velocities: Vec<f32>,
}
impl PartialEq for ParticleEmitterState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ParticleEmitterState {
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
