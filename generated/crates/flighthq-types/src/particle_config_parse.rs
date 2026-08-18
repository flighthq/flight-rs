// @generated from upstream/packages/types/src/ParticleConfigParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ParticleEmitterConfig};

// Source: upstream/packages/types/src/ParticleConfigParse.ts:8 (sha256:ed6280081a404eba4fad504c5ca4d1e11c22e7b56059074856c88f5ee6a65b2d)
#[derive(Clone, Default)]
pub struct ParseParticleConfigOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture_size: Option<f64>,
    pub pixels_per_unit: Option<f64>,
}
impl PartialEq for ParseParticleConfigOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleConfigParse.ts:11 (sha256:cf586ff3a1651856e5f9d7e20e518de5ade5c79d21d0c84c351dc30c9259e3cb)
#[derive(Clone, Default)]
pub struct ParticleConfigParseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub diagnostics: Vec<ImportDiagnostic>,
    pub format: Option<String>,
}
impl PartialEq for ParticleConfigParseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
