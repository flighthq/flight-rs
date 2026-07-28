// @generated from upstream/packages/types/src/SceneHit.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Mesh;

// Source: upstream/packages/types/src/SceneHit.ts:10 (sha256:e9a2b22b1b333a9ccf977475cb5aec34adb10248f4b01b7d65b733cab6ff42f2)
#[derive(Clone)]
pub struct SceneHit {
    pub node: Mesh,
    pub distance: f64,
    pub triangle_index: f64,
    pub u: f64,
    pub v: f64,
    pub w: f64,
    pub point_x: f64,
    pub point_y: f64,
    pub point_z: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}
