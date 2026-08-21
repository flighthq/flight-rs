// @generated from upstream/packages/glyphatlas/src/enableGlyphAtlasGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_glyph_atlas_entry_guard;
use flighthq_log::log_once;
use flighthq_types::{LogData, LogDataProvider, LogLevel};

#[inline]

fn __flight_number_to_string(value: f64, radix: f64) -> String {
    let radix = radix.trunc().clamp(2.0_f64, 36.0_f64) as u32;
    let mut value = value.trunc().rem_euclid(4294967296.0_f64) as u32;
    if value == 0 {
        return "0".to_owned();
    }
    let mut digits = Vec::new();
    while value > 0 {
        let digit = value % radix;
        digits.push(char::from_digit(digit, radix).unwrap());
        value /= radix;
    }
    digits.iter().rev().collect()
}

#[inline]

fn __flight_pad_start(value: String, width: f64, pad: String) -> String {
    let length = value.chars().count();
    let width = width.max(0.0_f64).trunc() as usize;
    if length >= width || pad.is_empty() {
        return value;
    }
    let prefix: String = pad.chars().cycle().take(width - length).collect();
    prefix + &value
}

// Source: upstream/packages/glyphatlas/src/enableGlyphAtlasGuards.ts:7 (sha256:f87c9f7f3f0d90fe1256dc636e3237b36ba9606aa6b2a263e25df4ec031b512d)
pub fn disable_glyph_atlas_guards() -> () {
    set_glyph_atlas_entry_guard(&(None));
}

// Source: upstream/packages/glyphatlas/src/enableGlyphAtlasGuards.ts:29 (sha256:9c254f7fdef8502ff2fdc6dd4501fcca40698a9dc28c0229bfdb04cd5cf2a00f)
pub fn enable_glyph_atlas_guards() -> () {
    set_glyph_atlas_entry_guard(&(warn_on_glyph_atlas_entry_blocked));
}

// Source: upstream/packages/glyphatlas/src/enableGlyphAtlasGuards.ts:33 (sha256:d3dc8804fde4a8f9201fc5d2b7cf0b56300abc0748a5c2a5b75ac81a961f3b00)
fn warn_on_glyph_atlas_entry_blocked(reason: String, codepoint: f64) -> () {
    let printable = format!(
        "U+{}",
        __flight_pad_start(
            (__flight_number_to_string(codepoint, 16.0_f64)).to_uppercase(),
            4.0_f64,
            "0".to_owned()
        )
    );
    if (reason == "rasterizer-returned-null") {
        log_once(
            "glyphatlas:rasterizer-returned-null".to_owned(),
            LogLevel::Warn,
            &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                String,
                Vec<(String, crate::FlightValue)>,
            >::B({
                let mut __flight_record = Vec::new();
                __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}{}", format!("getGlyphAtlasEntry: the rasterizer produced nothing for {}, so it will not render. ", (printable).clone()), "The host may have no canvas, the font may not cover this codepoint, or a custom backend declined."); crate::FlightValue::String((&__flight_portable_source).clone()) }));
                __flight_record
            }))),
            Some(("glyphatlas".to_owned()).clone()),
        );
        return;
    }
    if (reason == "glyph-larger-than-atlas") {
        log_once(
            "glyphatlas:glyph-larger-than-atlas".to_owned(),
            LogLevel::Warn,
            &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                String,
                Vec<(String, crate::FlightValue)>,
            >::B({
                let mut __flight_record = Vec::new();
                __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}{}", format!("{}{}", format!("getGlyphAtlasEntry: {} rasterizes larger than the atlas's usable area, so it can never ", (printable).clone()), "be placed however much is evicted. Enlarge the atlas or reduce the font size; "), "explainGlyphAtlasEntry reports the measured sizes."); crate::FlightValue::String((&__flight_portable_source).clone()) }));
                __flight_record
            }))),
            Some(("glyphatlas".to_owned()).clone()),
        );
        return;
    }
    log_once(
        "glyphatlas:repack-dropped".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}{}", format!("{}{}", format!("getGlyphAtlasEntry: a repack could not replace {} and dropped it. Occasional drops are ", (printable).clone()), "normal under pressure; repeated ones mean the atlas is thrashing and glyphs are being "), "re-rasterized on every use. Consider a larger atlas or a byte/area budget."); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("glyphatlas".to_owned()).clone()),
    );
}
