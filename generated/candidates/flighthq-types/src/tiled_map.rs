// @generated from upstream/packages/types/src/TiledMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledLayer, TiledProperty, TiledTilesetRef};

// Source: upstream/packages/types/src/TiledMap.ts:7 (sha256:683916d7a3fc7c916229a6af1731f16f25d9d19f5eab12ac5c3782af8e5b6b9a)
pub type TiledOrientation = String;

// Source: upstream/packages/types/src/TiledMap.ts:11 (sha256:f3aa1003ca838d65814f081cca0cbb3b2d229e394f421872780e723375f136b7)
pub type TiledRenderOrder = String;

// Source: upstream/packages/types/src/TiledMap.ts:18 (sha256:06addefb47009dd6ad6194898472603ce2dd11f327687e4795e7ed1fa107eb9f)
#[derive(Clone, Default)]
pub struct TiledMap {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub version: String,
    pub tiled_version: Option<String>,
    pub orientation: TiledOrientation,
    pub render_order: TiledRenderOrder,
    pub width: f64,
    pub height: f64,
    pub tile_width: f64,
    pub tile_height: f64,
    pub infinite: bool,
    pub background_color: Option<f64>,
    pub layers: Vec<TiledLayer>,
    pub tilesets: Vec<TiledTilesetRef>,
    pub properties: Vec<TiledProperty>,
}
impl PartialEq for TiledMap {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
