// @generated from upstream/packages/types/src/AsepriteSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetData;

// Source: upstream/packages/types/src/AsepriteSchema.ts:8 (sha256:680c5ff941b1d1ce07dc7beaef691ce9cc8dde960c10b6fb453e81b7f5de6098)
#[derive(Clone, Default)]
pub struct AsepriteRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for AsepriteRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:15 (sha256:6e9c8f873cb201229b6ac5d3f229ef70abf511e0fc44050ef0c4818bb94e4fb0)
#[derive(Clone, Default)]
pub struct AsepriteSize {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: f64,
    pub w: f64,
}
impl PartialEq for AsepriteSize {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:20 (sha256:30f97ec17c8bda55ce1e5e018cdca0b46d443dacc22b826c96eead1589d2ee15)
#[derive(Clone, Default)]
pub struct AsepriteFrameTag {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: String,
    pub from: f64,
    pub name: String,
    pub to: f64,
    pub color: Option<String>,
}
impl PartialEq for AsepriteFrameTag {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:32 (sha256:69cd2b4643bbc135d89d6ddfb37d61d10981152f0a43199b2c10da5839c57cbb)
#[derive(Clone, Default)]
pub struct AsepriteLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: String,
    pub name: String,
    pub opacity: f64,
}
impl PartialEq for AsepriteLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:38 (sha256:09512deac32c5ca8b242f904699164bf979a3f162eef5289b2073c22ab440755)
#[derive(Clone, Default)]
pub struct AsepriteBaseFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub frame: AsepriteRect,
    pub rotated: bool,
    pub source_size: AsepriteSize,
    pub sprite_source_size: AsepriteRect,
    pub trimmed: bool,
}
impl PartialEq for AsepriteBaseFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:49 (sha256:7c494978a5fe13f833f20498cbfb7f9d360a059009b833ec38d7d3b337275138)
pub type AsepriteHashFrame = AsepriteBaseFrame;

// Source: upstream/packages/types/src/AsepriteSchema.ts:52 (sha256:068b4c436e296042b4e4d7892f9980cc324c32c1b017e5b0c25663be93427bc5)
#[derive(Clone, Default)]
pub struct AsepriteArrayFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub frame: AsepriteRect,
    pub rotated: bool,
    pub source_size: AsepriteSize,
    pub sprite_source_size: AsepriteRect,
    pub trimmed: bool,
    pub filename: String,
}
impl PartialEq for AsepriteArrayFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:56 (sha256:1232b636807cb66efe51b250cfb3befa06982f773de79c6d9c60c028f3cea551)
#[derive(Clone)]
pub struct AsepriteMeta {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: String,
    pub format: String,
    pub frame_tags: Vec<AsepriteFrameTag>,
    pub image: String,
    pub layers: Option<Vec<AsepriteLayer>>,
    pub scale: crate::FlightUnion2<f64, String>,
    pub size: AsepriteSize,
    pub slices: Option<Vec<crate::OpaqueHostValue>>,
    pub version: String,
}
impl PartialEq for AsepriteMeta {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:69 (sha256:0aea13a6311a93d1e546664307ad7789961f6e8f3c22f668f6af5389e41fe630)
#[derive(Clone)]
pub struct AsepriteHashDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<(String, AsepriteHashFrame)>,
    pub meta: AsepriteMeta,
}
impl PartialEq for AsepriteHashDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:75 (sha256:94db88bc3dd2655300b33e4777ad6c5c90e96eb9118e21720f470caf8b4306b6)
#[derive(Clone)]
pub struct AsepriteArrayDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Vec<AsepriteArrayFrame>,
    pub meta: AsepriteMeta,
}
impl PartialEq for AsepriteArrayDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:80 (sha256:18e0404f42de5e29d18768b5b0cb4cd4ecf60ab365ee1decb2ac130c8fc6de6d)
pub type AsepriteDocument = crate::FlightUnion2<AsepriteArrayDocument, AsepriteHashDocument>;

// Source: upstream/packages/types/src/AsepriteSchema.ts:83 (sha256:b875098780abb47b362c3a941efda0931e73544b21fc156364a67205a617b97e)
#[derive(Clone)]
pub struct AsepriteParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: SpritesheetData,
    pub document: AsepriteDocument,
}
impl PartialEq for AsepriteParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AsepriteSchema.ts:88 (sha256:ad77ec90545b8c96f7b29e5016d7fa8367e731960f142d0adda8e1489aa4b4af)
#[derive(Clone, Default)]
pub struct AsepriteSerializeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub variant: Option<String>,
}
impl PartialEq for AsepriteSerializeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
