// @generated from upstream/packages/types/src/SpecularPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/SpecularPbrMaterial.ts:9 (sha256:2bead9a06dcdc8051b768a13fda3930c65487e64e643d1439da8b709d5d437e2)
#[derive(Clone)]
pub struct SpecularPbrMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub specular: f64,
    pub specular_color: f64,
    pub specular_color_map: Option<Texture>,
    pub specular_map: Option<Texture>,
    pub standard: StandardPbrMaterialProperties,
}

// Source: upstream/packages/types/src/SpecularPbrMaterial.ts:17 (sha256:b004cfe22309bb64bfa674d73b685cfeaa905bfb14758fae6f5441f508b9b420)
pub const SPECULAR_PBR_MATERIAL_KIND: &'static str = "SpecularPbrMaterial";
