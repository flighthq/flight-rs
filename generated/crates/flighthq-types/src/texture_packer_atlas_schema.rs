// @generated from upstream/packages/types/src/TexturePackerAtlasSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:5 (sha256:8c641baac48b698f3500e8c79b7f86fa76255b774e34f5d393f75ad813fd078e)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TexturePackerAtlasRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:12 (sha256:f1a2d7689a6c47dcb79962d0871e1da12e7587eae663b15ba31936d34a94c6e5)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasSize {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
}
impl PartialEq for TexturePackerAtlasSize {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:17 (sha256:598c7a4c75a87d846ea1b661dfcc8e9f415d319296f23596a8bfb3db6d70cbc9)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasPivot {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TexturePackerAtlasPivot {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:22 (sha256:ffddae777bdb6aa6c7cffe3d2badf5936985afc1bb8eee19089a1e02f64033c3)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasFrameTag {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: Option<String>,
    pub from: f64,
    pub name: String,
    pub to: f64,
}
impl PartialEq for TexturePackerAtlasFrameTag {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:29 (sha256:e9bf7ebf7b42cfed919188e74f18297ac79702ed9c32ac72232b5b1df10b29ef)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasHashFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: TexturePackerAtlasRect,
    pub pivot: Option<TexturePackerAtlasPivot>,
    pub rotated: bool,
    pub source_size: TexturePackerAtlasSize,
    pub sprite_source_size: TexturePackerAtlasRect,
    pub trimmed: bool,
}
impl PartialEq for TexturePackerAtlasHashFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:38 (sha256:10029516be37570432f246b5a8852b3da13e1cd192d78f899c7b38f362c2c3f1)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasArrayFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: TexturePackerAtlasRect,
    pub pivot: Option<TexturePackerAtlasPivot>,
    pub rotated: bool,
    pub source_size: TexturePackerAtlasSize,
    pub sprite_source_size: TexturePackerAtlasRect,
    pub trimmed: bool,
    pub filename: String,
}
impl PartialEq for TexturePackerAtlasArrayFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:42 (sha256:2ffa010ec6860f029207a48caf916f96a79f26fc4d2c4ef49d59d70913e3cafc)
#[derive(Clone)]
pub struct TexturePackerAtlasMeta {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: String,
    pub format: String,
    pub frame_tags: Option<Vec<TexturePackerAtlasFrameTag>>,
    pub image: String,
    pub scale: crate::FlightUnion2<f64, String>,
    pub size: TexturePackerAtlasSize,
    pub version: String,
}
impl PartialEq for TexturePackerAtlasMeta {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:53 (sha256:4db335ed549fe2697134624e01a46872a33b17515122bb2786ef0578e4371f53)
#[derive(Clone)]
pub struct TexturePackerAtlasHashDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<(String, TexturePackerAtlasHashFrame)>,
    pub meta: TexturePackerAtlasMeta,
}
impl PartialEq for TexturePackerAtlasHashDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:59 (sha256:6e461bfdfe8e5a80ad8d6ed4466f979f67dc17fc0651011ffc88a71452dcd92d)
#[derive(Clone)]
pub struct TexturePackerAtlasArrayDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<TexturePackerAtlasArrayFrame>,
    pub meta: TexturePackerAtlasMeta,
}
impl PartialEq for TexturePackerAtlasArrayDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerAtlasSchema.ts:64 (sha256:d3b731ab17b8bfc00ae55e1775abf154e2b5c02dc7d8933367993094e721ab1d)
pub type TexturePackerAtlasDocument =
    crate::FlightUnion2<TexturePackerAtlasArrayDocument, TexturePackerAtlasHashDocument>;
