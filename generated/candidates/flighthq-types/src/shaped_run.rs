// @generated from upstream/packages/types/src/ShapedRun.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FontResource, TextDirection};

// Source: upstream/packages/types/src/ShapedRun.ts:4 (sha256:97982e4b3f7fa038ad153749e13161c636d57c83607c9e39fce12652e3168c43)
#[derive(Clone)]
pub struct ShapedGlyph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cluster: f64,
    pub glyph_id: f64,
    pub x_advance: f64,
    pub x_offset: f64,
    pub y_advance: f64,
    pub y_offset: f64,
}
impl PartialEq for ShapedGlyph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapedRun.ts:13 (sha256:8b0fb4643dfec361ac4d51caaaef5867c9e05efc2ef364fddcf328380fc07ac5)
#[derive(Clone)]
pub struct ShapedRun {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advance_width: f64,
    pub direction: TextDirection,
    pub font: Option<FontResource>,
    pub glyph_count: f64,
    pub glyphs: Vec<ShapedGlyph>,
    pub script: String,
}
impl PartialEq for ShapedRun {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
