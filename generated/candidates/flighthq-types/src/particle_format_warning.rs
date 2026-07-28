// @generated from upstream/packages/types/src/ParticleFormatWarning.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ParticleFormatKind;

// Source: upstream/packages/types/src/ParticleFormatWarning.ts:5 (sha256:e17a40d9745f51eaa861cde213c2a2d40161bad467ebc6f22d74c520ca586e46)
#[derive(Clone)]
pub struct ParticleFormatWarning {
    pub format: ParticleFormatKind,
    pub code: String,
    pub message: String,
}
