// @generated from upstream/packages/glyphatlas/src/glyphAtlas.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_glyph_rasterizer_backend;
use flighthq_bitmap::create_bitmap;
use flighthq_types::{
    Bitmap, GlyphAtlas, GlyphAtlasOptions, GlyphAtlasRuntime, GlyphMetrics, GlyphRasterizeOptions,
    GlyphRasterizerBackend,
};

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:18 (sha256:63fdcec55e56b320bf56c26ad48ba88b2dbbc667cb21f241aec4c6383135d648)
#[derive(Clone, Default)]
struct CreateGlyphAtlasRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGlyphAtlasRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateGlyphAtlasSynthesizedRecord773034950 {
    __flight_identity: std::sync::Arc<()>,
    font_style: Option<String>,
}
impl PartialEq for CreateGlyphAtlasSynthesizedRecord773034950 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateGlyphAtlasSynthesizedRecord3400484059 {
    __flight_identity: std::sync::Arc<()>,
    font_weight: Option<String>,
}
impl PartialEq for CreateGlyphAtlasSynthesizedRecord3400484059 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_glyph_atlas(options: &GlyphAtlasOptions) -> GlyphAtlas {
    let padding = (options.padding).clone().unwrap_or(1.0_f64);
    let rasterizer_backend = ((options.rasterizer_backend).clone())
        .clone()
        .unwrap_or(get_glyph_rasterizer_backend());
    let rasterize_options: GlyphRasterizeOptions = {
        let __flight_spread_2 = if ((options.font_style).clone()).is_some() {
            CreateGlyphAtlasSynthesizedRecord773034950 {
                __flight_identity: std::sync::Arc::new(()),
                font_style: (options.font_style).clone(),
            }
        } else {
            CreateGlyphAtlasSynthesizedRecord773034950 {
                __flight_identity: std::sync::Arc::new(()),
                font_style: Default::default(),
            }
        };
        let __flight_spread_3 = if ((options.font_weight).clone()).is_some() {
            CreateGlyphAtlasSynthesizedRecord3400484059 {
                __flight_identity: std::sync::Arc::new(()),
                font_weight: (options.font_weight).clone(),
            }
        } else {
            CreateGlyphAtlasSynthesizedRecord3400484059 {
                __flight_identity: std::sync::Arc::new(()),
                font_weight: Default::default(),
            }
        };
        GlyphRasterizeOptions {
            __flight_identity: std::sync::Arc::new(()),
            font_family: (options.font_family).clone(),
            font_size: options.font_size,
            font_style: (__flight_spread_2.font_style).clone(),
            font_weight: (__flight_spread_3.font_weight).clone(),
        }
    };
    return GlyphAtlas {
        __flight_identity: std::sync::Arc::new(()),
        runtime: GlyphAtlasRuntime {
            __flight_identity: std::sync::Arc::new(()),
            bitmaps: Vec::new(),
            dirty: false,
            dirty_max_x: 0.0_f64,
            dirty_max_y: 0.0_f64,
            dirty_min_x: 0.0_f64,
            dirty_min_y: 0.0_f64,
            entries: Vec::new(),
            lru: Vec::new(),
            max_area: (options.max_area).clone().unwrap_or(0.0_f64),
            max_bytes: (options.max_bytes).clone().unwrap_or(0.0_f64),
            max_glyphs: (options.max_glyphs).clone().unwrap_or(0.0_f64),
            occupied_area: 0.0_f64,
            retained_bytes: 0.0_f64,
            metrics: _resolve_glyph_atlas_metrics(&rasterizer_backend, &rasterize_options),
            pack_bottom: padding,
            padding: padding,
            rasterizer_backend: (rasterizer_backend).clone(),
            rasterize_options: (rasterize_options).clone(),
            shelves: vec![],
            bitmap: create_bitmap(options.width, options.height, None),
        },
    };
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:57 (sha256:02144fc8e4a8bb19c5b5ae13610e05379634175e86dc693b7a4afa7e8c1d9fbb)
pub fn derive_glyph_metrics_from_font_size(font_size: f64) -> GlyphMetrics {
    return GlyphMetrics {
        __flight_identity: std::sync::Arc::new(()),
        ascent: (font_size * 0.8_f64),
        descent: (font_size * 0.2_f64),
        line_gap: 0.0_f64,
    };
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:70 (sha256:019c606755f88258e5c66fce259917c86faabb11dd3136dac46919ab0e53fdde)
pub fn dispose_glyph_atlas(atlas: &mut GlyphAtlas) -> () {
    atlas.runtime.entries.clear();
    atlas.runtime.bitmaps.clear();
    atlas.runtime.occupied_area = 0.0_f64;
    atlas.runtime.retained_bytes = 0.0_f64;
    atlas.runtime.lru.clear();
    atlas.runtime.shelves.clear();
    atlas.runtime.pack_bottom = atlas.runtime.padding;
    atlas.runtime.dirty = false;
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:84 (sha256:67e6cab1efb885f6de4dc4eb8b1cdbc1e2e26d618ea6c1d662cb3c5b488995ee)
pub fn get_glyph_atlas_bitmap(atlas: &GlyphAtlas) -> Bitmap {
    return (atlas.runtime.bitmap).clone();
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:91 (sha256:b7dcd4380ef728b8ede3538f6dd2952a5252a5339522ebbdb1c1ed7fc49e6e5f)
fn _resolve_glyph_atlas_metrics(
    backend: &GlyphRasterizerBackend,
    rasterize_options: &GlyphRasterizeOptions,
) -> GlyphMetrics {
    let measured = {
        let __flight_callback = (backend.measure_metrics).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()((*rasterize_options).clone()))
    };
    return (measured)
        .clone()
        .unwrap_or(derive_glyph_metrics_from_font_size(
            rasterize_options.font_size,
        ));
}
