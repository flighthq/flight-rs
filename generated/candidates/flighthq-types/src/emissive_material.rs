// @generated from upstream/packages/types/src/EmissiveMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/EmissiveMaterial.ts:7 (sha256:69801d3535461cd2a982b945b3099efbf3129d5d5152ed829a5e5ab849141c58)
#[derive(Clone)]
pub struct EmissiveMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: f64,
}

// Source: upstream/packages/types/src/EmissiveMaterial.ts:13 (sha256:0ea80f244d12092673fd5f187113c720c534bde41773158f4178e57fa1f4f09f)
pub const EMISSIVE_MATERIAL_KIND: &'static str = "EmissiveMaterial";
