// @generated from upstream/packages/textlayout/src/textFormatRange.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{TextFormat, TextFormatRange};

// Source: upstream/packages/textlayout/src/textFormatRange.ts:3 (sha256:4e0d8eb466dd5ff005788aee9b0f3a2d3fc89aacd908d053d7550ea05c5d3890)
pub fn create_text_format_range(format: &TextFormat, start: f64, end: f64) -> TextFormatRange {
    return TextFormatRange {
        __flight_identity: std::sync::Arc::new(()),
        end: end,
        format: (*format).clone(),
        start: start,
    };
}
