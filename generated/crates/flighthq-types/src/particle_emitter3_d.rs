// @generated from upstream/packages/types/src/ParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, ParticleBlendMode, ParticleEmitterData, Quaternion, Vector3};

// Source: upstream/packages/types/src/ParticleEmitter3D.ts:5 (sha256:64e20d991efa3af3e4d7ea369d2494215759ec7b97040fd164291220452e4e3d)
#[derive(Clone, Default)]
pub struct ParticleEmitter3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub data: ParticleEmitterData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub blend_mode: ParticleBlendMode,
}
impl PartialEq for ParticleEmitter3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ParticleEmitter3D {
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

// Source: upstream/packages/types/src/ParticleEmitter3D.ts:13 (sha256:5b5854dee4462374b30dabf5c0b3dc5494051b3627707ccfc7970c684fb8cac0)
pub type ParticleEmitter3DRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/ParticleEmitter3D.ts:15 (sha256:353010cff3c09a3d101a72c968869edbda5e245192be162cece34fa3828f8ac1)
pub const PARTICLE_EMITTER3_D_KIND: &'static str = "ParticleEmitter3D";
