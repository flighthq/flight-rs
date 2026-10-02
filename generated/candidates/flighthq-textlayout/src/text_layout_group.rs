// @generated from upstream/packages/textlayout/src/textLayoutGroup.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, TextFormat, TextLayoutGroup};

// Source: upstream/packages/textlayout/src/textLayoutGroup.ts:4 (sha256:bbc461fb6bedb5b0fa6e9520a6698982ba36fb0682c5f50b511a5357f2cd9774)
pub fn create_text_layout_group(
    format: &TextFormat,
    start_index: f64,
    end_index: f64,
) -> TextLayoutGroup {
    let mut out = allocate_entity();
    initialize_text_layout_group((out).clone(), format, start_index, end_index);
    return finish_entity((out).clone());
}

// Source: upstream/packages/textlayout/src/textLayoutGroup.ts:10 (sha256:6de3e801ea6f9a8d27ea85730257ed601c8e03cd1186353596123897498ae857)
pub fn initialize_text_layout_group(
    out: EntityConstruction<TextLayoutGroup>,
    format: &TextFormat,
    start_index: f64,
    end_index: f64,
) -> () {
    crate::host_set("host.ascent", 0.0_f64);
    crate::host_set("host.descent", 0.0_f64);
    crate::host_set("host.endIndex", end_index);
    crate::host_set("host.format", format);
    crate::host_set("host.height", 0.0_f64);
    crate::host_set("host.leading", 0.0_f64);
    crate::host_set("host.lineIndex", 0.0_f64);
    crate::host_set("host.offsetX", 0.0_f64);
    crate::host_set("host.offsetY", 0.0_f64);
    crate::host_set("host.positions", vec![]);
    crate::host_set("host.startIndex", start_index);
    crate::host_set("host.width", 0.0_f64);
}
