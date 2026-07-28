// @generated from upstream/packages/types/src/ClearcoatPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/ClearcoatPbrMaterial.ts:9 (sha256:5a1b624a30b93da464dee8f67fcb3068ba0d034302bd5e979b853388f2a51369)
#[derive(Clone)]
pub struct ClearcoatPbrMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub clearcoat: f64,
    pub clearcoat_map: Option<Texture>,
    pub clearcoat_normal_map: Option<Texture>,
    pub clearcoat_roughness: f64,
    pub clearcoat_roughness_map: Option<Texture>,
    pub standard: StandardPbrMaterialProperties,
}

// Source: upstream/packages/types/src/ClearcoatPbrMaterial.ts:18 (sha256:5d4a9fac8216afb0c9a38d0acd9567fa18878c290ad7a197ef8485886b91a804)
pub const CLEARCOAT_PBR_MATERIAL_KIND: &'static str = "ClearcoatPbrMaterial";
