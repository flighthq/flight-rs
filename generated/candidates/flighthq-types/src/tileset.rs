// @generated from upstream/packages/types/src/Tileset.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureAtlas;

// Source: upstream/packages/types/src/Tileset.ts:4 (sha256:d6e33651124681afb7fbc943153d3030db984440a90a66e9199e28c4e7a67f9d)
#[derive(Clone)]
pub struct Tileset {
    pub atlas: Option<TextureAtlas>,
    pub columns: f64,
    pub margin: f64,
    pub rows: f64,
    pub spacing: f64,
    pub tile_height: f64,
    pub tile_width: f64,
}
