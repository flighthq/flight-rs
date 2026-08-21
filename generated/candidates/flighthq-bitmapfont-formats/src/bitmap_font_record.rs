// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_bitmapfont::create_bitmap_font;
use flighthq_types::{
    BitmapFont, BitmapFontCharRecord, BitmapFontData, BitmapFontGlyphData, BitmapFontKerningData,
    BitmapFontKerningRecord, BitmapFontParseOptions, BitmapFontRecord, GlyphMetrics, TextureAtlas,
};

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:23 (sha256:e1e716b298a1cf4d381df73304608996c45a4e4f0169125fdb7637c9e1fc57a5)
pub fn build_bitmap_font_from_record(
    record: BitmapFontRecord,
    options: Option<BitmapFontParseOptions>,
) -> Option<BitmapFont> {
    let resolve_page = options
        .as_ref()
        .and_then(|value| (value.resolve_page).clone());
    let mut resolved: Vec<(f64, TextureAtlas)> = Vec::new();
    let mut max_page_id = (-1.0_f64);
    if (resolve_page).is_some() {
        for page in ((record.pages).clone()).iter().cloned() {
            let atlas = {
                let __flight_callback = (resolve_page.as_ref().unwrap()).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()(page.id, (page.file).clone());
                __flight_result
            };
            if ((atlas).clone()).is_some() {
                {
                    let __flight_key = page.id;
                    let __flight_value = (atlas.as_ref().unwrap()).clone();
                    if let Some((_, value)) =
                        resolved.iter_mut().find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        resolved.push((__flight_key, __flight_value));
                    }
                };
                if (page.id > max_page_id) {
                    max_page_id = page.id;
                }
            }
        }
    }
    for char in ((record.chars).clone()).iter().cloned() {
        if (!resolved.iter().any(|(key, _)| key == &char.page)) {
            return None;
        }
        if (char.page > max_page_id) {
            max_page_id = char.page;
        }
    }
    let mut pages: Vec<TextureAtlas> = vec![];
    {
        let mut id = 0.0_f64;
        while (id <= max_page_id) {
            let atlas = resolved
                .iter()
                .find(|(key, _)| key == &id)
                .map(|(_, value)| value.clone());
            if ((atlas).clone()).is_some() {
                {
                    let __flight_index = (id) as usize;
                    let __flight_value = (atlas.as_ref().unwrap()).clone();
                    if __flight_index == pages.len() {
                        pages.push(__flight_value);
                    } else {
                        pages[__flight_index] = __flight_value;
                    }
                };
            }
            {
                id += 1.0;
                id
            };
        }
    }
    let glyphs: Vec<BitmapFontGlyphData> = ((record.chars).clone())
        .iter()
        .cloned()
        .map(|char: BitmapFontCharRecord| -> crate::OpaqueHostValue {
            crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                let __flight_key_0 = "advance".to_owned();
                let __flight_value_0 = {
                    let __flight_portable_source = char.xadvance;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_0)
                {
                    *__flight_existing = __flight_value_0;
                } else {
                    __flight_record.push((__flight_key_0, __flight_value_0));
                }
                let __flight_key_1 = "bearingX".to_owned();
                let __flight_value_1 = {
                    let __flight_portable_source = char.xoffset;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_1)
                {
                    *__flight_existing = __flight_value_1;
                } else {
                    __flight_record.push((__flight_key_1, __flight_value_1));
                }
                let __flight_key_2 = "bearingY".to_owned();
                let __flight_value_2 = {
                    let __flight_portable_source = (record.base - char.yoffset);
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_2)
                {
                    *__flight_existing = __flight_value_2;
                } else {
                    __flight_record.push((__flight_key_2, __flight_value_2));
                }
                let __flight_key_3 = "codepoint".to_owned();
                let __flight_value_3 = {
                    let __flight_portable_source = char.id;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_3)
                {
                    *__flight_existing = __flight_value_3;
                } else {
                    __flight_record.push((__flight_key_3, __flight_value_3));
                }
                let __flight_key_4 = "height".to_owned();
                let __flight_value_4 = {
                    let __flight_portable_source = char.height;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_4)
                {
                    *__flight_existing = __flight_value_4;
                } else {
                    __flight_record.push((__flight_key_4, __flight_value_4));
                }
                let __flight_key_5 = "page".to_owned();
                let __flight_value_5 = {
                    let __flight_portable_source = char.page;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_5)
                {
                    *__flight_existing = __flight_value_5;
                } else {
                    __flight_record.push((__flight_key_5, __flight_value_5));
                }
                let __flight_key_6 = "width".to_owned();
                let __flight_value_6 = {
                    let __flight_portable_source = char.width;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_6)
                {
                    *__flight_existing = __flight_value_6;
                } else {
                    __flight_record.push((__flight_key_6, __flight_value_6));
                }
                let __flight_key_7 = "x".to_owned();
                let __flight_value_7 = {
                    let __flight_portable_source = char.x;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_7)
                {
                    *__flight_existing = __flight_value_7;
                } else {
                    __flight_record.push((__flight_key_7, __flight_value_7));
                }
                let __flight_key_8 = "y".to_owned();
                let __flight_value_8 = {
                    let __flight_portable_source = char.y;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_8)
                {
                    *__flight_existing = __flight_value_8;
                } else {
                    __flight_record.push((__flight_key_8, __flight_value_8));
                }
                __flight_record
            })
        })
        .collect();
    let kerning: Vec<BitmapFontKerningData> = ((record.kernings).clone())
        .iter()
        .cloned()
        .map(|pair: BitmapFontKerningRecord| -> crate::OpaqueHostValue {
            crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                let __flight_key_0 = "amount".to_owned();
                let __flight_value_0 = {
                    let __flight_portable_source = pair.amount;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_0)
                {
                    *__flight_existing = __flight_value_0;
                } else {
                    __flight_record.push((__flight_key_0, __flight_value_0));
                }
                let __flight_key_1 = "left".to_owned();
                let __flight_value_1 = {
                    let __flight_portable_source = pair.first;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_1)
                {
                    *__flight_existing = __flight_value_1;
                } else {
                    __flight_record.push((__flight_key_1, __flight_value_1));
                }
                let __flight_key_2 = "right".to_owned();
                let __flight_value_2 = {
                    let __flight_portable_source = pair.second;
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(existing, _)| existing == &__flight_key_2)
                {
                    *__flight_existing = __flight_value_2;
                } else {
                    __flight_record.push((__flight_key_2, __flight_value_2));
                }
                __flight_record
            })
        })
        .collect();
    let mut data: BitmapFontData = BitmapFontData {
        __flight_identity: std::sync::Arc::new(()),
        encoding: Some((record.encoding).clone()),
        glyphs: (glyphs).clone(),
        kerning: Some((kerning).clone()),
        metrics: GlyphMetrics {
            __flight_identity: std::sync::Arc::new(()),
            ascent: record.base,
            descent: (record.line_height - record.base),
            line_gap: 0.0_f64,
        },
        pages: (pages).clone(),
    };
    return Some(create_bitmap_font(&data));
}
