// @generated from upstream/packages/types/src/TiledTileset.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledObject, TiledProperty};

// Source: upstream/packages/types/src/TiledTileset.ts:6 (sha256:d03a4ec13a0db461ca7538d2c409c6030e58dc2cc2c5929fe64061d173a5d9a8)
#[derive(Clone)]
pub struct TiledTilesetTileFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tile_id: f64,
    pub duration: f64,
}
impl PartialEq for TiledTilesetTileFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledTileset.ts:15 (sha256:f20a5988a4c187a5ab14cafc6d9e22031b7dd254f8a130eb362beafdafe8fe92)
#[derive(Clone)]
pub struct TiledTilesetTile {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub type_: String,
    pub properties: Vec<TiledProperty>,
    pub animation: Option<Vec<TiledTilesetTileFrame>>,
    pub objects: Option<Vec<TiledObject>>,
    pub image: Option<String>,
}
impl PartialEq for TiledTilesetTile {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledTileset.ts:29 (sha256:f7f49b1c5693d038732edcc23550418414f1b7bca0501669372a4e0d11f212eb)
#[derive(Clone)]
pub struct TiledTileset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub tile_width: f64,
    pub tile_height: f64,
    pub tile_count: f64,
    pub columns: f64,
    pub image: Option<String>,
    pub image_width: f64,
    pub image_height: f64,
    pub margin: f64,
    pub spacing: f64,
    pub tiles: Vec<TiledTilesetTile>,
    pub properties: Vec<TiledProperty>,
}
impl PartialEq for TiledTileset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledTileset.ts:48 (sha256:240a78b98b30601002a1f3bfa62be8394bd11f25ff22d798f7c1ac216d01ba3b)
#[derive(Clone)]
pub struct TiledTilesetRef {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub first_gid: f64,
    pub source: Option<String>,
    pub tileset: Option<TiledTileset>,
}
impl PartialEq for TiledTilesetRef {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
