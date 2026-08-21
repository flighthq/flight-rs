// @generated from upstream/packages/bitmapfont/src/summarizeBitmapFont.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_textureatlas::get_texture_atlas_byte_size;
use flighthq_types::{BitmapFont, BitmapFontSummary, Texture2D, TextureAtlasRegion};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: Option<Texture2D>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmapfont/src/summarizeBitmapFont.ts:15 (sha256:6d8701f68ebed960e032d89571fffafe00ee872eb0234544769c9cd9123f0c73)
pub fn summarize_bitmap_font(font: &BitmapFont) -> BitmapFontSummary {
    let mut byte_size = 0.0_f64;
    for page in ((font.pages).clone()).iter().cloned() {
        byte_size += get_texture_atlas_byte_size(&page);
    }
    let mut min_codepoint = (-1.0_f64);
    let mut max_codepoint = (-1.0_f64);
    for codepoint in (font
        .glyphs
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        if (min_codepoint < 0.0_f64) || (codepoint < min_codepoint) {
            min_codepoint = codepoint;
        }
        if (codepoint > max_codepoint) {
            max_codepoint = codepoint;
        }
    }
    return BitmapFontSummary {
        __flight_identity: std::sync::Arc::new(()),
        byte_size: byte_size,
        glyph_count: (font.glyphs.len() as f64),
        kerning_pair_count: (font.kerning.len() as f64),
        max_codepoint: max_codepoint,
        min_codepoint: min_codepoint,
        page_count: (font.pages.len() as f64),
    };
}
