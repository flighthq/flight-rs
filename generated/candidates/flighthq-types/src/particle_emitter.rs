// @generated from upstream/packages/types/src/ParticleEmitter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, TextureAtlas};

// Source: upstream/packages/types/src/ParticleEmitter.ts:5 (sha256:9c6f0dcedb3a91c11fff28e45f21e2ee263d55c2d2364234aab4c53af2fdf631)
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

// Source: upstream/packages/types/src/ParticleEmitter.ts:22 (sha256:cfd23ad8d6d6473467d64b0b8abfdde50b7323073b7827338c806c5a583a6809)
pub type ParticleEmitterRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/ParticleEmitter.ts:26 (sha256:47bb56e88c7034c21aa5566b2b4795a62133103582b000b558fd6de0ba71275b)
#[derive(Clone, Default)]
pub struct ParticleEmitter {
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
impl PartialEq for ParticleEmitter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ParticleEmitter {
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

// Source: upstream/packages/types/src/ParticleEmitter.ts:30 (sha256:5da0b11f26d26a0b2b699b53ec2492deb25fbb5594940e68f3395aabf10aca7a)
pub const PARTICLE_EMITTER_KIND: &'static str = "ParticleEmitter";
