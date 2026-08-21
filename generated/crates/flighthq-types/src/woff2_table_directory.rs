// @generated from upstream/packages/types/src/Woff2TableDirectory.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Woff2TableDirectory.ts:7 (sha256:4bf3f9cb08a7f89dc757f5a18a91f191089e75bf6a44e09fd0a3bb870c597177)
#[derive(Clone, Default)]
pub struct Woff2TableDirectory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entries: Vec<Woff2TableEntry>,
    pub stream_start: f64,
    pub total_uncompressed_length: f64,
}
impl PartialEq for Woff2TableDirectory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Woff2TableDirectory.ts:18 (sha256:24ae797b8608d8871a3ebd378c3101ddb9f4b4bc1859b2b282e50ae9f4b8c1f8)
#[derive(Clone, Default)]
pub struct Woff2TableEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub original_length: f64,
    pub tag: String,
    pub transform_length: f64,
    pub transform_version: f64,
    pub transformed: bool,
}
impl PartialEq for Woff2TableEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
