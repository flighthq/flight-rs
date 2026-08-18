// @generated from upstream/packages/types/src/SpineParticleSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ParticleEmitterConfig};

// Source: upstream/packages/types/src/SpineParticleSchema.ts:11 (sha256:03261e2b8031f85df8759cc31f71e6309b32b30e780732d4152bdbdd7c2d7e55)
pub type SpineBlendMode = String;

// Source: upstream/packages/types/src/SpineParticleSchema.ts:13 (sha256:580eb2376cbef43767db2b242fc00292c0035c4a64a7e48cfc48b98fc07e7267)
#[derive(Clone, Default)]
pub struct SpineRangeValue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub low: f64,
    pub high: f64,
}
impl PartialEq for SpineRangeValue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineParticleSchema.ts:18 (sha256:b560446723c3ccbb25d3234d3edbccf7c514e1973f47f312510d28273b7d72ba)
#[derive(Clone, Default)]
pub struct SpineAlphaKeyframe {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub alpha: f64,
}
impl PartialEq for SpineAlphaKeyframe {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineParticleSchema.ts:23 (sha256:bc176f837203740be594a5086733b10bd8550182068665f8898458276fc7b2e4)
#[derive(Clone, Default)]
pub struct SpineTintKeyframe {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub color: String,
}
impl PartialEq for SpineTintKeyframe {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineParticleSchema.ts:29 (sha256:e8f4aac054f7c6f4dceb482ba393f069a6ad38e7da1f9a84d806b2c9eae8549f)
#[derive(Clone, Default)]
pub struct SpineParticleDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub max_particles: f64,
    pub continuous: bool,
    pub duration: f64,
    pub emission: SpineRangeValue,
    pub life: SpineRangeValue,
    pub life_offset: SpineRangeValue,
    pub x: SpineRangeValue,
    pub y: SpineRangeValue,
    pub spawn_shape: String,
    pub spawn_width: SpineRangeValue,
    pub spawn_height: SpineRangeValue,
    pub velocity: SpineRangeValue,
    pub angle: SpineRangeValue,
    pub rotation: SpineRangeValue,
    pub wind: SpineRangeValue,
    pub gravity: SpineRangeValue,
    pub scale: SpineRangeValue,
    pub scale_end: SpineRangeValue,
    pub tint: Vec<SpineTintKeyframe>,
    pub alpha: Vec<SpineAlphaKeyframe>,
    pub blend_mode: SpineBlendMode,
    pub premultiplied: bool,
    pub images: Vec<String>,
}
impl PartialEq for SpineParticleDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineParticleSchema.ts:70 (sha256:b2a302ae7e751079717db45f54e36b898cb30675e541d15366a3592ab5d887be)
#[derive(Clone, Default)]
pub struct SpineParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub document: SpineParticleDocument,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for SpineParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
