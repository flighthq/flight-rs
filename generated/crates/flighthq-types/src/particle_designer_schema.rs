// @generated from upstream/packages/types/src/ParticleDesignerSchema.ts; do not edit.
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

// Source: upstream/packages/types/src/ParticleDesignerSchema.ts:8 (sha256:00bb12659ac76a9ea4b65c2e6822291ce110e34b9b8fc220f5975c4fb3f44c84)
pub type ParticleDesignerEmitterType = f64;

// Source: upstream/packages/types/src/ParticleDesignerSchema.ts:10 (sha256:a0256789ce9fbe4a0bbd0e76473aae1cade9c354fbf61972195043eeceeccce0)
#[derive(Clone, Default)]
pub struct ParticleDesignerDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_particles: f64,
    pub emitter_type: ParticleDesignerEmitterType,
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
    pub start_color_red: f64,
    pub start_color_green: f64,
    pub start_color_blue: f64,
    pub start_color_alpha: f64,
    pub start_color_variance_red: f64,
    pub start_color_variance_green: f64,
    pub start_color_variance_blue: f64,
    pub start_color_variance_alpha: f64,
    pub finish_color_red: f64,
    pub finish_color_green: f64,
    pub finish_color_blue: f64,
    pub finish_color_alpha: f64,
    pub finish_color_variance_red: f64,
    pub finish_color_variance_green: f64,
    pub finish_color_variance_blue: f64,
    pub finish_color_variance_alpha: f64,
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
    pub blend_func_source: f64,
    pub blend_func_destination: f64,
    pub texture_file_name: String,
}
impl PartialEq for ParticleDesignerDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleDesignerSchema.ts:84 (sha256:3b64188eb66fa1f2aa900a0416a8dbaf5046672a24db92d8e6733601be0eaa00)
pub type ParticleDesignerRawDict = Vec<(
    String,
    crate::FlightUnion2<String, crate::FlightUnion2<f64, bool>>,
)>;

// Source: upstream/packages/types/src/ParticleDesignerSchema.ts:86 (sha256:e2f4cd57e80a8216ea0d3bb1fad2f877d8cc841b5c4505b3235c3e44e13a6d51)
#[derive(Clone, Default)]
pub struct ParticleDesignerParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for ParticleDesignerParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleDesignerSchema.ts:92 (sha256:e600e1dda18994ef53183f57c80e2196fb0bc89a9c5da4f15170ebe57cbe25f7)
#[derive(Clone, Default)]
pub struct ParticleDesignerParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub document: ParticleDesignerDocument,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for ParticleDesignerParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleDesignerSchema.ts:100 (sha256:b2e4440760747d9e15b6e3ef46aca01a7d09760db0383e004ec18d289dc95e82)
#[derive(Clone, Default)]
pub struct ParticleDesignerSerializeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for ParticleDesignerSerializeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
