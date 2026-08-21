// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_bitmapfont::create_bitmap_font;
use flighthq_importdiagnostics::report_import_diagnostic;
use flighthq_types::{
    BitmapFont, BitmapFontCharRecord, BitmapFontData, BitmapFontGlyphData, BitmapFontKerningData,
    BitmapFontKerningRecord, BitmapFontParseOptions, BitmapFontRecord, GlyphMetrics,
    IMPORT_DIAGNOSTIC_SEVERITY as import_diagnostic_severity_constant, ImportDiagnostic,
    TextureAtlas,
};

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:26 (sha256:e1e716b298a1cf4d381df73304608996c45a4e4f0169125fdb7637c9e1fc57a5)
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
        if (!resolved
            .iter()
            .any(|(entry_key, _)| entry_key == &char.page))
        {
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
                .find(|(entry_key, _)| entry_key == &id)
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
        .map(|char: BitmapFontCharRecord| -> BitmapFontGlyphData {
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
        .collect::<Vec<_>>();
    let kerning: Vec<BitmapFontKerningData> = ((record.kernings).clone())
        .iter()
        .cloned()
        .map(|pair: BitmapFontKerningRecord| -> BitmapFontKerningData {
            BitmapFontKerningData {
                __flight_identity: std::sync::Arc::new(()),
                amount: pair.amount,
                left: pair.first,
                right: pair.second,
            }
        })
        .collect::<Vec<_>>();
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

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts:96 (sha256:aef8898ba71e7161729e0132fe94dd186beef3e79518357d1acd019452544835)
pub fn report_dropped_bitmap_font_records(
    diagnostics: &mut Option<Vec<ImportDiagnostic>>,
    origin: String,
    pages: f64,
    chars: f64,
    kernings: f64,
) -> () {
    if (pages > 0.0_f64) {
        report_import_diagnostic(
            diagnostics,
            (import_diagnostic_severity_constant.drop).clone(),
            "bmfont.page-unreadable".to_owned(),
            (origin).clone(),
            Some({
                let mut __flight_record = Vec::new();
                __flight_record.push((
                    "records".to_owned(),
                    crate::FlightUnion2::<bool, crate::FlightUnion2<f64, String>>::B(
                        crate::FlightUnion2::<f64, String>::A(pages),
                    ),
                ));
                __flight_record
            }),
        );
    }
    if (chars > 0.0_f64) {
        report_import_diagnostic(
            diagnostics,
            (import_diagnostic_severity_constant.drop).clone(),
            "bmfont.char-unreadable".to_owned(),
            (origin).clone(),
            Some({
                let mut __flight_record = Vec::new();
                __flight_record.push((
                    "records".to_owned(),
                    crate::FlightUnion2::<bool, crate::FlightUnion2<f64, String>>::B(
                        crate::FlightUnion2::<f64, String>::A(chars),
                    ),
                ));
                __flight_record
            }),
        );
    }
    if (kernings > 0.0_f64) {
        report_import_diagnostic(
            diagnostics,
            (import_diagnostic_severity_constant.drop).clone(),
            "bmfont.kerning-unreadable".to_owned(),
            (origin).clone(),
            Some({
                let mut __flight_record = Vec::new();
                __flight_record.push((
                    "records".to_owned(),
                    crate::FlightUnion2::<bool, crate::FlightUnion2<f64, String>>::B(
                        crate::FlightUnion2::<f64, String>::A(kernings),
                    ),
                ));
                __flight_record
            }),
        );
    }
}
