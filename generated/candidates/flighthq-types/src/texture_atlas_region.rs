// @generated from upstream/packages/types/src/TextureAtlasRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextureAtlasRegion.ts:3 (sha256:65a7d001c8ca2defe349f281d226c6c796c8b9e14f2bf8701f6bc6bdc86dcb5d)
#[derive(Clone)]
pub struct TextureAtlasRegion {
    pub height: f64,
    pub id: f64,
    pub name: Option<String>,
    pub original_height: Option<f64>,
    pub original_width: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: bool,
    pub source_x: f64,
    pub source_y: f64,
    pub trimmed: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}

// Source: upstream/packages/types/src/TextureAtlasRegion.ts:20 (sha256:9863842621d051ad75d93d3a933cbd0c0dac801afd48d9f6d7bb6a7094f9e34d)
pub type TextureAtlasRegionLike = TextureAtlasRegion;
