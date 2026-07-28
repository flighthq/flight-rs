// @generated from upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:5 (sha256:b90622ac69b3fedb4b2dec510b2e99c623651e4bf98a07613a26b0273d90fede)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TextureAtlasPackerRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:12 (sha256:3d25d4fb63609ac30574f0b5b8f7722faa89ee63a70a1e54c14e56d02136c66b)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerSize {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
}
impl PartialEq for TextureAtlasPackerSize {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:17 (sha256:fe1acd4ffb3357c26c2a2bfd2ed94086829b6a903da894ee5be31e06d73bbaad)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerPivot {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TextureAtlasPackerPivot {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:22 (sha256:1f1fde2202c9eabd9d7cabb7884fcf1f567961fc723415e1bac27fab0ae69807)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerFrameTag {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: Option<String>,
    pub from: f64,
    pub name: String,
    pub to: f64,
}
impl PartialEq for TextureAtlasPackerFrameTag {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:29 (sha256:864b10bf3703b4f5b773b0425e7d33bc137a004fa2099697b03d411981808987)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerHashFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: TextureAtlasPackerRect,
    pub pivot: Option<TextureAtlasPackerPivot>,
    pub rotated: bool,
    pub source_size: TextureAtlasPackerSize,
    pub sprite_source_size: TextureAtlasPackerRect,
    pub trimmed: bool,
}
impl PartialEq for TextureAtlasPackerHashFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:38 (sha256:9ab26f23dd1cb31acedabd8644d504e3df1efb4f0d6f58e8ab3b1116478e0c38)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerArrayFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: TextureAtlasPackerRect,
    pub pivot: Option<TextureAtlasPackerPivot>,
    pub rotated: bool,
    pub source_size: TextureAtlasPackerSize,
    pub sprite_source_size: TextureAtlasPackerRect,
    pub trimmed: bool,
    pub filename: String,
}
impl PartialEq for TextureAtlasPackerArrayFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:42 (sha256:82009b0dfa1b4e7f2c8549da8ed731b0b3a541de46397a4dc5404747efb285d9)
#[derive(Clone)]
pub struct TextureAtlasPackerMeta {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: String,
    pub format: String,
    pub frame_tags: Option<Vec<TextureAtlasPackerFrameTag>>,
    pub image: String,
    pub scale: crate::FlightUnion2<f64, String>,
    pub size: TextureAtlasPackerSize,
    pub version: String,
}
impl PartialEq for TextureAtlasPackerMeta {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:53 (sha256:0abdb3a30701c687c457bdec896f232331316f5eb470594e7493ffb66a74c1db)
#[derive(Clone)]
pub struct TextureAtlasPackerHashDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<(String, TextureAtlasPackerHashFrame)>,
    pub meta: TextureAtlasPackerMeta,
}
impl PartialEq for TextureAtlasPackerHashDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:59 (sha256:363c0b8ea1a4a2897da5288846c6fa9dd48f5510a2a9e3011b8012484dba174d)
#[derive(Clone)]
pub struct TextureAtlasPackerArrayDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<TextureAtlasPackerArrayFrame>,
    pub meta: TextureAtlasPackerMeta,
}
impl PartialEq for TextureAtlasPackerArrayDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerSchema.ts:64 (sha256:b6692c2ae05861bdefe6e9637d85ef8969344a8c13e1349f11ce2a8827c78d8d)
pub type TextureAtlasPackerDocument =
    crate::FlightUnion2<TextureAtlasPackerArrayDocument, TextureAtlasPackerHashDocument>;
