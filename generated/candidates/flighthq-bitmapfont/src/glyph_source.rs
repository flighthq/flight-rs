// @generated from upstream/packages/bitmapfont/src/glyphSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_bitmap_font_glyph, get_bitmap_font_kerning, get_bitmap_font_metrics};
use flighthq_types::{BitmapFont, GlyphEntry, GlyphMetrics, GlyphSource, ImageResource};

// Source: upstream/packages/bitmapfont/src/glyphSource.ts:9 (sha256:392346d77a8fdad0b22d570ff4e32a2ac8675633a5bfd95fba3b097c1500fe36)
pub fn create_glyph_source_from_bitmap_font(font: BitmapFont) -> GlyphSource {
    return GlyphSource {
        __flight_identity: std::sync::Arc::new(()),
        get_glyph_atlas_image: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let font = font.clone();
            move |page: Option<f64>| -> Option<ImageResource> {
                return (font.pages[page as usize].image).clone();
            }
        })
            as Box<dyn FnMut(f64) -> Option<ImageResource> + Send + 'static>)),
        get_glyph_entry: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let font = font.clone();
            move |codepoint: f64| -> Option<GlyphEntry> {
                return get_bitmap_font_glyph(&font, codepoint);
            }
        })
            as Box<dyn FnMut(f64) -> Option<GlyphEntry> + Send + 'static>)),
        get_glyph_kerning: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let font = font.clone();
            move |left: f64, right: f64| -> f64 {
                return get_bitmap_font_kerning(&font, left, right);
            }
        })
            as Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>)),
        get_glyph_metrics: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let font = font.clone();
            move || -> GlyphMetrics {
                return get_bitmap_font_metrics(&font);
            }
        })
            as Box<dyn FnMut() -> GlyphMetrics + Send + 'static>)),
    };
}
