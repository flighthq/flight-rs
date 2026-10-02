// @generated from upstream/packages/textlayout/src/textFormatRange.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, TextFormat, TextFormatRange};

// Source: upstream/packages/textlayout/src/textFormatRange.ts:4 (sha256:a57bb969bac171b1185dcb2c625b5c9288cbdda8977231c7b4bd5feee92aee8a)
pub fn create_text_format_range(format: &TextFormat, start: f64, end: f64) -> TextFormatRange {
    let mut out = allocate_entity();
    initialize_text_format_range((out).clone(), format, start, end);
    return finish_entity((out).clone());
}

// Source: upstream/packages/textlayout/src/textFormatRange.ts:10 (sha256:e504e8fa3d0077966c82f048bbe1b333a081a86ac15175bbaf7780fa43b62cc7)
pub fn initialize_text_format_range(
    out: EntityConstruction<TextFormatRange>,
    format: &TextFormat,
    start: f64,
    end: f64,
) -> () {
    crate::host_set("host.end", end);
    crate::host_set("host.format", format);
    crate::host_set("host.start", start);
}
