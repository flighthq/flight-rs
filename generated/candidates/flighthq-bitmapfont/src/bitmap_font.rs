// @generated from upstream/packages/bitmapfont/src/bitmapFont.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    BitmapFont, BitmapFontData, BitmapFontKerningPair, GlyphEntry, GlyphMetrics, TextureAtlas,
};

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:17 (sha256:66957a022265a885115b24bb16a0362db2be301b4d22258ea640fec714b6e3d4)
pub fn create_bitmap_font(data: &BitmapFontData) -> BitmapFont {
    let page_count = (data.pages.len() as f64);
    let mut glyphs: Vec<(f64, GlyphEntry)> = Vec::new();
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
                page: resolve_bitmap_font_glyph_page(glyph.codepoint, page, page_count),
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
    let mut kerning: Vec<(f64, f64)> = Vec::new();
    if ((data.kerning).clone()).is_some() {
        for pair in ((data.kerning).clone())
            .as_ref()
            .expect("TypeScript nullable iterable was not narrowed")
            .iter()
            .cloned()
        {
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

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:54 (sha256:75b023c1bcfa773974bfe875b59885986814b99a215accccfaad4764acbd12d2)
pub fn get_bitmap_font_glyph(font: &BitmapFont, codepoint: f64) -> Option<GlyphEntry> {
    return font
        .glyphs
        .iter()
        .find(|(entry_key, _)| entry_key == &codepoint)
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:60 (sha256:88c51c8d1739991d7c3bfe57d5552b9c03eea166fe41207422bfdd0943064341)
pub fn get_bitmap_font_kerning(font: &BitmapFont, left: f64, right: f64) -> f64 {
    return (font
        .kerning
        .iter()
        .find(|(entry_key, _)| entry_key == &pack_bitmap_font_kerning_key(left, right))
        .map(|(_, value)| value.clone()))
    .unwrap_or(0.0_f64);
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:65 (sha256:0d374b086f8e9511f00257270132c432ae3fd0b8b4ed72b715af295eb4b1efcb)
pub fn get_bitmap_font_metrics(font: &BitmapFont) -> GlyphMetrics {
    return (font.metrics).clone();
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:72 (sha256:218dcdd14a6c9b700527ee366c96640271fe60814ffbfcb3213fbf11a6abc5d9)
pub fn get_bitmap_font_page(font: &BitmapFont, page: Option<f64>) -> Option<TextureAtlas> {
    let page = page.unwrap_or(0.0_f64);
    return Some(font.pages[page as usize].clone());
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:78 (sha256:f628c8a8c889b69bafd24b20239f28c2ec03f10b9f4dd8e9e4564ad4b2656b43)
pub fn get_bitmap_font_pages(font: &BitmapFont) -> Vec<TextureAtlas> {
    return (font.pages).clone();
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:86 (sha256:4d6204561fc95c737865c087ca0d3f9e26fabee3f94271b774a070bb80b9af8e)
pub fn has_bitmap_font_glyph(font: &BitmapFont, codepoint: f64) -> bool {
    return font
        .glyphs
        .iter()
        .any(|(entry_key, _)| entry_key == &codepoint);
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:103 (sha256:d570e7e666f09e96f4f9aeade7c373138e5a13f655e9c0b5ab3537def98ea66b)
pub fn pack_bitmap_font_kerning_key(left: f64, right: f64) -> f64 {
    return ((left * UNICODE_CODEPOINT_SPACE) + right);
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:110 (sha256:ae05998183f5a21bcc27c0d4d5ebae600cf2a37717cea4beb1b83f3eddfc1a59)
pub fn set_bitmap_font_guard(
    guard: &Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64, f64) -> () + Send + 'static>>>,
    >,
) -> () {
    (*_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:121 (sha256:7c202829dbd290e6007fb559a337c1be5e5f1a6eb25086a3a2ed6586827867c2)
pub fn unpack_bitmap_font_kerning_key(
    key: f64,
    out: &mut BitmapFontKerningPair,
) -> BitmapFontKerningPair {
    out.left = (key / UNICODE_CODEPOINT_SPACE).floor();
    out.right = (key % UNICODE_CODEPOINT_SPACE);
    return out.clone();
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:131 (sha256:10e33cbe47b5df05572a8d9ff6f3d21fb3b65f96b4fb2a5b73ca8e5e01ddf03c)
fn resolve_bitmap_font_glyph_page(codepoint: f64, page: f64, page_count: f64) -> f64 {
    if (page >= 0.0_f64) && (page < page_count) {
        return page;
    }
    {
        let __flight_callback = (*_GUARD.lock().unwrap()).clone();
        __flight_callback.as_ref().map(|callback| {
            callback.lock().unwrap()("page-out-of-range".to_owned(), codepoint, page)
        })
    };
    return 0.0_f64;
}

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:138 (sha256:d8cb40fbbffc531e5b027c0b36e7bd0d3d1a8c83379c57d024776a6e0b121be4)
const UNICODE_CODEPOINT_SPACE: f64 = 1114112.0_f64;

// Source: upstream/packages/bitmapfont/src/bitmapFont.ts:140 (sha256:9a316f6b8a74d2685d817b8cb59c2d7c55a861c76b8ec93e338151fc6fae29d9)
static _GUARD: std::sync::LazyLock<
    std::sync::Mutex<
        Option<
            std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(String, f64, f64) -> () + Send + 'static>>,
            >,
        >,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
