// @generated from upstream/packages/textbidi/src/textBidiGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::TextBidiGuard;

// Source: upstream/packages/textbidi/src/textBidiGuards.ts:3 (sha256:f32de455c3d37a40e9413fd2c604aa41ece022667aa18f8e73894080d9f9d28a)
pub fn report_text_bidi_compact_table_miss(codepoint: f64) -> () {
    {
        let __flight_callback = (*_GUARD.lock().unwrap()).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()(codepoint))
    };
}

// Source: upstream/packages/textbidi/src/textBidiGuards.ts:7 (sha256:0da965347eeb93da88b886bcddcdccc49d06156381f2148fc174343be6631eae)
pub fn set_text_bidi_guard(guard: &Option<TextBidiGuard>) -> () {
    (*_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/textbidi/src/textBidiGuards.ts:11 (sha256:8c84b9cc0c6db79a1176bf1625cfb5905c70442b1735d9adc30b38acb0564a63)
static _GUARD: std::sync::LazyLock<std::sync::Mutex<Option<TextBidiGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
