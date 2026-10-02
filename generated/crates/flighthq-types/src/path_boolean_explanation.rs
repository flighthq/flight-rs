// @generated from upstream/packages/types/src/PathBooleanExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PathBooleanExplanation.ts:1 (sha256:8fb0de3cf5580d6fe2cae7e4814b96e3d0a1e2078e8c3353597de587aa33a0da)
#[derive(Clone, Default)]
pub struct PathBooleanExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
    pub reason: String,
}
impl PartialEq for PathBooleanExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
