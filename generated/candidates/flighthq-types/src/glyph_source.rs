// @generated from upstream/packages/types/src/GlyphSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Surface;

// Source: upstream/packages/types/src/GlyphSource.ts:15 (sha256:620fb139146de60e01d255de5d3f0c7beff4e8627770f3bcc76d5eec3d1aa3a2)
#[derive(Clone)]
pub struct GlyphSource {
    pub get_glyph_atlas_image: crate::OpaqueHostValue,
    pub get_glyph_entry: crate::OpaqueHostValue,
    pub get_glyph_kerning: crate::OpaqueHostValue,
    pub get_glyph_metrics: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlyphSource.ts:34 (sha256:35b568c220bc7e447eb59d00ddffd58a2c092b969eb3e20a71199392e2edb9a4)
#[derive(Clone)]
pub struct GlyphEntry {
    pub advance: f64,
    pub bearing_x: f64,
    pub bearing_y: f64,
    pub height: f64,
    pub page: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/GlyphSource.ts:50 (sha256:0eb6596ab428f58bb83ef3c812867a6539e640d5c18b243b4e76a11c5b205617)
#[derive(Clone)]
pub struct GlyphMetrics {
    pub ascent: f64,
    pub descent: f64,
    pub line_gap: f64,
}

// Source: upstream/packages/types/src/GlyphSource.ts:60 (sha256:f7176fd3653d1eed66fa8e264d284cacdba9c577464213d51d710bf2f18bea8b)
#[derive(Clone)]
pub struct GlyphRasterizedBitmap {
    pub advance: f64,
    pub bearing_x: f64,
    pub bearing_y: f64,
    pub height: f64,
    pub pixels: Vec<u8>,
    pub width: f64,
}

// Source: upstream/packages/types/src/GlyphSource.ts:71 (sha256:41c7407d7cd069a73c5d57fee671d76247ab5f92dfb2173c052ae8224759b857)
#[derive(Clone)]
pub struct GlyphRasterizeOptions {
    pub font_family: String,
    pub font_size: f64,
    pub font_style: Option<String>,
    pub font_weight: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/GlyphSource.ts:81 (sha256:48735754c6f79d36f0da82d8f760d14bde9e75339b7c5fecf8e2a486129c5b5a)
#[derive(Clone)]
pub struct GlyphRasterizerBackend {
    pub rasterize: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlyphSource.ts:88 (sha256:0e4e882c018edcf42950aa1ffa8e7c33e117c5021bf9aaf9df71ea1547866b3c)
#[derive(Clone)]
pub struct GlyphAtlasOptions {
    pub font_family: String,
    pub font_size: f64,
    pub height: f64,
    pub max_glyphs: Option<f64>,
    pub padding: Option<f64>,
    pub width: f64,
}

// Source: upstream/packages/types/src/GlyphSource.ts:99 (sha256:dc15e695a4ce542d6a7bf3906d5ca2b88fa4114d6404823a05cb8eb49a786bfc)
#[derive(Clone)]
pub struct GlyphAtlasShelf {
    pub cursor_x: f64,
    pub height: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/GlyphSource.ts:109 (sha256:93ed7ab4507aab5b5f68c6ccd4bc3dadbdcc4563a8fb382623d12c8cc3195291)
#[derive(Clone)]
pub struct GlyphAtlasRuntime {
    pub bitmaps: crate::OpaqueHostValue,
    pub dirty: bool,
    pub dirty_max_x: f64,
    pub dirty_max_y: f64,
    pub dirty_min_x: f64,
    pub dirty_min_y: f64,
    pub entries: crate::OpaqueHostValue,
    pub lru: Vec<f64>,
    pub max_glyphs: f64,
    pub metrics: GlyphMetrics,
    pub pack_bottom: f64,
    pub padding: f64,
    pub rasterize_options: GlyphRasterizeOptions,
    pub shelves: Vec<GlyphAtlasShelf>,
    pub surface: Surface,
}

// Source: upstream/packages/types/src/GlyphSource.ts:131 (sha256:8e4d8aabe5261fd2a81059c9e8298e9eef9beea41daee9fbb92c9c0e4b57d96d)
#[derive(Clone)]
pub struct GlyphAtlas {
    pub runtime: GlyphAtlasRuntime,
}
