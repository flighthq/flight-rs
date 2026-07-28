// @generated from upstream/packages/types/src/SurfaceRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Surface;

// Source: upstream/packages/types/src/SurfaceRegion.ts:3 (sha256:f1eded16cf834bd12772e6b22abdafe531feeab64b0e59d11e2ef44f42e39c41)
#[derive(Clone)]
pub struct SurfaceRegion {
    pub height: f64,
    pub surface: Surface,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
