// @generated from upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:6 (sha256:2beead3ae5eac032bc7a50b746eb0b1898365ec252e52473c4f9c0ddebd635f8)
#[derive(Clone, Default)]
pub struct TextureAtlasAsepriteRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TextureAtlasAsepriteRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:13 (sha256:0c0a517b77629b15a5c7f0996b5d745135521a390b8e344bfbac77dffce09ca1)
#[derive(Clone, Default)]
pub struct TextureAtlasAsepriteSize {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
}
impl PartialEq for TextureAtlasAsepriteSize {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:18 (sha256:d4896ebca38c25e376543c3a1cfb238c813dc355bc0f6d9dd1bd24f9400a4b26)
#[derive(Clone, Default)]
pub struct TextureAtlasAsepriteFrameTag {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: String,
    pub from: f64,
    pub name: String,
    pub to: f64,
    pub color: Option<String>,
}
impl PartialEq for TextureAtlasAsepriteFrameTag {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:30 (sha256:c398159e8f726192008c83badf867731d8c23a7bf213a7a2774073f3e75c8877)
#[derive(Clone, Default)]
pub struct TextureAtlasAsepriteBaseFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub frame: TextureAtlasAsepriteRect,
    pub rotated: bool,
    pub source_size: TextureAtlasAsepriteSize,
    pub sprite_source_size: TextureAtlasAsepriteRect,
    pub trimmed: bool,
}
impl PartialEq for TextureAtlasAsepriteBaseFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:40 (sha256:a7a3d04a04d919c0d43aaeed6ec020e4ffdff53d0ab869de8f530f758c53e0a9)
pub type TextureAtlasAsepriteHashFrame = TextureAtlasAsepriteBaseFrame;

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:42 (sha256:0ff3adfc0f9e14a55e2c6c04be79cb8527c30b213c5e673c9b2af54bba2aa90a)
#[derive(Clone, Default)]
pub struct TextureAtlasAsepriteArrayFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub frame: TextureAtlasAsepriteRect,
    pub rotated: bool,
    pub source_size: TextureAtlasAsepriteSize,
    pub sprite_source_size: TextureAtlasAsepriteRect,
    pub trimmed: bool,
    pub filename: String,
}
impl PartialEq for TextureAtlasAsepriteArrayFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:46 (sha256:8f2a921232579e2a47c42ee169d01bb56842626bbc59540c392b7e9c4d2e0886)
#[derive(Clone)]
pub struct TextureAtlasAsepriteMeta {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: String,
    pub format: String,
    pub frame_tags: Option<Vec<TextureAtlasAsepriteFrameTag>>,
    pub image: String,
    pub scale: crate::FlightUnion2<f64, String>,
    pub size: TextureAtlasAsepriteSize,
    pub version: String,
}
impl PartialEq for TextureAtlasAsepriteMeta {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:56 (sha256:064075433ef596eced8ea3b8b3fb3f1776b25cb9ff01baead966a5bd6bfb264c)
#[derive(Clone)]
pub struct TextureAtlasAsepriteHashDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<(String, TextureAtlasAsepriteHashFrame)>,
    pub meta: TextureAtlasAsepriteMeta,
}
impl PartialEq for TextureAtlasAsepriteHashDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:61 (sha256:1dd9569641a41fb68ffe9d131071a3594db55b48606625b1394088ec6dcf83b1)
#[derive(Clone)]
pub struct TextureAtlasAsepriteArrayDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<TextureAtlasAsepriteArrayFrame>,
    pub meta: TextureAtlasAsepriteMeta,
}
impl PartialEq for TextureAtlasAsepriteArrayDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteSchema.ts:66 (sha256:d8ce364c99c3a57b1e1e41a2e376d5be570db1bae573a89f886a0b475bfca6af)
pub type TextureAtlasAsepriteDocument =
    crate::FlightUnion2<TextureAtlasAsepriteArrayDocument, TextureAtlasAsepriteHashDocument>;
