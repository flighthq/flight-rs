// @generated from upstream/packages/types/src/ParticleFormatCodec.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ParticleEmitterConfig, ParticleSerializeResult};

// Source: upstream/packages/types/src/ParticleFormatCodec.ts:5 (sha256:d47fe92f7235d26926b010d0ac005f2504b005cb6c402a62e8774852c2bf5776)
#[derive(Clone, Default)]
pub struct ParticleFormatCodecRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for ParticleFormatCodecRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ParticleFormatCodec {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub parse_to_config: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> ParticleEmitterConfig + Send + 'static>>,
    >,
    pub parse_to_document: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> ParticleFormatCodecRecord1 + Send + 'static>>,
    >,
    pub serialize: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ParticleEmitterConfig) -> ParticleSerializeResult + Send + 'static>,
        >,
    >,
}
impl PartialEq for ParticleFormatCodec {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
