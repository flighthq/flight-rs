// @generated from upstream/packages/types/src/DecodedImage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DecodedImage.ts:5 (sha256:b15a526392f9278b8177a746db9d128ae267248a7f2fc9c968da69246f3df017)
#[derive(Clone, Default)]
pub struct DecodedImage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Vec<u8>,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for DecodedImage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
