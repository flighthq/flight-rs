// @generated from upstream/packages/types/src/ParticleEmitter2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, TextureAtlas};

// Source: upstream/packages/types/src/ParticleEmitter2D.ts:5 (sha256:56265a2a70d16090bfde55d92b3f1b50be63638030ea3492c5e2dcbc857b32bd)
#[derive(Clone, Default)]
pub struct ParticleEmitterData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alphas: Vec<f32>,
    pub atlas: Option<TextureAtlas>,
    pub colors: Vec<f32>,
    pub ids: Vec<u16>,
    pub particle_count: f64,
    pub positions_z: Vec<f32>,
    pub transforms: Vec<f32>,
    pub velocities: Vec<f32>,
    pub world_space: bool,
}
impl PartialEq for ParticleEmitterData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleEmitter2D.ts:22 (sha256:deb108f7b92dc39b0f03a09d21db4e466504522b2e7c420ab7e97e51463e7592)
pub type ParticleEmitter2DRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/ParticleEmitter2D.ts:26 (sha256:a61f0a0f982eecc4ba94071ce9c3edd69b89f801d9daac15cffde11834e67726)
#[derive(Clone, Default)]
pub struct ParticleEmitter2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: ParticleEmitterData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub blend_mode: Option<BlendMode>,
    pub clip: Option<ClipRegion>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for ParticleEmitter2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ParticleEmitter2D {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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

// Source: upstream/packages/types/src/ParticleEmitter2D.ts:30 (sha256:3341c9d1575585841089cb5d7400f3f950d5b0c9d5188ef9e9bdf3826a4b3b4f)
pub const PARTICLE_EMITTER2_D_KIND: &'static str = "ParticleEmitter2D";
