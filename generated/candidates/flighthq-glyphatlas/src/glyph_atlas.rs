// @generated from upstream/packages/glyphatlas/src/glyphAtlas.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_surface::create_surface;
use flighthq_types::{
    GlyphAtlas, GlyphAtlasOptions, GlyphAtlasRuntime, GlyphMetrics, GlyphRasterizeOptions, Surface,
};

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:9 (sha256:372b121630af8d8b1a0bff8d3b52a0b84160263be6c37cc49772bdb021c3669c)
pub fn create_glyph_atlas(options: &GlyphAtlasOptions) -> GlyphAtlas {
    let padding = (options.padding).unwrap_or(1.0_f64);
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
            lru: vec![],
            max_glyphs: (options.max_glyphs).unwrap_or(0.0_f64),
            metrics: derive_glyph_metrics_from_font_size(options.font_size),
            pack_bottom: padding,
            padding: padding,
            rasterize_options: GlyphRasterizeOptions {
                __flight_identity: std::sync::Arc::new(()),
                font_family: (options.font_family).clone(),
                font_size: options.font_size,
                font_style: None,
                font_weight: None,
            },
            shelves: vec![],
            surface: create_surface(options.width, options.height, None),
        },
    };
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:38 (sha256:02144fc8e4a8bb19c5b5ae13610e05379634175e86dc693b7a4afa7e8c1d9fbb)
pub fn derive_glyph_metrics_from_font_size(font_size: f64) -> GlyphMetrics {
    return GlyphMetrics {
        __flight_identity: std::sync::Arc::new(()),
        ascent: (font_size * 0.8_f64),
        descent: (font_size * 0.2_f64),
        line_gap: 0.0_f64,
    };
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:51 (sha256:0adf94879e960a588b16e32c50fa1b3deeb99b6686ff2ba4158e0fbadacab06f)
pub fn dispose_glyph_atlas(atlas: &mut GlyphAtlas) -> () {
    atlas.runtime.entries.clear();
    atlas.runtime.bitmaps.clear();
    atlas.runtime.lru.clear();
    atlas.runtime.shelves.clear();
    atlas.runtime.pack_bottom = atlas.runtime.padding;
    atlas.runtime.dirty = false;
}

// Source: upstream/packages/glyphatlas/src/glyphAtlas.ts:63 (sha256:b8a5e96b823a1b8017a4454d119e430996f1e2cfa6db9f7338ef8abc5f5db65e)
pub fn get_glyph_atlas_surface(atlas: &GlyphAtlas) -> Surface {
    return (atlas.runtime.surface).clone();
}
