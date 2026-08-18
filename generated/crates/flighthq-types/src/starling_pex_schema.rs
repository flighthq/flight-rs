// @generated from upstream/packages/types/src/StarlingPexSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ParticleEmitterConfig};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingPexSchema.ts:18 (sha256:d10f6382bd2953e8f9b803835db55522612f4d469c538cb12ab46977d83c4af8)
#[derive(Clone, Default)]
pub struct StarlingPexColor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}
impl PartialEq for StarlingPexColor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingPexSchema.ts:26 (sha256:59aace6cf3340aaeaf9a3753fe20e98387acd556acd1a5f9a91297ac243f8890)
#[derive(Clone, Default)]
pub struct StarlingPexDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_particles: f64,
    pub emitter_type: f64,
    pub duration: f64,
    pub particle_lifespan: f64,
    pub particle_lifespan_variance: f64,
    pub speed: f64,
    pub speed_variance: f64,
    pub angle: f64,
    pub angle_variance: f64,
    pub gravityx: f64,
    pub gravityy: f64,
    pub source_position_variancex: f64,
    pub source_position_variancey: f64,
    pub start_particle_size: f64,
    pub start_particle_size_variance: f64,
    pub finish_particle_size: f64,
    pub finish_particle_size_variance: f64,
    pub start_color: StarlingPexColor,
    pub start_color_variance: StarlingPexColor,
    pub finish_color: StarlingPexColor,
    pub finish_color_variance: StarlingPexColor,
    pub rotation_start: f64,
    pub rotation_start_variance: f64,
    pub rotation_end: f64,
    pub rotation_end_variance: f64,
    pub max_radius: f64,
    pub max_radius_variance: f64,
    pub min_radius: f64,
    pub min_radius_variance: f64,
    pub rotate_per_second: f64,
    pub rotate_per_second_variance: f64,
    pub radial_acceleration: f64,
    pub radial_accel_variance: f64,
    pub tangential_acceleration: f64,
    pub tangential_accel_variance: f64,
    pub blend_func_source: f64,
    pub blend_func_destination: f64,
    pub texture_file_name: String,
}
impl PartialEq for StarlingPexDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingPexSchema.ts:68 (sha256:c8bdb0a99c7c043e7de25a7f99999c3b026377eaaa17a0e3ed5415ab0ddda6f9)
#[derive(Clone, Default)]
pub struct StarlingPexParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for StarlingPexParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingPexSchema.ts:74 (sha256:39d95cf5b33ff0f5320156d09cbaffba390a4fffe9e02ad96d1e337a68049d23)
#[derive(Clone, Default)]
pub struct StarlingPexParseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub document: StarlingPexDocument,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for StarlingPexParseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingPexSchema.ts:82 (sha256:09e4448fd1f0a3cb26193d39dab78003e1698a40d8957dbac3415befe098183f)
pub type StarlingPexParsed = StarlingPexParseResult;

// Source: upstream/packages/types/src/StarlingPexSchema.ts:84 (sha256:d2faeb92a4c6bd1c583f1068ebf9425db42469f4299c8cf7ea8f5beaf2528e63)
#[derive(Clone, Default)]
pub struct StarlingPexSerializeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for StarlingPexSerializeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
