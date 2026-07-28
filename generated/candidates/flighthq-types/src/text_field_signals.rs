// @generated from upstream/packages/types/src/TextFieldSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/TextFieldSignals.ts:4 (sha256:0bb251d437abeca372e55ededd638d514ae52b590f9733a5547ced24b2f4f562)
#[derive(Clone)]
pub struct TextFieldChangeEvent {
    pub previous_text: String,
    pub text: String,
}

// Source: upstream/packages/types/src/TextFieldSignals.ts:10 (sha256:d95669ea7d7acf941d87103a00653ddf36137335866b5ef1ca81504d696b9522)
#[derive(Clone)]
pub struct TextFieldLinkEvent {
    pub url: String,
    pub x: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/TextFieldSignals.ts:17 (sha256:287c64c789388d20f99a69a6a87c0b2e6a9df7b301f039fe09b25118491921e2)
#[derive(Clone)]
pub struct TextFieldScrollEvent {
    pub previous_scroll_h: f64,
    pub previous_scroll_v: f64,
    pub scroll_h: f64,
    pub scroll_v: f64,
}

// Source: upstream/packages/types/src/TextFieldSignals.ts:26 (sha256:d56b6dc910194cc6c1a94a7bf2097bfcecfbb2351905dc92250f532e3473f647)
#[derive(Clone)]
pub struct TextFieldSignals {
    pub on_text_field_change: Signal,
    pub on_text_field_link: Signal,
    pub on_text_field_scroll: Signal,
}
