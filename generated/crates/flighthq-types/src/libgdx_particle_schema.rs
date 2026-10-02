// @generated from upstream/packages/types/src/LibgdxParticleSchema.ts; do not edit.
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

// Source: upstream/packages/types/src/LibgdxParticleSchema.ts:16 (sha256:362411d11a4d8f732e24882e04deff53fa42258d0f3e8a6f3ce66e137b3718d9)
#[derive(Clone, Default)]
pub struct LibgdxRangeValue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub low_min: f64,
    pub low_max: f64,
    pub high_min: f64,
    pub high_max: f64,
    pub relative: bool,
    pub scaling: Vec<f64>,
    pub timeline: Vec<f64>,
}
impl PartialEq for LibgdxRangeValue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxParticleSchema.ts:28 (sha256:61f1753e181d3e63089f97d47e2008b4ff2b4002f2011a04527d7b0458c82574)
#[derive(Clone, Default)]
pub struct LibgdxActiveRangeValue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub low_min: f64,
    pub low_max: f64,
    pub high_min: f64,
    pub high_max: f64,
    pub relative: bool,
    pub scaling: Vec<f64>,
    pub timeline: Vec<f64>,
    pub active: bool,
}
impl PartialEq for LibgdxActiveRangeValue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxParticleSchema.ts:33 (sha256:27874e07c64cca82920bf2390d3c3ae323ad2816cccccebb6e4c676634c8dbc5)
#[derive(Clone, Default)]
pub struct LibgdxParticleDocumentRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub colors: Vec<String>,
    pub timeline: Vec<f64>,
}
impl PartialEq for LibgdxParticleDocumentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LibgdxParticleDocumentRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub shape: String,
    pub edges: bool,
    pub side: String,
}
impl PartialEq for LibgdxParticleDocumentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LibgdxParticleDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub min_particle_count: f64,
    pub max_particle_count: f64,
    pub additive: bool,
    pub premultiplied_alpha: bool,
    pub delay: LibgdxActiveRangeValue,
    pub duration: LibgdxRangeValue,
    pub emission: LibgdxRangeValue,
    pub life: LibgdxRangeValue,
    pub life_offset: LibgdxActiveRangeValue,
    pub x_offset: LibgdxActiveRangeValue,
    pub y_offset: LibgdxActiveRangeValue,
    pub spawn_shape: LibgdxParticleDocumentRecord2,
    pub spawn_width: LibgdxRangeValue,
    pub spawn_height: LibgdxRangeValue,
    pub scale: LibgdxRangeValue,
    pub velocity: LibgdxActiveRangeValue,
    pub angle: LibgdxActiveRangeValue,
    pub rotation: LibgdxActiveRangeValue,
    pub wind: LibgdxActiveRangeValue,
    pub gravity: LibgdxActiveRangeValue,
    pub tint: LibgdxParticleDocumentRecord1,
    pub transparency: LibgdxRangeValue,
    pub image_count: f64,
    pub image_path: String,
}
impl PartialEq for LibgdxParticleDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxParticleSchema.ts:70 (sha256:910fccf72e4a84df347e92697e47faacf0fbb046745aee5312fea12639de17f3)
#[derive(Clone, Default)]
pub struct LibgdxParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for LibgdxParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxParticleSchema.ts:76 (sha256:1594badef97bb900fd15a07ee2e1bd2361bb8895f1bd0d49d4e0c5649a43dac2)
#[derive(Clone, Default)]
pub struct LibgdxParseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub document: LibgdxParticleDocument,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for LibgdxParseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxParticleSchema.ts:84 (sha256:cf4e592c67bce2ca2973f64cc9a76a19172e029e4fd81e6ab5354e9dfba8cfbb)
#[derive(Clone, Default)]
pub struct LibgdxSerializeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
}
impl PartialEq for LibgdxSerializeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
