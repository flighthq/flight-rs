// @generated from upstream/packages/types/src/ParticleFormatDescriptor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ParticleFormatCodec, ParticleFormatKind};

// Source: upstream/packages/types/src/ParticleFormatDescriptor.ts:14 (sha256:3aabeee7ea251ad1bdb11bd02405f809f3b8cf489174735b562529faf77f83a7)
#[derive(Clone)]
pub struct ParticleFormatDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub codec: ParticleFormatCodec,
    pub kind: ParticleFormatKind,
}
impl PartialEq for ParticleFormatDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleFormatDescriptor.ts:39 (sha256:c44e974e742a8f6ed497269c3ace1c5509e4aeebda888d265153985e13a2ebf7)
#[derive(Clone, Default)]
pub struct ParticleImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub formats: Option<Vec<ParticleFormatDescriptor>>,
}
impl PartialEq for ParticleImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
