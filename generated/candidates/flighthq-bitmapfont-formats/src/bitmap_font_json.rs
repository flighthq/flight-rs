// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{build_bitmap_font_from_record, report_dropped_bitmap_font_records};
use flighthq_types::{
    BitmapFont, BitmapFontCharRecord, BitmapFontEncoding, BitmapFontKerningRecord,
    BitmapFontPageRecord, BitmapFontParseOptions, BitmapFontRecord, ImportDiagnostic,
};

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:21 (sha256:de0efced0fae7b70f2a866fc8fa02acc399f136b7df4221802cf69fa5f557fb9)
pub fn parse_bitmap_font_json(
    text: String,
    options: Option<BitmapFontParseOptions>,
    mut diagnostics: Option<Vec<ImportDiagnostic>>,
) -> Option<BitmapFont> {
    let record = parse_bitmap_font_json_record((text).clone(), &mut (diagnostics));
    if (record).is_none() {
        return None;
    }
    return build_bitmap_font_from_record(
        (record.as_ref().unwrap()).clone(),
        Some(((options).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:33 (sha256:26fd4c0b26ed48669384b8f0f84603034c248d4ab8b5880e6cbe8ef0f2fb6c9f)
fn parse_bitmap_font_json_record(
    text: String,
    diagnostics: &mut Option<Vec<ImportDiagnostic>>,
) -> Option<BitmapFontRecord> {
    let mut root: crate::OpaqueHostValue;
    let __flight_try_return: Option<Option<BitmapFontRecord>> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<Option<BitmapFontRecord>> {
            {
                root = (json.parse)(text);
            }
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<Option<BitmapFontRecord>> {
            {
                return Some(None);
            }
            None
        })(),
    };
    if let Some(__flight_return) = __flight_try_return {
        return __flight_return;
    }
    if (!is_object((root).clone())) {
        return None;
    }
    let common = crate::host_value::<crate::OpaqueHostValue>("host.common");
    if (!is_object((common).clone())) {
        return None;
    }
    let line_height = read_json_number(crate::host_value::<crate::OpaqueHostValue>(
        "host.lineHeight",
    ));
    let base = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.base"));
    if ((line_height).is_none()) || ((base).is_none()) {
        return None;
    }
    let mut dropped_pages = 0.0_f64;
    let mut dropped_chars = 0.0_f64;
    let mut dropped_kernings = 0.0_f64;
    let raw_chars = crate::host_value::<crate::OpaqueHostValue>("host.chars");
    if (!false) {
        return None;
    }
    let mut chars: Vec<BitmapFontCharRecord> = vec![];
    for raw in (raw_chars).iter().cloned() {
        let char = read_json_char(raw);
        if ((char).clone()).is_none() {
            {
                dropped_chars += 1.0;
                dropped_chars
            };
        } else {
            chars.push(((char.as_ref().unwrap()).clone()).clone());
        }
    }
    if ((chars.len() as f64) == 0.0_f64) {
        return None;
    }
    let mut pages: Vec<BitmapFontPageRecord> = vec![];
    if false {
        {
            let mut id = 0.0_f64;
            while (id < crate::host_value::<f64>("host.length")) {
                let file = crate::host_value::<crate::OpaqueHostValue>("host.index");
                pages.push(BitmapFontPageRecord {
                    __flight_identity: std::sync::Arc::new(()),
                    file: if (match &((file).clone()) {
                        crate::OpaqueHostValue::Undefined => "undefined",
                        crate::OpaqueHostValue::Null
                        | crate::OpaqueHostValue::Array(_)
                        | crate::OpaqueHostValue::Record(_)
                        | crate::OpaqueHostValue::Object => "object",
                        crate::OpaqueHostValue::Bool(_) => "boolean",
                        crate::OpaqueHostValue::Number(_) => "number",
                        crate::OpaqueHostValue::String(_) => "string",
                        crate::OpaqueHostValue::Function => "function",
                        crate::OpaqueHostValue::Symbol => "symbol",
                    } == "string")
                    {
                        (file).clone()
                    } else {
                        "".to_owned()
                    },
                    id: id,
                });
                {
                    id += 1.0;
                    id
                };
            }
        }
    }
    let mut kernings: Vec<BitmapFontKerningRecord> = vec![];
    if false {
        for raw in (crate::host_value::<crate::OpaqueHostValue>("host.kernings"))
            .iter()
            .cloned()
        {
            let kerning = read_json_kerning(raw);
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
    report_dropped_bitmap_font_records(
        diagnostics,
        "parseBitmapFontJsonRecord".to_owned(),
        dropped_pages,
        dropped_chars,
        dropped_kernings,
    );
    return Some(BitmapFontRecord {
        __flight_identity: std::sync::Arc::new(()),
        base: *(base.as_ref().unwrap()),
        chars: (chars).clone(),
        encoding: read_json_encoding(crate::host_value::<crate::OpaqueHostValue>(
            "host.distanceField",
        )),
        kernings: (kernings).clone(),
        line_height: *(line_height.as_ref().unwrap()),
        pages: (pages).clone(),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:84 (sha256:66636d3c94d03704e09c7fa198f1c90a1d05a0826fbb0e13f35270cd137ac31a)
fn is_object(value: crate::OpaqueHostValue) -> bool {
    return ((match &(value) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null
        | crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
        crate::OpaqueHostValue::Function => "function",
        crate::OpaqueHostValue::Symbol => "symbol",
    } == "object")
        && ((value).is_some()))
        && (!false);
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:88 (sha256:2c43614aab520677cd8862d1077ddd6189c2090a04062b507efbb1b3195de053)
fn read_json_char(raw: crate::OpaqueHostValue) -> Option<BitmapFontCharRecord> {
    if (!is_object((raw).clone())) {
        return None;
    }
    let id = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.id"));
    let x = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.x"));
    let y = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.y"));
    let width = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.width"));
    let height = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.height"));
    let xoffset = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.xoffset"));
    let yoffset = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.yoffset"));
    let xadvance = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.xadvance"));
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
        page: (read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.page")))
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

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:115 (sha256:7f649aa5d20242c9a7859dbd556b4dedf2366f9e3bbdf94c83514953040eaf8a)
fn read_json_encoding(distance_field: crate::OpaqueHostValue) -> BitmapFontEncoding {
    if is_object((distance_field).clone()) {
        let field_type = crate::host_value::<crate::OpaqueHostValue>("host.fieldType");
        if (field_type == "msdf") || (field_type == "sdf") {
            return field_type;
        }
    }
    return "raster".to_owned();
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:123 (sha256:22bc29f4d4a1fbda3c55d75a345267908a61b3a093cec3b5426d6bb44428c6b9)
fn read_json_kerning(raw: crate::OpaqueHostValue) -> Option<BitmapFontKerningRecord> {
    if (!is_object((raw).clone())) {
        return None;
    }
    let first = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.first"));
    let second = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.second"));
    let amount = read_json_number(crate::host_value::<crate::OpaqueHostValue>("host.amount"));
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

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts:132 (sha256:3a349797c1268183a7048ca6ab1ee6f48958c971159db4a2cbacf46a838877d5)
fn read_json_number(value: crate::OpaqueHostValue) -> Option<f64> {
    return if (match &(value) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null
        | crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
        crate::OpaqueHostValue::Function => "function",
        crate::OpaqueHostValue::Symbol => "symbol",
    } == "number")
        && ((value).is_finite())
    {
        Some(value)
    } else {
        None
    };
}
