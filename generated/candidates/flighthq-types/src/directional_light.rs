// @generated from upstream/packages/types/src/DirectionalLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3;

// Source: upstream/packages/types/src/DirectionalLight.ts:6 (sha256:5aa15d73a4d69dda6f617f278e05d90700a178b45474ec248e36e1a1139373ae)
#[derive(Clone)]
pub struct DirectionalLight {
    pub kind: String,
    pub casts_shadow: bool,
    pub color: f64,
    pub direction: Vector3,
    pub intensity: f64,
    pub normal_bias: f64,
    pub pcf_radius: f64,
    pub shadow_bias: f64,
}

// Source: upstream/packages/types/src/DirectionalLight.ts:17 (sha256:14d67d7a33aa627cd20f5f9d8654dc1da943a0d5920a3d62d8399d764b3550de)
pub const DIRECTIONAL_LIGHT_KIND: &'static str = "DirectionalLight";
