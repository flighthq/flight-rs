// @generated from upstream/packages/textlayout/src/textMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{TextLayoutResult, TextMetrics};

// Source: upstream/packages/textlayout/src/textMetrics.ts:3 (sha256:847084489806b3129c9219dafdd66d83752fdbb3de22646d9e9a873945f03dd8)
#[derive(Clone)]
struct CreateTextMetricsRecord1 {
    __flight_identity: std::sync::Arc<()>,
    height: f64,
    num_lines: f64,
    width: f64,
}
impl PartialEq for CreateTextMetricsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_text_metrics() -> TextMetrics {
    return TextMetrics {
        __flight_identity: std::sync::Arc::new(()),
        height: 0.0_f64,
        num_lines: 0.0_f64,
        width: 0.0_f64,
    };
}

// Source: upstream/packages/textlayout/src/textMetrics.ts:10 (sha256:9413fc356c0adf62e9038cb4569b98576d07a4624d2342e01105bf3837de52b2)
pub fn get_text_metrics(out: &mut TextMetrics, layout: &TextLayoutResult) -> () {
    out.width = (layout.text_width).ceil();
    out.height = (layout.text_height).ceil();
    out.num_lines = layout.num_lines;
}
