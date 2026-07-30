// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BitmapFontCharRecord, BitmapFontKerningRecord, BitmapFontPageRecord, BitmapFontRecord,
    build_bitmap_font_from_record,
};
use flighthq_bitmapfont::get_bitmap_font_metrics;
use flighthq_types::{BitmapFont, BitmapFontParseOptions};

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

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:20 (sha256:c243209a8b6c2bcd530f5103f375256a88d1b51b9b9b9864b74fbad3c64e3f0e)
pub fn format_bitmap_font_fnt(font: &BitmapFont) -> String {
    let metrics = get_bitmap_font_metrics(font);
    let line_height = ((metrics.ascent + metrics.descent) + metrics.line_gap);
    let base = metrics.ascent;
    let primary_image = (font.pages[0.0_f64 as usize].image).clone();
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
            .find(|(key, _)| key == &codepoint)
            .map(|(_, value)| value.clone());
        lines.push(
            (format!(
                "char id={} x={} y={} width={} height={} ",
                codepoint, glyph.x, glyph.y, glyph.width, glyph.height
            ) + format!(
                "xoffset={} yoffset={} xadvance={} page={} chnl=15",
                glyph.bearing_x,
                (base - glyph.bearing_y),
                glyph.advance,
                glyph.page
            )),
        );
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
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value.clone());
        let first = (__flight_js_to_u32(key) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64;
        let second = (__flight_js_to_i32(key) & __flight_js_to_i32(65535.0_f64)) as f64;
        lines.push(format!(
            "kerning first={} second={} amount={}",
            first, second, amount
        ));
    }
    return ((lines.join)("\n") + "\n");
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:67 (sha256:c42d4ad04ba342517df76eb8c8859fdc40889dc37715254b4a79eff0307298ea)
pub fn parse_bitmap_font_fnt(
    text: String,
    options: Option<BitmapFontParseOptions>,
) -> Option<BitmapFont> {
    let record = parse_bitmap_font_fnt_record((text).clone());
    if (record).is_none() {
        return None;
    }
    return build_bitmap_font_from_record(
        (record.as_ref().unwrap()).clone(),
        Some(((options).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:76 (sha256:e23c8a29db1a156d98532613a4ef71de7976db5eab0a5670f19d3c07e321f640)
fn parse_bitmap_font_fnt_record(text: String) -> Option<BitmapFontRecord> {
    let mut line_height: Option<f64> = None;
    let mut base: Option<f64> = None;
    let mut pages: Vec<BitmapFontPageRecord> = vec![];
    let mut chars: Vec<BitmapFontCharRecord> = vec![];
    let mut kernings: Vec<BitmapFontKerningRecord> = vec![];
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
        if (line == "") {
            continue;
        }
        let space_at = (regex::RegexBuilder::new("\\s")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .find(&(line))
        .map_or(-1.0_f64, |matched| matched.start() as f64);
        let tag = if (space_at < 0.0_f64) {
            (line).clone()
        } else {
            String::from_utf16_lossy(
                &(line)
                    .encode_utf16()
                    .skip((0.0_f64) as usize)
                    .take(((space_at) as usize).saturating_sub((0.0_f64) as usize))
                    .collect::<Vec<u16>>(),
            )
        };
        let fields = parse_fnt_fields(String::from_utf16_lossy(
            &(line)
                .encode_utf16()
                .skip((tag.encode_utf16().count() as f64) as usize)
                .collect::<Vec<u16>>(),
        ));
        if (tag == "common") {
            line_height = read_fnt_number(Some(
                (fields
                    .iter()
                    .find(|(key, _)| key == &"lineHeight".to_owned())
                    .map(|(_, value)| value.clone())
                    .expect("TypeScript Record key was absent"))
                .clone(),
            ));
            base = read_fnt_number(Some(
                (fields
                    .iter()
                    .find(|(key, _)| key == &"base".to_owned())
                    .map(|(_, value)| value.clone())
                    .expect("TypeScript Record key was absent"))
                .clone(),
            ));
        } else {
            if (tag == "page") {
                let id = read_fnt_number(Some(
                    (fields
                        .iter()
                        .find(|(key, _)| key == &"id".to_owned())
                        .map(|(_, value)| value.clone())
                        .expect("TypeScript Record key was absent"))
                    .clone(),
                ));
                if (id).is_some() {
                    pages.push(BitmapFontPageRecord {
                        __flight_identity: std::sync::Arc::new(()),
                        file: fields
                            .iter()
                            .find(|(key, _)| key == &"file".to_owned())
                            .map(|(_, value)| value.clone())
                            .expect("TypeScript Record key was absent"),
                        id: *(id.as_ref().unwrap()),
                    });
                }
            } else {
                if (tag == "char") {
                    let char = read_fnt_char(&fields);
                    if (char).is_some() {
                        chars.push(((char.as_ref().unwrap()).clone()).clone());
                    }
                } else {
                    if (tag == "kerning") {
                        let kerning = read_fnt_kerning(&fields);
                        if (kerning).is_some() {
                            kernings.push(((kerning.as_ref().unwrap()).clone()).clone());
                        }
                    }
                }
            }
        }
    }
    if (((line_height).is_none()) || ((base).is_none())) || ((chars.len() as f64) == 0.0_f64) {
        return None;
    }
    return Some(BitmapFontRecord {
        __flight_identity: std::sync::Arc::new(()),
        base: (base).clone().unwrap(),
        chars: (chars).clone(),
        encoding: "raster".to_owned(),
        kernings: (kernings).clone(),
        line_height: (line_height).clone().unwrap(),
        pages: (pages).clone(),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:111 (sha256:220544cc241ccf8ae5b74176f788cd6e3b1b2fd0cfe8fa87b012c9a310e21bc1)
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
    let mut match_: Option<crate::OpaqueHostValue>;
    while ({
        match_ = {
            let __flight_regex = re;
            __flight_regex.captures(&((rest).clone())).map(|captures| {
                (0..captures.len())
                    .map(|index| {
                        captures
                            .get(index)
                            .map_or("", |matched| matched.as_str())
                            .to_owned()
                    })
                    .collect::<Vec<_>>()
            })
        };
        match_
    })
    .is_some()
    {
        fields
            .iter()
            .find(|(key, _)| key == &crate::host_value::<String>("host.index"))
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            if (crate::host_value::<crate::OpaqueHostValue>("host.index")).is_some() {
                crate::host_value::<String>("host.index")
            } else {
                crate::host_value::<crate::OpaqueHostValue>("host.index")
            };
    }
    return fields;
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:121 (sha256:e9ec04962c1833deb51d449cae0e2ac62efddcc46ec756b1c89c5d2c713d5de0)
fn read_fnt_char(fields: &Vec<(String, String)>) -> Option<BitmapFontCharRecord> {
    let id = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"id".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let x = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"x".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let y = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"y".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let width = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"width".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let height = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"height".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let xoffset = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"xoffset".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let yoffset = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"yoffset".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let xadvance = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"xadvance".to_owned())
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
        height: (height).clone().unwrap(),
        id: (id).clone().unwrap(),
        page: (read_fnt_number(Some(
            (fields
                .iter()
                .find(|(key, _)| key == &"page".to_owned())
                .map(|(_, value)| value.clone())
                .expect("TypeScript Record key was absent"))
            .clone(),
        )))
        .unwrap_or(0.0_f64),
        width: (width).clone().unwrap(),
        x: (x).clone().unwrap(),
        xadvance: (xadvance).clone().unwrap(),
        xoffset: (xoffset).clone().unwrap(),
        y: (y).clone().unwrap(),
        yoffset: (yoffset).clone().unwrap(),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:145 (sha256:163dd9f2cc4b00d747ce9e81b378c96d30f120670dc9661243b8de447c50cb8a)
fn read_fnt_kerning(fields: &Vec<(String, String)>) -> Option<BitmapFontKerningRecord> {
    let first = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"first".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let second = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"second".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    let amount = read_fnt_number(Some(
        (fields
            .iter()
            .find(|(key, _)| key == &"amount".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
        .clone(),
    ));
    if (((first).is_none()) || ((second).is_none())) || ((amount).is_none()) {
        return None;
    }
    return Some(BitmapFontKerningRecord {
        __flight_identity: std::sync::Arc::new(()),
        amount: (amount).clone().unwrap(),
        first: (first).clone().unwrap(),
        second: (second).clone().unwrap(),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts:155 (sha256:061d1555bcce0612a142320f51716accd695dcd24345b46d2f9dc0143fad2697)
fn read_fnt_number(value: Option<String>) -> Option<f64> {
    if ((value).is_none()) || ((value).trim().to_owned() == "") {
        return None;
    }
    let parsed = number(value);
    return if (parsed).is_finite() {
        Some(parsed)
    } else {
        None
    };
}
