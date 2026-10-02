// @generated from upstream/packages/types/src/TiledLayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledObject, TiledProperty};

// Source: upstream/packages/types/src/TiledLayer.ts:9 (sha256:e1652ebd0f27647309fdd5521587072dd0b3d5bf2ebdb9c85fb72620ca6a22fa)
#[derive(Clone, Default)]
pub struct TiledLayerBase {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub tint_color: Option<f64>,
    pub parallax_x: f64,
    pub parallax_y: f64,
    pub class: String,
    pub properties: Vec<TiledProperty>,
}
impl PartialEq for TiledLayerBase {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledLayer.ts:31 (sha256:74eafef6a565157e5970c142944550be373bd48e9bfd2fe885439256462d07b4)
#[derive(Clone, Default)]
pub struct TiledTileLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub tint_color: Option<f64>,
    pub parallax_x: f64,
    pub parallax_y: f64,
    pub class: String,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub width: f64,
    pub height: f64,
    pub data: Vec<u32>,
}
impl PartialEq for TiledTileLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledLayer.ts:41 (sha256:958f497af9dc09e685c1f7282cc9a69e1c0d647240663b9eab0d21a101d0d577)
#[derive(Clone, Default)]
pub struct TiledObjectGroup {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub tint_color: Option<f64>,
    pub parallax_x: f64,
    pub parallax_y: f64,
    pub class: String,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub objects: Vec<TiledObject>,
}
impl PartialEq for TiledObjectGroup {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledLayer.ts:46 (sha256:cd55582489fe243a3b9f2bf0a56676911b8555e83d8d02a8d9d6e272740e63e9)
#[derive(Clone, Default)]
pub struct TiledImageLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub tint_color: Option<f64>,
    pub parallax_x: f64,
    pub parallax_y: f64,
    pub class: String,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub image: String,
    pub repeat_x: bool,
    pub repeat_y: bool,
}
impl PartialEq for TiledImageLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledLayer.ts:55 (sha256:c4b29754412667f51fc041ca12b866ff1f739c898f34cf9c020a45dc8dd2c7ef)
#[derive(Clone, Default)]
pub struct TiledGroupLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub offset_x: f64,
    pub offset_y: f64,
    pub tint_color: Option<f64>,
    pub parallax_x: f64,
    pub parallax_y: f64,
    pub class: String,
    pub properties: Vec<TiledProperty>,
    pub type_: String,
    pub layers: Vec<TiledLayer>,
}
impl PartialEq for TiledGroupLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledLayer.ts:60 (sha256:7b1454d3f87e66313adfa3a2953ea116e3d3adbfc8fb936b8c8628765cf8cdf4)
pub type TiledLayer = crate::FlightUnion2<
    TiledTileLayer,
    crate::FlightUnion2<TiledObjectGroup, crate::FlightUnion2<TiledImageLayer, TiledGroupLayer>>,
>;
