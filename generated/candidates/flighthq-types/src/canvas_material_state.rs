// @generated from upstream/packages/types/src/CanvasMaterialState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CanvasMaterialState.ts:10 (sha256:f5ca82625513b77cd226c8943e1b28c78a44186cd6f2c42822439644719c69f8)
#[derive(Clone)]
pub struct CanvasMaterialState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub composite: Option<crate::OpaqueHostValue>,
    pub filter: Option<String>,
}
impl PartialEq for CanvasMaterialState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
