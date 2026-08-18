// @generated from upstream/packages/types/src/AnimationTrackValidationDiagnostic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AnimationTrackValidationDiagnostic.ts:4 (sha256:b99c64d082039e5401a40fb8746c41ad24fb0092dc6751a6e12562534becdc27)
pub type AnimationTrackValidationCode = String;

// Source: upstream/packages/types/src/AnimationTrackValidationDiagnostic.ts:11 (sha256:f69ccacd70f068caceeaabfaa792b8a1e9ccb6913c87a4cea6f9322515abb218)
#[derive(Clone, Default)]
pub struct AnimationTrackValidationDiagnostic {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub code: AnimationTrackValidationCode,
    pub index: Option<f64>,
    pub message: String,
}
impl PartialEq for AnimationTrackValidationDiagnostic {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
