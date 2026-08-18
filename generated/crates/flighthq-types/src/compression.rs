// @generated from upstream/packages/types/src/Compression.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Compression.ts:8 (sha256:5999f775f3faed0b2b55af9401bdafbaf0f1c4e4c5e2eaab345a38fb601156f8)
pub type Decompressor = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Vec<u8>, f64) -> Option<Vec<u8>> + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Compression.ts:14 (sha256:f9f22217d497d13486a40ffe418b2045c1637a3971062176826e062dd9a2eca5)
#[derive(Clone, Default)]
pub struct CompressionValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deflate: String,
    pub lzma: String,
}
impl PartialEq for CompressionValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static COMPRESSION: std::sync::LazyLock<CompressionValues> =
    std::sync::LazyLock::new(|| CompressionValues {
        __flight_identity: std::sync::Arc::new(()),
        deflate: "deflate".to_owned(),
        lzma: "lzma".to_owned(),
    });

// Source: upstream/packages/types/src/Compression.ts:19 (sha256:4a4cbb08689ef32ef9ed902e3c65e57fa3f9653d018d17766641b43294c759e3)
pub type Compression = crate::OpaqueHostValue;
