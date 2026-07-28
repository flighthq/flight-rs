// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_bitmapfont::create_bitmap_font;
use flighthq_types::{
    BitmapFont, BitmapFontData, BitmapFontEncoding, BitmapFontGlyphData, BitmapFontKerningData,
    BitmapFontParseOptions, GlyphMetrics, TextureAtlas,
};

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:16 (sha256:182ceaddee1847336c2926bdbcbd220df1fad5ad725ec3b0cc21f45dd02da713)
#[derive(Clone)]
pub struct BitmapFontCharRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub id: f64,
    pub page: f64,
    pub width: f64,
    pub x: f64,
    pub xadvance: f64,
    pub xoffset: f64,
    pub y: f64,
    pub yoffset: f64,
}
impl PartialEq for BitmapFontCharRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:30 (sha256:4f116fa26e71d8730c5448413eb7005c1f0f4539e88c6d0da93c80091aa64f8d)
#[derive(Clone)]
pub struct BitmapFontKerningRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub amount: f64,
    pub first: f64,
    pub second: f64,
}
impl PartialEq for BitmapFontKerningRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:38 (sha256:90e36dee9bbb4651c12cb4465b07b9039fbe398c4ca8975e052c6179e700355c)
#[derive(Clone)]
pub struct BitmapFontPageRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub file: String,
    pub id: f64,
}
impl PartialEq for BitmapFontPageRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:48 (sha256:ae30b6e490fa58d911ccf7d6ad417734cdf4bcb81f934892511ccb503ffd7231)
#[derive(Clone)]
pub struct BitmapFontRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub base: f64,
    pub chars: Vec<BitmapFontCharRecord>,
    pub encoding: BitmapFontEncoding,
    pub kernings: Vec<BitmapFontKerningRecord>,
    pub line_height: f64,
    pub pages: Vec<BitmapFontPageRecord>,
}
impl PartialEq for BitmapFontRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:68 (sha256:e1e716b298a1cf4d381df73304608996c45a4e4f0169125fdb7637c9e1fc57a5)
pub fn build_bitmap_font_from_record(
    record: BitmapFontRecord,
    options: Option<BitmapFontParseOptions>,
) -> Option<BitmapFont> {
    let resolve_page = options
        .as_ref()
        .and_then(|value| (value.resolve_page).clone());
    let mut resolved = Vec::new();
    let mut max_page_id = (-1.0_f64);
    if (resolve_page).is_some() {
        for page in ((record.pages).clone()).iter().cloned() {
            let atlas = {
                let __flight_callback = (resolve_page.as_ref().unwrap()).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()(page.id, (page.file).clone());
                __flight_result
            };
            if (atlas).is_some() {
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
            if (atlas).is_some() {
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
            BitmapFontGlyphData {
                __flight_identity: std::sync::Arc::new(()),
                advance: char.xadvance,
                bearing_x: char.xoffset,
                bearing_y: (record.base - char.yoffset),
                codepoint: char.id,
                height: char.height,
                page: Some(char.page),
                width: char.width,
                x: char.x,
                y: char.y,
            }
        })
        .collect();
    let kerning: Vec<BitmapFontKerningData> = ((record.kernings).clone())
        .iter()
        .cloned()
        .map(|pair: BitmapFontKerningRecord| -> crate::OpaqueHostValue {
            BitmapFontKerningData {
                __flight_identity: std::sync::Arc::new(()),
                amount: pair.amount,
                left: pair.first,
                right: pair.second,
            }
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
