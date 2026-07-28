// @generated from upstream/packages/types/src/Obb.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Obb.ts:6 (sha256:11179d71566a75a56eb4e9778d3032adf6a87a52cdd4ab1f888e67e145a6cd86)
#[derive(Clone)]
pub struct Obb {
    pub center_x: f64,
    pub center_y: f64,
    pub center_z: f64,
    pub half_extent_x: f64,
    pub half_extent_y: f64,
    pub half_extent_z: f64,
    pub orientation_w: f64,
    pub orientation_x: f64,
    pub orientation_y: f64,
    pub orientation_z: f64,
}

// Source: upstream/packages/types/src/Obb.ts:19 (sha256:125e1ee9f42a223a1865a79adfb8d6d032807341ff9d59c8d23493c9ee55ac59)
pub type ObbLike = Obb;
