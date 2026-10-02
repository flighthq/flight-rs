// @generated from upstream/packages/types/src/RawProjectionOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Matrix4Like;

// Source: upstream/packages/types/src/RawProjectionOptions.ts:4 (sha256:2b86d3d4f744a713046ab9893b48879c4ac74c146d7da6188f2424e052a75d42)
#[derive(Clone, Default)]
pub struct RawProjectionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub matrix: Matrix4Like,
}
impl PartialEq for RawProjectionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
