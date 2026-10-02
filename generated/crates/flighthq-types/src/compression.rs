// @generated from upstream/packages/types/src/Compression.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub decompress: Decompressor,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub compress: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<u8>, CompressionFraming) -> Vec<u8> + Send + 'static>>,
    >,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

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
pub type CompressionFraming = String;

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
pub type Compression = String;

// Source: upstream/packages/types/src/Compression.ts:45 (sha256:d37b9f48212117c0f89fc1a641ddbc565c6cbbb70eeafb0f2048ac7ed647a47b)
#[derive(Clone)]
pub struct HostDecompressBrotliCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decompress: Decompressor,
}
impl PartialEq for HostDecompressBrotliCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Compression.ts:49 (sha256:a67e5a6c07cb9305a98e838926e1a394346a82acf22bd1973a2544fb4ddba21a)
#[derive(Clone)]
pub struct HostDecompressDeflateCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decompress: Decompressor,
}
impl PartialEq for HostDecompressDeflateCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Compression.ts:53 (sha256:37b4667a132b66ec3adc262e7bc67e725a19d8ae508f0156890e03528e43faf5)
#[derive(Clone)]
pub struct HostDecompressLzmaCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decompress: Decompressor,
}
impl PartialEq for HostDecompressLzmaCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Compression.ts:60 (sha256:33e9a97ef98dc847d5be24961c5fee454e3254b3de53da53f6076cb8790f87cb)
#[derive(Clone)]
pub struct HostCompressDeflateCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub compress: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<u8>, CompressionFraming) -> Vec<u8> + Send + 'static>>,
    >,
}
impl PartialEq for HostCompressDeflateCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Compression.ts:64 (sha256:71f5e783a3105b6787c50a50e7462f8d9b352044e6fef17366bf7e9f35a43de7)
#[derive(Clone)]
pub struct HostCompressLzmaCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub compress: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<u8>, CompressionFraming) -> Vec<u8> + Send + 'static>>,
    >,
}
impl PartialEq for HostCompressLzmaCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
