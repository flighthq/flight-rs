// @generated from upstream/packages/types/src/SheenPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/SheenPbrMaterial.ts:8 (sha256:bf8bbb2a4eae05ef9b0a9f4af3008d76ff6d0dd95b91491391b39256a66f9f5f)
#[derive(Clone)]
pub struct SheenPbrMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub sheen_color: f64,
    pub sheen_color_map: Option<Texture>,
    pub sheen_roughness: f64,
    pub sheen_roughness_map: Option<Texture>,
    pub standard: StandardPbrMaterialProperties,
}

// Source: upstream/packages/types/src/SheenPbrMaterial.ts:16 (sha256:f86a4ea6c9bc13fd4076726140efea67e23c54b89baf5b97da03ab1931e8dcb7)
pub const SHEEN_PBR_MATERIAL_KIND: &'static str = "SheenPbrMaterial";
