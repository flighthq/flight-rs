// @generated from upstream/packages/types/src/BitmapFont.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlyphMetrics, TextureAtlas};

// Source: upstream/packages/types/src/BitmapFont.ts:8 (sha256:0dbb28c501f1a07781092c05e4991200e0bb5aed88d8b1d6511b3cc484ce22fb)
pub type BitmapFontEncoding = String;

// Source: upstream/packages/types/src/BitmapFont.ts:19 (sha256:7623956115ce39a98a8e3b94251f15ed941de49c47aa9cac9d649a4f809f647d)
#[derive(Clone)]
pub struct BitmapFont {
    pub encoding: BitmapFontEncoding,
    pub glyphs: crate::OpaqueHostValue,
    pub kerning: crate::OpaqueHostValue,
    pub metrics: GlyphMetrics,
    pub pages: Vec<TextureAtlas>,
}

// Source: upstream/packages/types/src/BitmapFont.ts:32 (sha256:6e1a9d1837880bbfd8ef9f8b632301b8a37c4c91fc5990a7106573bb5c27f160)
#[derive(Clone)]
pub struct BitmapFontData {
    pub encoding: Option<BitmapFontEncoding>,
    pub glyphs: Vec<BitmapFontGlyphData>,
    pub kerning: Option<Vec<BitmapFontKerningData>>,
    pub metrics: GlyphMetrics,
    pub pages: Vec<TextureAtlas>,
}

// Source: upstream/packages/types/src/BitmapFont.ts:44 (sha256:659c79db7d449865e6cd120c135862dc456243a84ce5adaf6a18eb34a75b3304)
#[derive(Clone)]
pub struct BitmapFontGlyphData {
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

// Source: upstream/packages/types/src/BitmapFont.ts:59 (sha256:9a5991038acb1fc418e69418dd008620eeba8516a1f95edae5ac99aab30aa9b4)
#[derive(Clone)]
pub struct BitmapFontKerningData {
    pub amount: f64,
    pub left: f64,
    pub right: f64,
}

// Source: upstream/packages/types/src/BitmapFont.ts:74 (sha256:8a834ba012746d7f5fe7cfedc32bfe7fa3b30d2d87036511f915e71222c83f4e)
#[derive(Clone)]
pub struct BitmapFontParseOptions {
    pub resolve_page:
        Option<std::sync::Arc<dyn Fn(f64, String) -> Option<TextureAtlas> + Send + Sync + 'static>>,
}
