// @generated from upstream/packages/types/src/SwfParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{HostDecompressDeflateCapability, HostDecompressLzmaCapability, SwfTagHandler};

// Source: upstream/packages/types/src/SwfParseOptions.ts:21 (sha256:20ac688152a26ebb29eefab76c99c8bf6ffab814fabdae8fdb42590ec9b3e3a7)
#[derive(Clone, Default)]
pub struct SwfParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deflate: Option<HostDecompressDeflateCapability>,
    pub lzma: Option<HostDecompressLzmaCapability>,
    pub tags: Vec<SwfTagHandler>,
}
impl PartialEq for SwfParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
