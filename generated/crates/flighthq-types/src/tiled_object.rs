// @generated from upstream/packages/types/src/TiledObject.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledProperty, Vector2Like};

// Source: upstream/packages/types/src/TiledObject.ts:9 (sha256:b73f04b060019ef91a146e66dae2654a4fd9137c9f3f5f3985bff765667ee5a2)
#[derive(Clone, Default)]
pub struct TiledObject {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub name: String,
    pub type_: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub gid: Option<f64>,
    pub point: bool,
    pub ellipse: bool,
    pub polygon: Option<Vec<Vector2Like>>,
    pub polyline: Option<Vec<Vector2Like>>,
    pub rotation: f64,
    pub visible: bool,
    pub properties: Vec<TiledProperty>,
}
impl PartialEq for TiledObject {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
