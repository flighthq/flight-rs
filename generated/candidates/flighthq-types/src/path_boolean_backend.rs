// @generated from upstream/packages/types/src/PathBooleanBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PathBooleanBackend.ts:8 (sha256:737e2eb8477a25517503aae5b080064c4503f74fc32141eb2f993973d6aa47ee)
pub type PathBooleanContour = Vec<f64>;

// Source: upstream/packages/types/src/PathBooleanBackend.ts:18 (sha256:5eda3fc050baadf42efa384817a1db51a9192f1f65c7296c0e7665753edbabfe)
#[derive(Clone)]
pub struct PathBooleanBackend {
    pub compute_path_boolean: crate::OpaqueHostValue,
}
