// @generated from upstream/packages/types/src/ParticleConfigIssue.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ParticleEmitterConfig;

// Source: upstream/packages/types/src/ParticleConfigIssue.ts:3 (sha256:870f81d1bcca90743407102466af90fae576da09798c1d82c808d7d4aa3c0d0b)
#[derive(Clone)]
pub struct ParticleConfigIssue {
    pub field: ParticleEmitterConfig,
    pub message: String,
    pub severity: String,
}
