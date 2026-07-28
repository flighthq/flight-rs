// @generated from upstream/packages/types/src/ShadedMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Modifier, Texture};

// Source: upstream/packages/types/src/ShadedMaterial.ts:23 (sha256:f012cad97304e5b646c0f93382b021b88256802524f06f31c7c237f4904454f6)
#[derive(Clone)]
pub struct ShadedMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub modifiers: Vec<Modifier>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub shininess: f64,
    pub specular: f64,
    pub specular_map: Option<Texture>,
}

// Source: upstream/packages/types/src/ShadedMaterial.ts:34 (sha256:b7634fc89156534c53372e5bebcbed6102390550b5bcf8dc936efd1d74d135f0)
pub const SHADED_MATERIAL_KIND: &'static str = "ShadedMaterial";
