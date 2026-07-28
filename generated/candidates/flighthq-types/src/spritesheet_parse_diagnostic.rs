// @generated from upstream/packages/types/src/SpritesheetParseDiagnostic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SpritesheetParseDiagnostic.ts:1 (sha256:3dd09d2e5fe195ba39ef2f5bbc7eb62fd6689efcba0ba457aed05a3cd9c5407c)
pub type SpritesheetParseDiagnosticSeverity = String;

// Source: upstream/packages/types/src/SpritesheetParseDiagnostic.ts:2 (sha256:a8133944c6332a24d95de37fdef545eba1e984a455e1ed6d411fdd44e848f8bc)
#[derive(Clone, Default)]
pub struct SpritesheetParseDiagnostic {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame_name: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub severity: SpritesheetParseDiagnosticSeverity,
}
impl PartialEq for SpritesheetParseDiagnostic {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
