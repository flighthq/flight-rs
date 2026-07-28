// @generated from upstream/packages/types/src/Camera2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Rectangle;

// Source: upstream/packages/types/src/Camera2D.ts:13 (sha256:c8d86eb25d0a7636de1d27eead37d006d22b9ac4a8a6e55a07e4b1227f3a7a9e)
#[derive(Clone)]
pub struct Camera2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rotation: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}
impl PartialEq for Camera2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Camera2D.ts:33 (sha256:1a6e6412cf22c4268643adadbc08fab5e10f9911ce8eac4b47b5883e9c739143)
#[derive(Clone)]
pub struct Camera2DFollowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deadzone_half_height: Option<f64>,
    pub deadzone_half_width: Option<f64>,
    pub smooth_time: Option<f64>,
    pub world_bounds: Option<Rectangle>,
}
impl PartialEq for Camera2DFollowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Camera2D.ts:42 (sha256:a2dd28e6be1aa508210a4e0081af1eda684944dbeee1bc32166e096a2bc2dced)
#[derive(Clone)]
pub struct Camera2DOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rotation: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub zoom: Option<f64>,
}
impl PartialEq for Camera2DOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
