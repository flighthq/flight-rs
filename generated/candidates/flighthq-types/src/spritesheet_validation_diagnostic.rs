// @generated from upstream/packages/types/src/SpritesheetValidationDiagnostic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SpritesheetValidationDiagnostic.ts:2 (sha256:e54e32106530eb14567ea22c4ac8c0bfbabfdeee8bedca29d8e9ccba79c5a145)
pub type SpritesheetValidationSeverity = String;

// Source: upstream/packages/types/src/SpritesheetValidationDiagnostic.ts:4 (sha256:e2b5727a7ab4aa6a76e63867100facd11a5c6ad25e7d145c0f5a7e9ebcaee080)
#[derive(Clone)]
pub struct SpritesheetValidationDiagnostic {
    pub animation_name: Option<String>,
    pub frame_index: Option<f64>,
    pub message: String,
    pub severity: SpritesheetValidationSeverity,
}
