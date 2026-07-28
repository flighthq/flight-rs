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
    pub rotation: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

// Source: upstream/packages/types/src/Camera2D.ts:33 (sha256:1a6e6412cf22c4268643adadbc08fab5e10f9911ce8eac4b47b5883e9c739143)
#[derive(Clone)]
pub struct Camera2DFollowOptions {
    pub deadzone_half_height: Option<f64>,
    pub deadzone_half_width: Option<f64>,
    pub smooth_time: Option<f64>,
    pub world_bounds: Option<Rectangle>,
}

// Source: upstream/packages/types/src/Camera2D.ts:42 (sha256:a2dd28e6be1aa508210a4e0081af1eda684944dbeee1bc32166e096a2bc2dced)
#[derive(Clone)]
pub struct Camera2DOptions {
    pub rotation: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub zoom: Option<f64>,
}
