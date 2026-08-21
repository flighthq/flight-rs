// @generated from upstream/packages/bitmapfont/src/bitmapFontGlyphSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_bitmap_font_glyph, get_bitmap_font_kerning, get_bitmap_font_metrics};
use flighthq_types::{BitmapFont, GlyphEntry, GlyphMetrics, GlyphSource, TextureSource};

// Source: upstream/packages/bitmapfont/src/bitmapFontGlyphSource.ts:9 (sha256:49668de2c17af86da17fecb3ff685cd18bbe93a329b104024861d13577224929)
pub fn create_glyph_source_from_bitmap_font(font: BitmapFont) -> GlyphSource {
    return GlyphSource {
        __flight_identity: std::sync::Arc::new(()),
        get_glyph_atlas_image: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let font = font.clone();
            move |page: Option<f64>| -> Option<TextureSource> {
                let page = page.unwrap_or(0.0_f64);
                let texture = (font.pages[page as usize].texture).clone();
                return if (texture.as_ref().map(|value| (value.dimension).clone()))
                    == Some("2d".to_owned())
                {
                    (texture.as_ref().unwrap().source).clone()
                } else {
                    None
                };
            }
        })
            as Box<dyn FnMut(Option<f64>) -> Option<TextureSource> + Send + 'static>)),
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
