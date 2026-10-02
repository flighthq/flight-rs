// @generated from upstream/packages/textbidi/src/enableTextBidiGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_text_bidi_guard;
use flighthq_log::log_once;
use flighthq_types::{BidiClassKernel, LogData, LogDataProvider, LogLevel};

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

// Source: upstream/packages/textbidi/src/enableTextBidiGuards.ts:7 (sha256:c510a5e58ee524396053461a605a5bd83ced0a168b5dc8ef8ed0afcbc0a04477)
pub fn disable_text_bidi_guards() -> () {
    set_text_bidi_guard(&(None));
}

// Source: upstream/packages/textbidi/src/enableTextBidiGuards.ts:12 (sha256:d29452d469ed5ce2167015dbb742a42c745cb2153c7e047379db7f2fc5297330)
pub fn enable_text_bidi_guards() -> () {
    set_text_bidi_guard(&(warn_on_compact_table_miss));
}

// Source: upstream/packages/textbidi/src/enableTextBidiGuards.ts:16 (sha256:91a7718d27971ca9d1f192e8519e594160e9fa1be02b51564899baacfcb8bad3)
#[derive(Clone, Default)]
struct WarnOnCompactTableMissRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnCompactTableMissRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_compact_table_miss(codepoint: f64) -> () {
    log_once(
        "textbidi:compact-table-miss".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("resolveBidiLevels: U+{} is outside the compact bidi-class table and defaulted to L. Pass a full-coverage BidiClassKernel to resolveBidiLevels or getBidiRuns.", __flight_pad_start((__flight_number_to_string(codepoint, 16.0_f64)).to_uppercase(), 4.0_f64, "0".to_owned())); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("textbidi".to_owned()).clone()),
    );
}
