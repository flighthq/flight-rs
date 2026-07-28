// @generated from upstream/packages/types/src/BlinnPhongMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/BlinnPhongMaterial.ts:8 (sha256:398af6a2a8e8ac3c8ebb3cb210ab657da96659a27250218c3a4773a5ed0e4c88)
#[derive(Clone)]
pub struct BlinnPhongMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub shininess: f64,
    pub specular: f64,
    pub specular_map: Option<Texture>,
}

// Source: upstream/packages/types/src/BlinnPhongMaterial.ts:18 (sha256:5c93195075cdac2ebe9f0994ed3bd2eed4cfdf7c368df0a78d8edb6e7bc30b22)
pub const BLINN_PHONG_MATERIAL_KIND: &'static str = "BlinnPhongMaterial";
