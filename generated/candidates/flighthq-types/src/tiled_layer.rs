// @generated from upstream/packages/types/src/TiledLayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledObject, TiledProperty};

// Source: upstream/packages/types/src/TiledLayer.ts:9 (sha256:2d20861db9bf593f294963fcbdb3b3f0ab935e0c465e3d213b67d1aaff5b3a22)
#[derive(Clone)]
pub struct TiledLayerBase {
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub properties: Vec<TiledProperty>,
}

// Source: upstream/packages/types/src/TiledLayer.ts:19 (sha256:74eafef6a565157e5970c142944550be373bd48e9bfd2fe885439256462d07b4)
#[derive(Clone)]
pub struct TiledTileLayer {
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub width: f64,
    pub height: f64,
    pub data: Vec<u32>,
}

// Source: upstream/packages/types/src/TiledLayer.ts:29 (sha256:958f497af9dc09e685c1f7282cc9a69e1c0d647240663b9eab0d21a101d0d577)
#[derive(Clone)]
pub struct TiledObjectGroup {
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub objects: Vec<TiledObject>,
}

// Source: upstream/packages/types/src/TiledLayer.ts:34 (sha256:6fb8c67ce2d25882e8d4d9135bcc7f8153d2dca237447a72e913e4767cb5f3d5)
#[derive(Clone)]
pub struct TiledImageLayer {
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub image: String,
}

// Source: upstream/packages/types/src/TiledLayer.ts:39 (sha256:c4b29754412667f51fc041ca12b866ff1f739c898f34cf9c020a45dc8dd2c7ef)
#[derive(Clone)]
pub struct TiledGroupLayer {
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub layers: Vec<TiledLayer>,
}

// Source: upstream/packages/types/src/TiledLayer.ts:44 (sha256:7b1454d3f87e66313adfa3a2953ea116e3d3adbfc8fb936b8c8628765cf8cdf4)
pub type TiledLayer = crate::OpaqueHostValue;
