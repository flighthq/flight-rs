// @generated from upstream/packages/types/src/AnisotropyPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/AnisotropyPbrMaterial.ts:9 (sha256:7d4930b865e98d18c06c99337f13c915fac0fadbc100b8e0cecd23d4ca9d2080)
#[derive(Clone)]
pub struct AnisotropyPbrMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub anisotropy_map: Option<Texture>,
    pub anisotropy_rotation: f64,
    pub anisotropy_strength: f64,
    pub standard: StandardPbrMaterialProperties,
}
impl PartialEq for AnisotropyPbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnisotropyPbrMaterial.ts:16 (sha256:e598aa4a77417c54e87bb5884df9e0299d828d945bc5d404d070d3cdbfc954a3)
pub const ANISOTROPY_PBR_MATERIAL_KIND: &'static str = "AnisotropyPbrMaterial";
