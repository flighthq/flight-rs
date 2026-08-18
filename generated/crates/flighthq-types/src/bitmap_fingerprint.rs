// @generated from upstream/packages/types/src/BitmapFingerprint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapFingerprint.ts:8 (sha256:50b5f1e7cf212f956951395d3caf98ac9fefcaaf982b3bc77b182c1b6ae2ecde)
#[derive(Clone, Default)]
pub struct BitmapFingerprint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub grid_size: f64,
    pub cells: Vec<u8>,
}
impl PartialEq for BitmapFingerprint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
