// @generated from upstream/packages/textlayout/src/textMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, TextLayoutResult, TextMetrics};

// Source: upstream/packages/textlayout/src/textMetrics.ts:4 (sha256:4e1204e34ce951f5a9d28f306557024807aeafa8f77058066e6fdd5b002bb8dc)
pub fn create_text_metrics() -> TextMetrics {
    let mut out = allocate_entity();
    initialize_text_metrics((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/textlayout/src/textMetrics.ts:13 (sha256:9413fc356c0adf62e9038cb4569b98576d07a4624d2342e01105bf3837de52b2)
pub fn get_text_metrics(out: &mut TextMetrics, layout: &TextLayoutResult) -> () {
    out.width = (layout.text_width).ceil();
    out.height = (layout.text_height).ceil();
    out.num_lines = layout.num_lines;
}

// Source: upstream/packages/textlayout/src/textMetrics.ts:19 (sha256:768b03d25503d1bbd51dce6dec94a3abe7f0a9736d6835c40afd1b6df3b520e8)
pub fn initialize_text_metrics(out: EntityConstruction<TextMetrics>) -> () {
    crate::host_set("host.height", 0.0_f64);
    crate::host_set("host.numLines", 0.0_f64);
    crate::host_set("host.width", 0.0_f64);
}
