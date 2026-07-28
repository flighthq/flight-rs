// @generated from upstream/packages/types/src/Transform2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Transform2D.ts:7 (sha256:749a846f4dec5c40a8c70da40251904477dd924b7522bf0bc30df38e2aa6a04f)
#[derive(Clone)]
pub struct Transform2D {
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/Transform2D.ts:19 (sha256:8877f144b6ec3b9ec8849be7ab2ebb42aa9e59ebea9192de2fb4629b55388755)
pub type Transform2DLike = Transform2D;
