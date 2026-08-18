// @generated from upstream/packages/types/src/CompressedImageData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureContainer;

// Source: upstream/packages/types/src/CompressedImageData.ts:6 (sha256:d7bacc0c0714c22beb89d0d503f5a25e0a296445d56e813b8d029df6b5b25166)
#[derive(Clone, Default)]
pub struct CompressedImageData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub container: TextureContainer,
    pub payload: Vec<u8>,
}
impl PartialEq for CompressedImageData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
