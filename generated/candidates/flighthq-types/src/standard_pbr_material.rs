// @generated from upstream/packages/types/src/StandardPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/StandardPbrMaterial.ts:9 (sha256:e3e8bb5da43cdf61422509a2cf0c16e4467b16f4d54b14e2509c5242740f0f9d)
#[derive(Clone)]
pub struct StandardPbrMaterialProperties {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub base_color: f64,
    pub base_color_map: Option<Texture>,
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: f64,
    pub metallic: f64,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: f64,
    pub roughness: f64,
}
impl PartialEq for StandardPbrMaterialProperties {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StandardPbrMaterial.ts:26 (sha256:75623596e21f7fa8bdb96972f77d790d3fa4eaa91a9a238efce37fd2c87cff25)
#[derive(Clone)]
pub struct StandardPbrMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub base_color: f64,
    pub base_color_map: Option<Texture>,
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: f64,
    pub metallic: f64,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: f64,
    pub roughness: f64,
}
impl PartialEq for StandardPbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StandardPbrMaterial.ts:28 (sha256:c10ef55cb5f965b373510f0a8bce10c727e60f301669267c100433d0a01cbd00)
pub const STANDARD_PBR_MATERIAL_KIND: &'static str = "StandardPbrMaterial";
