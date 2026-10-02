// @generated from upstream/packages/types/src/BitmapFont.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GlyphEntry, GlyphMetrics, TextureAtlas};

// Source: upstream/packages/types/src/BitmapFont.ts:9 (sha256:0dbb28c501f1a07781092c05e4991200e0bb5aed88d8b1d6511b3cc484ce22fb)
pub type BitmapFontEncoding = String;

// Source: upstream/packages/types/src/BitmapFont.ts:21 (sha256:8ed3e84c04f4136e3374559978552f1c764a4490c205aa32f78e0e43435ae928)
#[derive(Clone, Default)]
pub struct BitmapFont {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub encoding: BitmapFontEncoding,
    pub glyphs: Vec<(f64, GlyphEntry)>,
    pub kerning: Vec<(f64, f64)>,
    pub metrics: GlyphMetrics,
    pub pages: Vec<TextureAtlas>,
}
impl PartialEq for BitmapFont {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for BitmapFont {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/BitmapFont.ts:34 (sha256:6e1a9d1837880bbfd8ef9f8b632301b8a37c4c91fc5990a7106573bb5c27f160)
#[derive(Clone, Default)]
pub struct BitmapFontData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub encoding: Option<BitmapFontEncoding>,
    pub glyphs: Vec<BitmapFontGlyphData>,
    pub kerning: Option<Vec<BitmapFontKerningData>>,
    pub metrics: GlyphMetrics,
    pub pages: Vec<TextureAtlas>,
}
impl PartialEq for BitmapFontData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFont.ts:46 (sha256:659c79db7d449865e6cd120c135862dc456243a84ce5adaf6a18eb34a75b3304)
#[derive(Clone, Default)]
pub struct BitmapFontGlyphData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advance: f64,
    pub bearing_x: f64,
    pub bearing_y: f64,
    pub codepoint: f64,
    pub height: f64,
    pub page: Option<f64>,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for BitmapFontGlyphData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFont.ts:61 (sha256:9a5991038acb1fc418e69418dd008620eeba8516a1f95edae5ac99aab30aa9b4)
#[derive(Clone, Default)]
pub struct BitmapFontKerningData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub amount: f64,
    pub left: f64,
    pub right: f64,
}
impl PartialEq for BitmapFontKerningData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFont.ts:70 (sha256:ec632b5827729a9f465c3d2233e792e93719216785472ad6454984b557e22ad2)
#[derive(Clone, Default)]
pub struct BitmapFontKerningPair {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub left: f64,
    pub right: f64,
}
impl PartialEq for BitmapFontKerningPair {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFont.ts:84 (sha256:8a834ba012746d7f5fe7cfedc32bfe7fa3b30d2d87036511f915e71222c83f4e)
#[derive(Clone, Default)]
pub struct BitmapFontParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolve_page: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(f64, String) -> Option<TextureAtlas> + Send + 'static>>,
        >,
    >,
}
impl PartialEq for BitmapFontParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
