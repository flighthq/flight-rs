// @generated from upstream/packages/types/src/IridescencePbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/IridescencePbrMaterial.ts:10 (sha256:5b3ea5014a55ca5d6c53018cfec04235621f6edd3a6f19fe85cf7a70a426a95e)
#[derive(Clone)]
pub struct IridescencePbrMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub iridescence: f64,
    pub iridescence_ior: f64,
    pub iridescence_map: Option<Texture>,
    pub iridescence_thickness_map: Option<Texture>,
    pub iridescence_thickness_max: f64,
    pub iridescence_thickness_min: f64,
    pub standard: StandardPbrMaterialProperties,
}

// Source: upstream/packages/types/src/IridescencePbrMaterial.ts:20 (sha256:b2c34ab23c39d3827393d7429d34c648d211ac43d4f9201fea05b009c71c5b79)
pub const IRIDESCENCE_PBR_MATERIAL_KIND: &'static str = "IridescencePbrMaterial";
