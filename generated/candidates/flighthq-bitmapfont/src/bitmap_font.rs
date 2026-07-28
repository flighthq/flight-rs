// @generated from upstream/packages/bitmapfont/src/bitmapFont.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BitmapFont, BitmapFontData, GlyphEntry, GlyphMetrics, TextureAtlas};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:10 (sha256:ddd67650e85a2fb766553fbfe3db270e9d61164942d225f66d3fbe92cee1aa9f)
pub fn create_bitmap_font(data: &BitmapFontData) -> BitmapFont {
    let page_count = (data.pages.len() as f64);
    let mut glyphs = Vec::new();
    for glyph in ((data.glyphs).clone()).iter().cloned() {
        let page = (glyph.page).unwrap_or(0.0_f64);
        {
            let __flight_key = glyph.codepoint;
            let __flight_value = GlyphEntry {
                __flight_identity: std::sync::Arc::new(()),
                advance: glyph.advance,
                bearing_x: glyph.bearing_x,
                bearing_y: glyph.bearing_y,
                height: glyph.height,
                page: if ((page >= 0.0_f64) && (page < page_count)) {
                    page
                } else {
                    0.0_f64
                },
                width: glyph.width,
                x: glyph.x,
                y: glyph.y,
            };
            if let Some((_, value)) = glyphs.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                glyphs.push((__flight_key, __flight_value));
            }
        };
    }
    let mut kerning = Vec::new();
    if ((data.kerning).clone()).is_some() {
        for pair in ((data.kerning).clone()).iter().cloned() {
            {
                let __flight_key = pack_bitmap_font_kerning_key(pair.left, pair.right);
                let __flight_value = pair.amount;
                if let Some((_, value)) = kerning.iter_mut().find(|(key, _)| key == &__flight_key) {
                    *value = __flight_value;
                } else {
                    kerning.push((__flight_key, __flight_value));
                }
            };
        }
    }
    return BitmapFont {
        __flight_identity: std::sync::Arc::new(()),
        encoding: ((data.encoding).clone()).unwrap_or("raster".to_owned()),
        glyphs: (glyphs).clone(),
        kerning: (kerning).clone(),
        metrics: GlyphMetrics {
            __flight_identity: std::sync::Arc::new(()),
            ascent: data.metrics.ascent,
            descent: data.metrics.descent,
            line_gap: data.metrics.line_gap,
        },
        pages: ((data.pages).clone()).clone(),
    };
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:47 (sha256:75b023c1bcfa773974bfe875b59885986814b99a215accccfaad4764acbd12d2)
pub fn get_bitmap_font_glyph(font: &BitmapFont, codepoint: f64) -> Option<GlyphEntry> {
    return font
        .glyphs
        .iter()
        .find(|(key, _)| key == &codepoint)
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:53 (sha256:88c51c8d1739991d7c3bfe57d5552b9c03eea166fe41207422bfdd0943064341)
pub fn get_bitmap_font_kerning(font: &BitmapFont, left: f64, right: f64) -> f64 {
    return (font
        .kerning
        .iter()
        .find(|(key, _)| key == &pack_bitmap_font_kerning_key(left, right))
        .map(|(_, value)| value.clone()))
    .unwrap_or(0.0_f64);
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:58 (sha256:0d374b086f8e9511f00257270132c432ae3fd0b8b4ed72b715af295eb4b1efcb)
pub fn get_bitmap_font_metrics(font: &BitmapFont) -> GlyphMetrics {
    return (font.metrics).clone();
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:65 (sha256:218dcdd14a6c9b700527ee366c96640271fe60814ffbfcb3213fbf11a6abc5d9)
pub fn get_bitmap_font_page(font: &BitmapFont, page: Option<f64>) -> Option<TextureAtlas> {
    let page = page.unwrap_or(0.0_f64);
    return Some(font.pages[page as usize].clone());
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:71 (sha256:f628c8a8c889b69bafd24b20239f28c2ec03f10b9f4dd8e9e4564ad4b2656b43)
pub fn get_bitmap_font_pages(font: &BitmapFont) -> Vec<TextureAtlas> {
    return (font.pages).clone();
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:78 (sha256:23950b42779d0e51645ae45fb045cab71358cb9fc7dcdb18bc01bad5737b6844)
fn pack_bitmap_font_kerning_key(left: f64, right: f64) -> f64 {
    return (__flight_js_to_i32(
        __flight_js_to_i32(left).wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) | __flight_js_to_i32(right)) as f64;
}
