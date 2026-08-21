// @generated from upstream/packages/types/src/WoffChecksumMismatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WoffChecksumMismatch.ts:8 (sha256:d681917d746d41849f9057ce0c8324c04c0925575a5f03dbdff912f8bca92b25)
#[derive(Clone, Default)]
pub struct WoffChecksumMismatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub stored: f64,
    pub computed: f64,
    pub tag: String,
}
impl PartialEq for WoffChecksumMismatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
