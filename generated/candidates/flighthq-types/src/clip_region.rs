// @generated from upstream/packages/types/src/ClipRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PathWinding, Rectangle};

// Source: upstream/packages/types/src/ClipRegion.ts:19 (sha256:f73b90fe6168b429bc413bda84ebe794b96c7345e5da4ab65264c4241d9995b2)
#[derive(Clone)]
pub struct ClipRegion {
    pub rect: Rectangle,
    pub contours: Option<Vec<Vec<f64>>>,
    pub winding: PathWinding,
    pub version: f64,
}
