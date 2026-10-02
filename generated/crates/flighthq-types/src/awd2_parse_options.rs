// @generated from upstream/packages/types/src/Awd2ParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Awd2BlockHandler, HostDecompressDeflateCapability, HostDecompressLzmaCapability};

// Source: upstream/packages/types/src/Awd2ParseOptions.ts:21 (sha256:1b820f2f56722ec36f89363ad4cb48cf064d2ea985e8e9b99e48e2543d7a3c69)
#[derive(Clone, Default)]
pub struct Awd2ParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blocks: Vec<Awd2BlockHandler>,
    pub deflate: Option<HostDecompressDeflateCapability>,
    pub lzma: Option<HostDecompressLzmaCapability>,
}
impl PartialEq for Awd2ParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
