// @generated from upstream/packages/types/src/VortexForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ForceFalloff;

// Source: upstream/packages/types/src/VortexForce.ts:3 (sha256:6aaa77d84372f1551d6fc26daaed4a2c899f8b66961ddd301bf997171fd67014)
#[derive(Clone)]
pub struct VortexForce {
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub axis_x: Option<f64>,
    pub axis_y: Option<f64>,
    pub axis_z: Option<f64>,
    pub strength: f64,
    pub radius: Option<f64>,
    pub falloff: Option<ForceFalloff>,
}

// Source: upstream/packages/types/src/VortexForce.ts:16 (sha256:e590b9a080af88357abaa3562bde81c8600bf2d28fa3ad351c999a047f7a4124)
pub const VORTEX_FORCE_KIND: &'static str = "VortexForce";
