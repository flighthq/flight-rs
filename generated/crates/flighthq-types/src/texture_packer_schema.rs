// @generated from upstream/packages/types/src/TexturePackerSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetData;

// Source: upstream/packages/types/src/TexturePackerSchema.ts:7 (sha256:b8822b9f6992aa201e245a3219573ad6696dfc3fc299aaff6a8964c681cd5de7)
#[derive(Clone, Default)]
pub struct TexturePackerRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TexturePackerRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:14 (sha256:3bb2bc436f9bd434075cd40ac0d6c1c43a3deef3756fa1b400a9e85583e7152b)
#[derive(Clone, Default)]
pub struct TexturePackerSize {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
}
impl PartialEq for TexturePackerSize {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:19 (sha256:a20bc06d5d27b1c3d90c8cbe60aee3d3b4a7b9f18262ddcc212c19bcebe876d6)
#[derive(Clone, Default)]
pub struct TexturePackerPivot {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TexturePackerPivot {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:24 (sha256:c3f8b0dc11ddbd63e6b39a8ee3a1469542659da86c1e6037e8470e67e467e24b)
#[derive(Clone, Default)]
pub struct TexturePackerFrameTag {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: String,
    pub from: f64,
    pub name: String,
    pub to: f64,
}
impl PartialEq for TexturePackerFrameTag {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:31 (sha256:f932cdca994ce7503b137855535fbb122df16ce492767fee5b52854f7b8b8ff6)
#[derive(Clone, Default)]
pub struct TexturePackerHashFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: TexturePackerRect,
    pub pivot: Option<TexturePackerPivot>,
    pub rotated: bool,
    pub source_size: TexturePackerSize,
    pub sprite_source_size: TexturePackerRect,
    pub trimmed: bool,
}
impl PartialEq for TexturePackerHashFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:40 (sha256:a39c6ecab5cc3e600facdec3483e48a4680f2ae830a200bd49cf37a36db216e6)
#[derive(Clone, Default)]
pub struct TexturePackerArrayFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: TexturePackerRect,
    pub pivot: Option<TexturePackerPivot>,
    pub rotated: bool,
    pub source_size: TexturePackerSize,
    pub sprite_source_size: TexturePackerRect,
    pub trimmed: bool,
    pub filename: String,
}
impl PartialEq for TexturePackerArrayFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:44 (sha256:b8f6a8e2db9bde2acbc15e9b90a883a844ff768ffb21d9c47732d2198c3438e9)
#[derive(Clone)]
pub struct TexturePackerMeta {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: String,
    pub format: String,
    pub frame_tags: Option<Vec<TexturePackerFrameTag>>,
    pub image: String,
    pub scale: crate::FlightUnion2<f64, String>,
    pub size: TexturePackerSize,
    pub version: String,
}
impl PartialEq for TexturePackerMeta {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:55 (sha256:69b2dbdeb7d460ef273bc23c86030e4ec52c529eaeaf69ab1ceb584055febba4)
#[derive(Clone)]
pub struct TexturePackerHashDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<(String, TexturePackerHashFrame)>,
    pub meta: TexturePackerMeta,
}
impl PartialEq for TexturePackerHashDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:61 (sha256:6ce9edc98dcd73b16c10b879b3b9ab3b9e87ec40058afca2bb83125f0c0275f1)
#[derive(Clone)]
pub struct TexturePackerArrayDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<TexturePackerArrayFrame>,
    pub meta: TexturePackerMeta,
}
impl PartialEq for TexturePackerArrayDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:66 (sha256:a7a4876c4423bcc41baa91fd4a3572aecdb6284014ff371a43d0b6382a872ff5)
pub type TexturePackerDocument =
    crate::FlightUnion2<TexturePackerArrayDocument, TexturePackerHashDocument>;

// Source: upstream/packages/types/src/TexturePackerSchema.ts:69 (sha256:dbb90dcfb2053970761b37db4ab2e47c5c179dac0bcf2d2820421fdf8a0012c2)
#[derive(Clone)]
pub struct TexturePackerParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: SpritesheetData,
    pub document: TexturePackerDocument,
}
impl PartialEq for TexturePackerParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TexturePackerSchema.ts:74 (sha256:a919495917d81cf989947bfd49badab814eb27a004c127c97c687af276aab325)
#[derive(Clone, Default)]
pub struct TexturePackerSerializeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub variant: Option<String>,
}
impl PartialEq for TexturePackerSerializeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
