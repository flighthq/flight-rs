// @generated from upstream/packages/bitmapfont-formats/src/bitmapFontXml.ts; do not edit.
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
use flighthq_types::{BitmapFont, BitmapFontParseOptions};
use flighthq_xml::{
    XmlElement, get_xml_element_attribute, get_xml_element_attribute_number,
    get_xml_element_child_by_name, get_xml_element_children_by_name, parse_xml_document,
};

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontXml.ts:24 (sha256:9a68c1623f2e120a120699af01da091963a308ad494eb6fd8d5f7213d04c25f3)
pub fn parse_bitmap_font_xml(
    text: String,
    options: Option<BitmapFontParseOptions>,
) -> Option<BitmapFont> {
    let record = parse_bitmap_font_xml_record((text).clone());
    if (record).is_none() {
        return None;
    }
    return build_bitmap_font_from_record(
        (record.as_ref().unwrap()).clone(),
        Some(((options).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontXml.ts:32 (sha256:acc65f2c3182bcac1f536aacb47b497f1a12f7e3b5cbbc3259dfb8310f7bdf34)
fn parse_bitmap_font_xml_record(text: String) -> Option<BitmapFontRecord> {
    let root = parse_xml_document((text).clone());
    if ((root).is_none()) || ((root.as_ref().unwrap().name).clone() != "font") {
        return None;
    }
    let common = get_xml_element_child_by_name(root.as_ref().unwrap(), "common".to_owned());
    if (common).is_none() {
        return None;
    }
    let line_height =
        get_xml_element_attribute_number(&common.as_ref().unwrap(), "lineHeight".to_owned());
    let base = get_xml_element_attribute_number(&common.as_ref().unwrap(), "base".to_owned());
    if ((line_height).is_none()) || ((base).is_none()) {
        return None;
    }
    let mut pages: Vec<BitmapFontPageRecord> = vec![];
    let pages_element = get_xml_element_child_by_name(root.as_ref().unwrap(), "pages".to_owned());
    if (pages_element).is_some() {
        for page_element in
            (get_xml_element_children_by_name(&pages_element.as_ref().unwrap(), "page".to_owned()))
                .iter()
                .cloned()
        {
            let id = get_xml_element_attribute_number(&page_element, "id".to_owned());
            if (id).is_some() {
                pages.push(BitmapFontPageRecord {
                    __flight_identity: std::sync::Arc::new(()),
                    file: (get_xml_element_attribute(&page_element, "file".to_owned()))
                        .unwrap_or("".to_owned()),
                    id: *(id.as_ref().unwrap()),
                });
            }
        }
    }
    let mut chars: Vec<BitmapFontCharRecord> = vec![];
    let chars_element = get_xml_element_child_by_name(root.as_ref().unwrap(), "chars".to_owned());
    if (chars_element).is_some() {
        for char_element in
            (get_xml_element_children_by_name(&chars_element.as_ref().unwrap(), "char".to_owned()))
                .iter()
                .cloned()
        {
            let char = read_xml_char(&char_element);
            if (char).is_some() {
                chars.push(((char.as_ref().unwrap()).clone()).clone());
            }
        }
    }
    if ((chars.len() as f64) == 0.0_f64) {
        return None;
    }
    let mut kernings: Vec<BitmapFontKerningRecord> = vec![];
    let kernings_element =
        get_xml_element_child_by_name(root.as_ref().unwrap(), "kernings".to_owned());
    if (kernings_element).is_some() {
        for kerning_element in (get_xml_element_children_by_name(
            &kernings_element.as_ref().unwrap(),
            "kerning".to_owned(),
        ))
        .iter()
        .cloned()
        {
            let kerning = read_xml_kerning(&kerning_element);
            if (kerning).is_some() {
                kernings.push(((kerning.as_ref().unwrap()).clone()).clone());
            }
        }
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

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontXml.ts:73 (sha256:04c5fb153376910e243deab138af00e02091efd913c14ffe7d834fe400181fa5)
fn read_xml_char(element: &XmlElement) -> Option<BitmapFontCharRecord> {
    let id = get_xml_element_attribute_number(element, "id".to_owned());
    let x = get_xml_element_attribute_number(element, "x".to_owned());
    let y = get_xml_element_attribute_number(element, "y".to_owned());
    let width = get_xml_element_attribute_number(element, "width".to_owned());
    let height = get_xml_element_attribute_number(element, "height".to_owned());
    let xoffset = get_xml_element_attribute_number(element, "xoffset".to_owned());
    let yoffset = get_xml_element_attribute_number(element, "yoffset".to_owned());
    let xadvance = get_xml_element_attribute_number(element, "xadvance".to_owned());
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
        page: (get_xml_element_attribute_number(element, "page".to_owned())).unwrap_or(0.0_f64),
        width: (width).clone().unwrap(),
        x: (x).clone().unwrap(),
        xadvance: (xadvance).clone().unwrap(),
        xoffset: (xoffset).clone().unwrap(),
        y: (y).clone().unwrap(),
        yoffset: (yoffset).clone().unwrap(),
    });
}

// Source: upstream/packages/bitmapfont-formats/src/bitmapFontXml.ts:107 (sha256:ace4fc1703698b38e6f2359ba6f0f3af2adeac0b5f8c9f0602490e626783e93e)
fn read_xml_kerning(element: &XmlElement) -> Option<BitmapFontKerningRecord> {
    let first = get_xml_element_attribute_number(element, "first".to_owned());
    let second = get_xml_element_attribute_number(element, "second".to_owned());
    let amount = get_xml_element_attribute_number(element, "amount".to_owned());
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
