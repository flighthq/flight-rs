// @generated from upstream/packages/types/src/SpecularGlossinessPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/SpecularGlossinessPbrMaterial.ts:8 (sha256:0507be5be486444087da384892e2e4cc933f986b96fce65dc8cae8f6304a069f)
#[derive(Clone)]
pub struct SpecularGlossinessPbrMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: f64,
    pub glossiness: f64,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: f64,
    pub specular: f64,
    pub specular_glossiness_map: Option<Texture>,
}

// Source: upstream/packages/types/src/SpecularGlossinessPbrMaterial.ts:23 (sha256:4056b9ab3bec4567e887ace32f4b9fbacf942f81b478c6539e5699f27fad407d)
pub const SPECULAR_GLOSSINESS_PBR_MATERIAL_KIND: &'static str = "SpecularGlossinessPbrMaterial";
