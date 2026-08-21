// @generated from upstream/packages/bitmapfont/src/enableBitmapFontGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_bitmap_font_guard;
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

// Source: upstream/packages/bitmapfont/src/enableBitmapFontGuards.ts:7 (sha256:519e7d59e8cb3b24bae341c3789df54b106bdeacfdf24ccbe13d898da0df7ca7)
pub fn disable_bitmap_font_guards() -> () {
    set_bitmap_font_guard(&(None));
}

// Source: upstream/packages/bitmapfont/src/enableBitmapFontGuards.ts:25 (sha256:abc4fd323f6854d62ef6acc452d73c083a0ca6207064ca275dd0728627488508)
pub fn enable_bitmap_font_guards() -> () {
    set_bitmap_font_guard(&(warn_on_bitmap_font_repair));
}

// Source: upstream/packages/bitmapfont/src/enableBitmapFontGuards.ts:33 (sha256:db76c05dfbf214f24edcb5201006aac64f577213ca65a8c995dcc31937b696f6)
fn warn_on_bitmap_font_repair(reason: String, codepoint: f64, page: f64) -> () {
    if (reason != "page-out-of-range") {
        return;
    }
    let printable = format!(
        "U+{}",
        __flight_pad_start(
            (__flight_number_to_string(codepoint, 16.0_f64)).to_uppercase(),
            4.0_f64,
            "0".to_owned()
        )
    );
    log_once(
        "bitmapfont:page-out-of-range".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}{}", format!("{}{}", format!("createBitmapFont: {} names page {}, which this font does not have, so it was ", (printable).clone(), page), "placed on page 0 and will sample whatever occupies those coordinates there. The font data is "), "wrong, not the atlas — check the page index the exporter wrote."); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("bitmapfont".to_owned()).clone()),
    );
}
