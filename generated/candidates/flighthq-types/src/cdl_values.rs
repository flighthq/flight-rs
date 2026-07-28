// @generated from upstream/packages/types/src/CdlValues.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CdlValues.ts:1 (sha256:047556e1bd2714b7eba4d25c2d396bdaa84aff6c40f6623e0d58b8a6ea6d7515)
#[derive(Clone)]
pub struct CdlValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub offset: Vec<f64>,
    pub power: Vec<f64>,
    pub slope: Vec<f64>,
}
impl PartialEq for CdlValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
