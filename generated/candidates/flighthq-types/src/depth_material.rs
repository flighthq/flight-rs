// @generated from upstream/packages/types/src/DepthMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode};

// Source: upstream/packages/types/src/DepthMaterial.ts:6 (sha256:2b6e077b6b1679a2d911c05c53b37f74ef4d6522381ff80e683b663407c47559)
#[derive(Clone)]
pub struct DepthMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub far: f64,
    pub near: f64,
}

// Source: upstream/packages/types/src/DepthMaterial.ts:11 (sha256:c1194c7b3c562cb4f9279d00f877fb6fc7a9b4a810295bad4a58f1ef5fe562ce)
pub const DEPTH_MATERIAL_KIND: &'static str = "DepthMaterial";
