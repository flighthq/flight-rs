// @generated from upstream/packages/types/src/ParticleFormatWarning.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ParticleFormatKind;

// Source: upstream/packages/types/src/ParticleFormatWarning.ts:5 (sha256:e17a40d9745f51eaa861cde213c2a2d40161bad467ebc6f22d74c520ca586e46)
#[derive(Clone, Default)]
pub struct ParticleFormatWarning {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: ParticleFormatKind,
    pub code: String,
    pub message: String,
}
impl PartialEq for ParticleFormatWarning {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
