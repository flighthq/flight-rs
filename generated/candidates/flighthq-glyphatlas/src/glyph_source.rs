// @generated from upstream/packages/glyphatlas/src/glyphSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    get_glyph_atlas_entry, get_glyph_atlas_kerning, get_glyph_atlas_metrics,
    get_glyph_atlas_surface,
};
use flighthq_types::{GlyphAtlas, GlyphEntry, GlyphMetrics, GlyphSource, ImageResource};

// Source: upstream/packages/glyphatlas/src/glyphSource.ts:11 (sha256:c68b339a60708c926a72ff35f655b01f3c62e2185400bf2c3d2cde737c2719b7)
pub fn create_glyph_source_from_glyph_atlas(mut atlas: GlyphAtlas) -> GlyphSource {
    return GlyphSource {
        __flight_identity: std::sync::Arc::new(()),
        get_glyph_atlas_image: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut atlas = atlas.clone();
            move |page: Option<f64>| -> Option<ImageResource> {
                let page = page.unwrap_or(0.0_f64);
                return if (page == 0.0_f64) {
                    Some(get_glyph_atlas_surface(&atlas))
                } else {
                    None
                };
            }
        })
            as Box<dyn FnMut(Option<f64>) -> Option<ImageResource> + Send + 'static>)),
        get_glyph_entry: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut atlas = atlas.clone();
            move |codepoint: f64| -> Option<GlyphEntry> {
                return get_glyph_atlas_entry(&mut atlas, codepoint);
            }
        })
            as Box<dyn FnMut(f64) -> Option<GlyphEntry> + Send + 'static>)),
        get_glyph_kerning: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut atlas = atlas.clone();
            move |left: f64, right: f64| -> f64 {
                return get_glyph_atlas_kerning(&atlas, left, right);
            }
        })
            as Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>)),
        get_glyph_metrics: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut atlas = atlas.clone();
            move || -> GlyphMetrics {
                return get_glyph_atlas_metrics(&atlas);
            }
        })
            as Box<dyn FnMut() -> GlyphMetrics + Send + 'static>)),
    };
}
