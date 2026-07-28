// @generated from upstream/packages/types/src/SpritesheetFrameData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SpritesheetFrameData.ts:1 (sha256:7d41b954f16f21af8a6aaffcacc5ea5bdc431fa356a5386e8973a752ffaf57fa)
#[derive(Clone)]
pub struct SpritesheetFrameData {
    pub height: f64,
    pub name: String,
    pub offset_x: f64,
    pub offset_y: f64,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: bool,
    pub source_height: f64,
    pub source_width: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
