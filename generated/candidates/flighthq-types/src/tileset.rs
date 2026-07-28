// @generated from upstream/packages/types/src/Tileset.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureAtlas;

// Source: upstream/packages/types/src/Tileset.ts:4 (sha256:d6e33651124681afb7fbc943153d3030db984440a90a66e9199e28c4e7a67f9d)
#[derive(Clone, Default)]
pub struct Tileset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub columns: f64,
    pub margin: f64,
    pub rows: f64,
    pub spacing: f64,
    pub tile_height: f64,
    pub tile_width: f64,
}
impl PartialEq for Tileset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
