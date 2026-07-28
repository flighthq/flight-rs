// @generated from upstream/packages/types/src/PathBooleanOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PathWinding;

// Source: upstream/packages/types/src/PathBooleanOptions.ts:8 (sha256:77f27bd1ddb42906bc777d6ec44717fb5c614f61f8351f86314d8f65fdfa8d74)
#[derive(Clone)]
pub struct PathBooleanOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fill_rule: Option<PathWinding>,
    pub tolerance: Option<f64>,
}
impl PartialEq for PathBooleanOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
