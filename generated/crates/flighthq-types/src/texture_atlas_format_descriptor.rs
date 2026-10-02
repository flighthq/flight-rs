// @generated from upstream/packages/types/src/TextureAtlasFormatDescriptor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TextureAtlas, TextureAtlasFormatKind, TextureAtlasParseOptions};

// Source: upstream/packages/types/src/TextureAtlasFormatDescriptor.ts:11 (sha256:508384f98ab6433f93a1e3b420d77ec2a8aaba05269bb82fd3d9621cdfe10fac)
#[derive(Clone)]
pub struct TextureAtlasFormatEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, TextureAtlas, TextureAtlasParseOptions) -> TextureAtlas
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TextureAtlasFormatEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextureAtlasFormatDescriptor.ts:24 (sha256:28aa194cad98608b1da8e42e575a8fa57720042514ea5f4473586a1bc75c9ca3)
#[derive(Clone)]
pub struct TextureAtlasFormatDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry: TextureAtlasFormatEntry,
    pub kind: TextureAtlasFormatKind,
}
impl PartialEq for TextureAtlasFormatDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextureAtlasFormatDescriptor.ts:49 (sha256:7c3da4e755aebeadf708f47715da22c7df3f5f0e75e1b4f1840a62dc3fc8f8e0)
#[derive(Clone, Default)]
pub struct TextureAtlasImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub formats: Option<Vec<TextureAtlasFormatDescriptor>>,
}
impl PartialEq for TextureAtlasImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
