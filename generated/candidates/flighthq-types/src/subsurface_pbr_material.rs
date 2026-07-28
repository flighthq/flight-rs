// @generated from upstream/packages/types/src/SubsurfacePbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/SubsurfacePbrMaterial.ts:10 (sha256:65c957b56652bf654a5edb4ff862a042de7eb6da4f81f68dc17489e12f8b3a6e)
#[derive(Clone)]
pub struct SubsurfacePbrMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub standard: StandardPbrMaterialProperties,
    pub subsurface: f64,
    pub subsurface_color: f64,
    pub subsurface_map: Option<Texture>,
    pub thickness: f64,
    pub thickness_map: Option<Texture>,
}
impl PartialEq for SubsurfacePbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SubsurfacePbrMaterial.ts:19 (sha256:26fd75a673d5e2d0426b892844f5fd645d2baf3315aa6fb1e1d5c8a7cfa3b5d8)
pub const SUBSURFACE_PBR_MATERIAL_KIND: &'static str = "SubsurfacePbrMaterial";
