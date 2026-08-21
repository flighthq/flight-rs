// @generated from upstream/packages/types/src/ImportDiagnostic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ImportDiagnostic.ts:17 (sha256:c060013b06ced444e8694568173367e58d597ed2845be791121f047159a18dc0)
#[derive(Clone, Default)]
pub struct ImportDiagnostic {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detail: Option<
        Vec<(
            String,
            crate::FlightUnion2<bool, crate::FlightUnion2<f64, String>>,
        )>,
    >,
    pub kind: String,
    pub origin: String,
    pub severity: ImportDiagnosticSeverity,
}
impl PartialEq for ImportDiagnostic {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImportDiagnostic.ts:59 (sha256:2a805fe9fefd96b197b13880029abd1788774d174b8799affabc2dae5f384fd3)
#[derive(Clone, Default)]
pub struct ImportDiagnosticSeverityValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub drop: String,
    pub recover: String,
    pub reject: String,
    pub skip: String,
}
impl PartialEq for ImportDiagnosticSeverityValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static IMPORT_DIAGNOSTIC_SEVERITY: std::sync::LazyLock<ImportDiagnosticSeverityValues> =
    std::sync::LazyLock::new(|| ImportDiagnosticSeverityValues {
        __flight_identity: std::sync::Arc::new(()),
        drop: "Drop".to_owned(),
        recover: "Recover".to_owned(),
        reject: "Reject".to_owned(),
        skip: "Skip".to_owned(),
    });

// Source: upstream/packages/types/src/ImportDiagnostic.ts:66 (sha256:1b383c04a4bc548977624f50a529cda34ea6a53fd2280734966ea5e0f8cc941f)
pub type ImportDiagnosticSeverity = String;
