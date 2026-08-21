// @generated from upstream/packages/bitmapfont/src/explainBitmapFontGlyph.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BitmapFont, BitmapFontGlyphExplanation};

// Source: upstream/packages/bitmapfont/src/explainBitmapFontGlyph.ts:14 (sha256:53a44c87b3ba705e9b735a434cac49a8950b712851e42fcf47f9463078497948)
#[derive(Clone, Default)]
struct ExplainBitmapFontGlyphSynthesizedRecord532822740 {
    __flight_identity: std::sync::Arc<()>,
    glyph_height: f64,
    glyph_width: f64,
    page: f64,
    page_count: f64,
}
impl PartialEq for ExplainBitmapFontGlyphSynthesizedRecord532822740 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn explain_bitmap_font_glyph(font: &BitmapFont, codepoint: f64) -> BitmapFontGlyphExplanation {
    let page_count = (font.pages.len() as f64);
    let glyph = font
        .glyphs
        .iter()
        .find(|(entry_key, _)| entry_key == &codepoint)
        .map(|(_, value)| value.clone());
    if (glyph).is_none() {
        return BitmapFontGlyphExplanation {
            __flight_identity: std::sync::Arc::new(()),
            glyph_height: 0.0_f64,
            glyph_width: 0.0_f64,
            page: (-1.0_f64),
            page_count: page_count,
            reason: "no-glyph".to_owned(),
            renderable: false,
        };
    }
    let shared = ExplainBitmapFontGlyphSynthesizedRecord532822740 {
        __flight_identity: std::sync::Arc::new(()),
        glyph_height: glyph.as_ref().unwrap().height,
        glyph_width: glyph.as_ref().unwrap().width,
        page: glyph.as_ref().unwrap().page,
        page_count: page_count,
    };
    if (page_count == 0.0_f64) {
        return {
            let __flight_spread_0 = (shared).clone();
            BitmapFontGlyphExplanation {
                __flight_identity: std::sync::Arc::new(()),
                renderable: false,
                reason: "no-pages".to_owned(),
                page: __flight_spread_0.page,
                page_count: __flight_spread_0.page_count,
                glyph_width: __flight_spread_0.glyph_width,
                glyph_height: __flight_spread_0.glyph_height,
            }
        };
    }
    if (glyph.as_ref().unwrap().width <= 0.0_f64) || (glyph.as_ref().unwrap().height <= 0.0_f64) {
        return {
            let __flight_spread_0 = (shared).clone();
            BitmapFontGlyphExplanation {
                __flight_identity: std::sync::Arc::new(()),
                renderable: false,
                reason: "empty-glyph".to_owned(),
                page: __flight_spread_0.page,
                page_count: __flight_spread_0.page_count,
                glyph_width: __flight_spread_0.glyph_width,
                glyph_height: __flight_spread_0.glyph_height,
            }
        };
    }
    return {
        let __flight_spread_0 = (shared).clone();
        BitmapFontGlyphExplanation {
            __flight_identity: std::sync::Arc::new(()),
            renderable: true,
            reason: "ok".to_owned(),
            page: __flight_spread_0.page,
            page_count: __flight_spread_0.page_count,
            glyph_width: __flight_spread_0.glyph_width,
            glyph_height: __flight_spread_0.glyph_height,
        }
    };
}
