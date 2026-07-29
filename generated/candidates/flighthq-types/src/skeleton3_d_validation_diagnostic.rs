// @generated from upstream/packages/types/src/Skeleton3DValidationDiagnostic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton3DValidationDiagnostic.ts:2 (sha256:41bab638d37fdc7e7496947a4605fcb02182c229ff3592074b2c4751f416b6de)
#[derive(Clone, Default)]
pub struct Skeleton3DValidationDiagnostic {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joint_count: f64,
    pub inverse_bind_matrices_length: f64,
    pub expected_inverse_bind_matrices_length: f64,
    pub message: String,
}
impl PartialEq for Skeleton3DValidationDiagnostic {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
