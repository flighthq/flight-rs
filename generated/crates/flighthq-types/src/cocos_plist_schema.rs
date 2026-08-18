// @generated from upstream/packages/types/src/CocosPlistSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetData;

// Source: upstream/packages/types/src/CocosPlistSchema.ts:8 (sha256:250923d38aeeb1412222a44d88d5e5f42d4055b68c7202c895de4d3354616d8e)
#[derive(Clone, Default)]
pub struct CocosPlistFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: String,
    pub sprite_offset: String,
    pub sprite_size: String,
    pub sprite_source_size: String,
    pub sprite_trimmed: bool,
    pub texture_rotated: bool,
    pub aliases: Option<Vec<String>>,
}
impl PartialEq for CocosPlistFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CocosPlistSchema.ts:25 (sha256:cb37a92b2fbeba643de2f51bed8ae4ebe3743d8b33caa20c25a1ee08c9ef7a85)
#[derive(Clone, Default)]
pub struct CocosPlistMetadata {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: f64,
    pub size: String,
    pub texture_file_name: String,
}
impl PartialEq for CocosPlistMetadata {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CocosPlistSchema.ts:34 (sha256:8ea7fe5e03bc3f451444521626f57fa3533ced7a79d99b097b7df9837bef6d1d)
#[derive(Clone, Default)]
pub struct CocosPlistDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<(String, CocosPlistFrame)>,
    pub metadata: CocosPlistMetadata,
}
impl PartialEq for CocosPlistDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CocosPlistSchema.ts:42 (sha256:f45dded508b249dec4372cff3a4ca36455da91faf68cefac430d89a3c5d4e492)
#[derive(Clone, Default)]
pub struct CocosPlistParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: SpritesheetData,
    pub document: CocosPlistDocument,
}
impl PartialEq for CocosPlistParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
