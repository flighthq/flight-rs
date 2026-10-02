// @generated from upstream/packages/types/src/TiledMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledLayer, TiledProperty, TiledTilesetRef};

// Source: upstream/packages/types/src/TiledMap.ts:6 (sha256:9c3c273d1b4cc3b3f28ae4b17bf1834d85f0e84b809028bc71d5b39c76126d21)
pub type TiledStaggerAxis = String;

// Source: upstream/packages/types/src/TiledMap.ts:9 (sha256:71dc989d3532747030f179488d2ecf931dd4f00593a738eeaf2c9172f53844f9)
pub type TiledStaggerIndex = String;

// Source: upstream/packages/types/src/TiledMap.ts:13 (sha256:683916d7a3fc7c916229a6af1731f16f25d9d19f5eab12ac5c3782af8e5b6b9a)
pub type TiledOrientation = String;

// Source: upstream/packages/types/src/TiledMap.ts:17 (sha256:f3aa1003ca838d65814f081cca0cbb3b2d229e394f421872780e723375f136b7)
pub type TiledRenderOrder = String;

// Source: upstream/packages/types/src/TiledMap.ts:24 (sha256:f05f238fa81055c57066f27ad8b332d867a3c20a642fb4bbd1d7a20d121600a4)
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
    pub stagger_axis: Option<TiledStaggerAxis>,
    pub stagger_index: Option<TiledStaggerIndex>,
    pub hex_side_length: Option<f64>,
    pub layers: Vec<TiledLayer>,
    pub tilesets: Vec<TiledTilesetRef>,
    pub properties: Vec<TiledProperty>,
}
impl PartialEq for TiledMap {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
