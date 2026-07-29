// @generated from upstream/packages/textlayout/src/textLayoutGroup.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{TextFormat, TextLayoutGroup};

// Source: upstream/packages/textlayout/src/textLayoutGroup.ts:3 (sha256:ec1569f265001102c47715e3216875aecd9a28c748973edb33964b5cb315b204)
pub fn create_text_layout_group(
    format: &TextFormat,
    start_index: f64,
    end_index: f64,
) -> TextLayoutGroup {
    return TextLayoutGroup {
        __flight_identity: std::sync::Arc::new(()),
        ascent: 0.0_f64,
        descent: 0.0_f64,
        end_index: end_index,
        format: (*format).clone(),
        height: 0.0_f64,
        leading: 0.0_f64,
        line_index: 0.0_f64,
        offset_x: 0.0_f64,
        offset_y: 0.0_f64,
        positions: vec![],
        start_index: start_index,
        width: 0.0_f64,
    };
}
