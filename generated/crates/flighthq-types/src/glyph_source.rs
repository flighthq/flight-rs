// @generated from upstream/packages/types/src/GlyphSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Bitmap, EntityRuntime, TextureSource};

// Source: upstream/packages/types/src/GlyphSource.ts:16 (sha256:19be199f93c89234fcaabd477fb5473e8bb63201b743aa3a795849cfcb3f74b8)
#[derive(Clone)]
pub struct GlyphSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub get_glyph_atlas_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<f64>) -> Option<TextureSource> + Send + 'static>>,
    >,
    pub get_glyph_entry: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> Option<GlyphEntry> + Send + 'static>>,
    >,
    pub get_glyph_kerning:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>>>,
    pub get_glyph_layout_version:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub get_glyph_metrics:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> GlyphMetrics + Send + 'static>>>,
}
impl PartialEq for GlyphSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlyphSource {
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

// Source: upstream/packages/types/src/GlyphSource.ts:49 (sha256:35b568c220bc7e447eb59d00ddffd58a2c092b969eb3e20a71199392e2edb9a4)
#[derive(Clone, Default)]
pub struct GlyphEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advance: f64,
    pub bearing_x: f64,
    pub bearing_y: f64,
    pub height: f64,
    pub page: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for GlyphEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:65 (sha256:0eb6596ab428f58bb83ef3c812867a6539e640d5c18b243b4e76a11c5b205617)
#[derive(Clone, Default)]
pub struct GlyphMetrics {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ascent: f64,
    pub descent: f64,
    pub line_gap: f64,
}
impl PartialEq for GlyphMetrics {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:75 (sha256:f7176fd3653d1eed66fa8e264d284cacdba9c577464213d51d710bf2f18bea8b)
#[derive(Clone, Default)]
pub struct GlyphRasterizedBitmap {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advance: f64,
    pub bearing_x: f64,
    pub bearing_y: f64,
    pub height: f64,
    pub pixels: Vec<u8>,
    pub width: f64,
}
impl PartialEq for GlyphRasterizedBitmap {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:86 (sha256:41c7407d7cd069a73c5d57fee671d76247ab5f92dfb2173c052ae8224759b857)
#[derive(Clone, Default)]
pub struct GlyphRasterizeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub font_family: String,
    pub font_size: f64,
    pub font_style: Option<String>,
    pub font_weight: Option<crate::FlightUnion2<f64, String>>,
}
impl PartialEq for GlyphRasterizeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:96 (sha256:32723ad35b8bae9c2d277785972218232bee01abcf9ec7aaaa774e6d7d7f8d00)
#[derive(Clone)]
pub struct HostGlyphRasterizerCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rasterize: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(f64, GlyphRasterizeOptions) -> Option<GlyphRasterizedBitmap>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub measure_metrics: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(GlyphRasterizeOptions) -> Option<GlyphMetrics> + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for HostGlyphRasterizerCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:108 (sha256:9939a54c61739f09cef4711f1dbe525c20858cb956b26d72c591d6a06120a063)
#[derive(Clone)]
pub struct GlyphAtlasOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub font_family: String,
    pub font_size: f64,
    pub font_style: Option<String>,
    pub font_weight: Option<String>,
    pub height: f64,
    pub max_area: Option<f64>,
    pub max_bytes: Option<f64>,
    pub max_glyphs: Option<f64>,
    pub padding: Option<f64>,
    pub rasterizer_backend: HostGlyphRasterizerCapability,
    pub width: f64,
}
impl PartialEq for GlyphAtlasOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:133 (sha256:dc15e695a4ce542d6a7bf3906d5ca2b88fa4114d6404823a05cb8eb49a786bfc)
#[derive(Clone, Default)]
pub struct GlyphAtlasShelf {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cursor_x: f64,
    pub height: f64,
    pub y: f64,
}
impl PartialEq for GlyphAtlasShelf {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:143 (sha256:77a8a7fb0248974f98ff4d0b9e6324949661f48bd0fd798e225b34eeed9008a2)
#[derive(Clone)]
pub struct GlyphAtlasRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bitmaps: Vec<(f64, GlyphRasterizedBitmap)>,
    pub dirty: bool,
    pub dirty_max_x: f64,
    pub dirty_max_y: f64,
    pub dirty_min_x: f64,
    pub dirty_min_y: f64,
    pub entries: Vec<(f64, GlyphEntry)>,
    pub lru: Vec<(f64, bool)>,
    pub layout_version: f64,
    pub max_area: f64,
    pub max_bytes: f64,
    pub max_glyphs: f64,
    pub occupied_area: f64,
    pub retained_bytes: f64,
    pub metrics: GlyphMetrics,
    pub pack_bottom: f64,
    pub padding: f64,
    pub rasterizer_backend: HostGlyphRasterizerCapability,
    pub rasterize_options: GlyphRasterizeOptions,
    pub shelves: Vec<GlyphAtlasShelf>,
    pub bitmap: Bitmap,
}
impl PartialEq for GlyphAtlasRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphSource.ts:185 (sha256:f67895c7dde73a5342c7cbc4940ef5ede0bf9bd3c4deaa0c7706ce63b5393bf2)
#[derive(Clone)]
pub struct GlyphAtlas {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub runtime: GlyphAtlasRuntime,
}
impl PartialEq for GlyphAtlas {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlyphAtlas {
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

// Source: upstream/packages/types/src/GlyphSource.ts:192 (sha256:146ba29345ed86a2d43b55bc642c4dbee418c10e71fc76a69a089e053846b028)
pub type GlyphRasterizerOperation = HostGlyphRasterizerCapability;
