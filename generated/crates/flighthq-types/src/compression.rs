// @generated from upstream/packages/types/src/Compression.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Compression.ts:8 (sha256:f3a41eb4c968c9a7bb29d8e08d4d1e5cf84a6de9989f1b2d781103aa310b88ab)
pub type Decompressor = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Vec<u8>, f64, CompressionFraming) -> Option<Vec<u8>> + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/Compression.ts:17 (sha256:902f32775fe6731d72d5e3ad1cd9445e7cd49b86501ed09599908fc02b595f5e)
#[derive(Clone, Default)]
pub struct CompressionFramingValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub raw: String,
    pub rfc1950: String,
}
impl PartialEq for CompressionFramingValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static COMPRESSION_FRAMING: std::sync::LazyLock<CompressionFramingValues> =
    std::sync::LazyLock::new(|| CompressionFramingValues {
        __flight_identity: std::sync::Arc::new(()),
        raw: "Raw".to_owned(),
        rfc1950: "Rfc1950".to_owned(),
    });

// Source: upstream/packages/types/src/Compression.ts:22 (sha256:0663a732b389fdc601327a8fa1cd571b28fb2f1d9f830a4e5c70d8c27236d3c2)
pub type CompressionFraming = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Compression.ts:28 (sha256:b302f6bcd038f52a990cbbaf9111552c232fb22e4e6a3d8f88eb03888ab5551e)
#[derive(Clone, Default)]
pub struct CompressionValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brotli: String,
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
        brotli: "brotli".to_owned(),
        deflate: "deflate".to_owned(),
        lzma: "lzma".to_owned(),
    });

// Source: upstream/packages/types/src/Compression.ts:38 (sha256:4a4cbb08689ef32ef9ed902e3c65e57fa3f9653d018d17766641b43294c759e3)
pub type Compression = crate::OpaqueHostValue;
