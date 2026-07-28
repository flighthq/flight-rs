// @generated from upstream/packages/types/src/AnimationTrackValidationDiagnostic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AnimationTrackValidationDiagnostic.ts:4 (sha256:6e162ff26c996de3d7920242456c5993a64fc6046f3f5a54f9e7caac530dd16c)
pub type AnimationTrackValidationCode = String;

// Source: upstream/packages/types/src/AnimationTrackValidationDiagnostic.ts:8 (sha256:f69ccacd70f068caceeaabfaa792b8a1e9ccb6913c87a4cea6f9322515abb218)
#[derive(Clone)]
pub struct AnimationTrackValidationDiagnostic {
    pub code: AnimationTrackValidationCode,
    pub index: Option<f64>,
    pub message: String,
}
