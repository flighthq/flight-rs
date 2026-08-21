// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{build_bitmap_font_from_record, report_dropped_bitmap_font_records};
use flighthq_bitmapfont::{get_bitmap_font_metrics, unpack_bitmap_font_kerning_key};
use flighthq_types::{
    BitmapFont, BitmapFontCharRecord, BitmapFontKerningPair, BitmapFontKerningRecord,
    BitmapFontPageRecord, BitmapFontParseOptions, BitmapFontRecord, ImportDiagnostic,
};

#[inline]

fn __flight_string_slice(value: &str, start: f64, end: Option<f64>) -> String {
    let value: Vec<u16> = value.encode_utf16().collect();
    let length = value.len();
    let relative = |index: f64| -> usize {
        if index.is_nan() {
            0
        } else if index < 0.0_f64 {
            length.saturating_sub((-index.trunc()) as usize)
        } else {
            (index.trunc() as usize).min(length)
        }
    };
    let start = relative(start);
    let end = end.map_or(length, relative);
    String::from_utf16_lossy(&value[start..end.max(start)])
}

#[inline]

fn __flight_number_from_string(value: &str) -> f64 {
    let value = value.trim();
    if value.is_empty() {
        return 0.0_f64;
    }
    match value {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    let prefixed = if let Some(digits) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        Some((digits, 16_u32))
    } else if let Some(digits) = value
        .strip_prefix("0b")
        .or_else(|| value.strip_prefix("0B"))
    {
        Some((digits, 2_u32))
    } else {
        value
            .strip_prefix("0o")
            .or_else(|| value.strip_prefix("0O"))
            .map(|digits| (digits, 8_u32))
    };
    if let Some((digits, radix)) = prefixed {
        return u64::from_str_radix(digits, radix).map_or(f64::NAN, |number| number as f64);
    }
    value.parse::<f64>().unwrap_or(f64::NAN)
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:24 (sha256:9066ea98917c13dfc3e2172f24e9617d31bb81e32d11fd5a2e19c658190d933d)
pub fn format_bitmap_font_fnt(font: &BitmapFont) -> String {
    let metrics = get_bitmap_font_metrics(font);
    let line_height = ((metrics.ascent + metrics.descent) + metrics.line_gap);
    let base = metrics.ascent;
    let primary_texture = (font.pages[0.0_f64 as usize].texture).clone();
    let primary_image = if (primary_texture
        .as_ref()
        .map(|value| (value.dimension).clone()))
        == Some("2d".to_owned())
    {
        (primary_texture.as_ref().unwrap().source).clone()
    } else {
        None
    };
    let scale_w = if (primary_image).is_some() {
        primary_image.as_ref().unwrap().width
    } else {
        0.0_f64
    };
    let scale_h = if (primary_image).is_some() {
        primary_image.as_ref().unwrap().height
    } else {
        0.0_f64
    };
    let page_count = (font.pages.len() as f64).max(1.0_f64);
    let mut lines: Vec<String> = vec![];
    lines.push(format!("info face=\"\" size={} bold=0 italic=0 charset=\"\" unicode=1 stretchH=100 smooth=1 aa=1 padding=0,0,0,0 spacing=0,0 outline=0", line_height));
    lines.push(format!("common lineHeight={} base={} scaleW={} scaleH={} pages={} packed=0 alphaChnl=1 redChnl=0 greenChnl=0 blueChnl=0", line_height, base, scale_w, scale_h, page_count));
    {
        let mut id = 0.0_f64;
        while (id < page_count) {
            lines.push(format!("page id={} file=\"\"", id));
            {
                id += 1.0;
                id
            };
        }
    }
    let codepoints = {
        let mut __flight_array = Vec::new();
        __flight_array.extend(
            (font
                .glyphs
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<_>>())
            .iter()
            .cloned(),
        );
        __flight_array
    };
    lines.push(format!("chars count={}", (codepoints.len() as f64)));
    for codepoint in (codepoints).iter().cloned() {
        let glyph = font
            .glyphs
            .iter()
            .find(|(entry_key, _)| entry_key == &(codepoint).clone())
            .map(|(_, value)| value.clone())
            .unwrap();
        lines.push(format!(
            "{}{}",
            format!(
                "char id={} x={} y={} width={} height={} ",
                (codepoint).clone(),
                glyph.x,
                glyph.y,
                glyph.width,
                glyph.height
            ),
            format!(
                "xoffset={} yoffset={} xadvance={} page={} chnl=15",
                glyph.bearing_x,
                (base - glyph.bearing_y),
                glyph.advance,
                glyph.page
            )
        ));
    }
    let kern_keys = {
        let mut __flight_array = Vec::new();
        __flight_array.extend(
            (font
                .kerning
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<_>>())
            .iter()
            .cloned(),
        );
        __flight_array
    };
    lines.push(format!("kernings count={}", (kern_keys.len() as f64)));
    for key in (kern_keys).iter().cloned() {
        let amount = font
            .kerning
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone())
            .unwrap();
        unpack_bitmap_font_kerning_key((key).clone(), &mut (*_KERNING_PAIR.lock().unwrap()));
        lines.push(format!(
            "kerning first={} second={} amount={}",
            (*_KERNING_PAIR.lock().unwrap()).left,
            (*_KERNING_PAIR.lock().unwrap()).right,
            amount
        ));
    }
    return format!(
        "{}{}",
        (lines)
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(("\n".to_owned()).as_str()),
        "\n"
    );
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:71 (sha256:0fb0bf39ba0e8dd8e474465330f02940ea1e00f04392b1c8e8e7dfc25f49d784)
pub fn parse_bitmap_font_fnt(
    text: String,
    options: Option<BitmapFontParseOptions>,
    mut diagnostics: Option<Vec<ImportDiagnostic>>,
) -> Option<BitmapFont> {
    let record = parse_bitmap_font_fnt_record((text).clone(), &mut (diagnostics));
    if (record).is_none() {
        return None;
    }
    return build_bitmap_font_from_record(
        (record.as_ref().unwrap()).clone(),
        Some(((options).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:84 (sha256:1396acdf14a6410d53d6ec9c15a57dd30c62660b5c17b7ce53702917d9d694c3)
fn parse_bitmap_font_fnt_record(
    text: String,
    diagnostics: &mut Option<Vec<ImportDiagnostic>>,
) -> Option<BitmapFontRecord> {
    let mut line_height: Option<f64> = None;
    let mut base: Option<f64> = None;
    let mut pages: Vec<BitmapFontPageRecord> = vec![];
    let mut chars: Vec<BitmapFontCharRecord> = vec![];
    let mut kernings: Vec<BitmapFontKerningRecord> = vec![];
    let mut dropped_pages = 0.0_f64;
    let mut dropped_chars = 0.0_f64;
    let mut dropped_kernings = 0.0_f64;
    for raw_line in ((regex::RegexBuilder::new("\\r\\n?|\\n")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .split(&(text))
    .map(|part| part.to_owned())
    .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        let line = (raw_line).trim().to_owned();
        if ((line).clone() == "") {
            continue;
        }
        let space_at = (regex::RegexBuilder::new("\\s")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .find(&((line).clone()))
        .map_or(-1.0_f64, |matched| matched.start() as f64);
        let tag = if (space_at < 0.0_f64) {
            (line).clone()
        } else {
            __flight_string_slice(&((line).clone()), 0.0_f64, Some(space_at))
        };
        let fields = parse_fnt_fields(__flight_string_slice(
            &((line).clone()),
            (tag.encode_utf16().count() as f64),
            None,
        ));
        if (tag == "common") {
            line_height = read_fnt_number(Some(
                (fields
                    .iter()
                    .find(|(entry_key, _)| entry_key == &"lineHeight".to_owned())
                    .map(|(_, value)| value.clone())
                    .expect("TypeScript Record key was absent"))
                .clone(),
            ));
            base = read_fnt_number(Some(
                (fields
                    .iter()
                    .find(|(entry_key, _)| entry_key == &"base".to_owned())
                    .map(|(_, value)| value.clone())
                    .expect("TypeScript Record key was absent"))
                .clone(),
            ));
        } else {
            if (tag == "page") {
                let id = read_fnt_number(Some(
                    (fields
                        .iter()
                        .find(|(entry_key, _)| entry_key == &"id".to_owned())
                        .map(|(_, value)| value.clone())
                        .expect("TypeScript Record key was absent"))
                    .clone(),
                ));
                if (id).is_none() {
                    {
                        dropped_pages += 1.0;
                        dropped_pages
                    };
                } else {
                    pages.push(BitmapFontPageRecord {
                        __flight_identity: std::sync::Arc::new(()),
                        file: fields
                            .iter()
                            .find(|(entry_key, _)| entry_key == &"file".to_owned())
                            .map(|(_, value)| value.clone())
                            .expect("TypeScript Record key was absent"),
                        id: *(id.as_ref().unwrap()),
                    });
                }
            } else {
                if (tag == "char") {
                    let char = read_fnt_char(&fields);
                    if ((char).clone()).is_none() {
                        {
                            dropped_chars += 1.0;
                            dropped_chars
                        };
                    } else {
                        chars.push(((char.as_ref().unwrap()).clone()).clone());
                    }
                } else {
                    if (tag == "kerning") {
                        let kerning = read_fnt_kerning(&fields);
                        if ((kerning).clone()).is_none() {
                            {
                                dropped_kernings += 1.0;
                                dropped_kernings
                            };
                        } else {
                            kernings.push(((kerning.as_ref().unwrap()).clone()).clone());
                        }
                    }
                }
            }
        }
    }
    report_dropped_bitmap_font_records(
        diagnostics,
        "parseBitmapFontFntRecord".to_owned(),
        dropped_pages,
        dropped_chars,
        dropped_kernings,
    );
    if (((line_height).is_none()) || ((base).is_none())) || ((chars.len() as f64) == 0.0_f64) {
        return None;
    }
    return Some(BitmapFontRecord {
        __flight_identity: std::sync::Arc::new(()),
        base: *(base.as_mut().unwrap()),
        chars: (chars).clone(),
        encoding: "raster".to_owned(),
        kernings: (kernings).clone(),
        line_height: *(line_height.as_mut().unwrap()),
        pages: (pages).clone(),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:128 (sha256:220544cc241ccf8ae5b74176f788cd6e3b1b2fd0cfe8fa87b012c9a310e21bc1)
#[derive(Clone, Default)]
struct ParseFntFieldsRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ParseFntFieldsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn parse_fnt_fields(rest: String) -> Vec<(String, String)> {
    let mut fields: Vec<(String, String)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    let re = regex::RegexBuilder::new("([A-Za-z_]\\w*)\\s*=\\s*(?:\"([^\"]*)\"|(\\S+))")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
    let mut match_: Option<Vec<Option<String>>>;
    while ({
        match_ = {
            let __flight_regex = &(re);
            __flight_regex.captures(&((rest).clone())).map(|captures| {
                (0..captures.len())
                    .map(|index| {
                        captures
                            .get(index)
                            .map(|matched| matched.as_str().to_owned())
                    })
                    .collect::<Vec<_>>()
            })
        };
        match_.clone()
    })
    .is_some()
    {
        {
            let __flight_key = match_.as_mut().unwrap()[1.0_f64 as usize].clone().unwrap();
            let __flight_value = if (match_.as_mut().unwrap()[2.0_f64 as usize].clone()).is_some() {
                match_.as_mut().unwrap()[2.0_f64 as usize].clone().unwrap()
            } else {
                (match_.as_mut().unwrap()[3.0_f64 as usize].clone())
                    .clone()
                    .unwrap_or("".to_owned())
            };
            if let Some((_, value)) = fields.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                fields.push((__flight_key, __flight_value));
            }
        };
    }
    return fields;
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:138 (sha256:e9ec04962c1833deb51d449cae0e2ac62efddcc46ec756b1c89c5d2c713d5de0)
fn read_fnt_char(fields: &Vec<(String, String)>) -> Option<BitmapFontCharRecord> {
    let id = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"id".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let x = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"x".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let y = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"y".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let width = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"width".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let height = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"height".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let xoffset = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"xoffset".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let yoffset = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"yoffset".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let xadvance = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"xadvance".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    if ((((((((id).is_none()) || ((x).is_none())) || ((y).is_none())) || ((width).is_none()))
        || ((height).is_none()))
        || ((xoffset).is_none()))
        || ((yoffset).is_none()))
        || ((xadvance).is_none())
    {
        return None;
    }
    return Some(BitmapFontCharRecord {
        __flight_identity: std::sync::Arc::new(()),
        height: *(height.as_ref().unwrap()),
        id: *(id.as_ref().unwrap()),
        page: (read_fnt_number(Some(
            (fields
                .iter()
                .find(|(entry_key, _)| entry_key == &"page".to_owned())
                .map(|(_, value)| value.clone())
                .expect("TypeScript Record key was absent"))
            .clone(),
        )))
        .clone()
        .unwrap_or(0.0_f64),
        width: *(width.as_ref().unwrap()),
        x: *(x.as_ref().unwrap()),
        xadvance: *(xadvance.as_ref().unwrap()),
        xoffset: *(xoffset.as_ref().unwrap()),
        y: *(y.as_ref().unwrap()),
        yoffset: *(yoffset.as_ref().unwrap()),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:162 (sha256:163dd9f2cc4b00d747ce9e81b378c96d30f120670dc9661243b8de447c50cb8a)
fn read_fnt_kerning(fields: &Vec<(String, String)>) -> Option<BitmapFontKerningRecord> {
    let first = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"first".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let second = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"second".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let amount = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(entry_key, _)| entry_key == &"amount".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    if (((first).is_none()) || ((second).is_none())) || ((amount).is_none()) {
        return None;
    }
    return Some(BitmapFontKerningRecord {
        __flight_identity: std::sync::Arc::new(()),
        amount: *(amount.as_ref().unwrap()),
        first: *(first.as_ref().unwrap()),
        second: *(second.as_ref().unwrap()),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:172 (sha256:061d1555bcce0612a142320f51716accd695dcd24345b46d2f9dc0143fad2697)
fn read_fnt_number(value: Option<String>) -> Option<f64> {
    if ((value).is_none()) || ((value.as_ref().unwrap()).trim().to_owned() == "") {
        return None;
    }
    let parsed = __flight_number_from_string(&((value.as_ref().unwrap()).clone()));
    return if (parsed).is_finite() {
        Some(parsed)
    } else {
        None
    };
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:180 (sha256:cac4787060332e3ffccce6a1df878b38cee7320de3329eba61bb6c8a79036bb3)
static _KERNING_PAIR: std::sync::LazyLock<std::sync::Mutex<BitmapFontKerningPair>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(BitmapFontKerningPair {
            __flight_identity: std::sync::Arc::new(()),
            left: 0.0_f64,
            right: 0.0_f64,
        })
    });
