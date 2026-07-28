// @generated from upstream/packages/types/src/ColorLutCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorLut;

// Source: upstream/packages/types/src/ColorLutCache.ts:10 (sha256:0d473d81ac14313acf8701b87b9845ba0b6598bffce370c9661c9a3a0d756df9)
#[derive(Clone)]
pub struct ColorLutCache {
    pub signature: Option<String>,
    pub lut: Option<ColorLut>,
}
