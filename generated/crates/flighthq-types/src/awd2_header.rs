// @generated from upstream/packages/types/src/Awd2Header.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Awd2Header.ts:9 (sha256:07afd4ec6d7e20ea32179423b6dcd68cf83b3f18050c4fb191405469414124ff)
#[derive(Clone, Default)]
pub struct Awd2Header {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_length: f64,
    pub compression: f64,
    pub flags: f64,
    pub version_major: f64,
    pub version_minor: f64,
}
impl PartialEq for Awd2Header {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
