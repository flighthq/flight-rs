// @generated from upstream/packages/types/src/ShadedMaterialOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, MaterialAlphaMode, Modifier, Texture};

// Source: upstream/packages/types/src/ShadedMaterialOptions.ts:8 (sha256:899baee68a77d6e79f89604f36582b807cda30631adbb64722cd348b8d23491e)
#[derive(Clone, Default)]
pub struct ShadedMaterialOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub modifiers: Option<Vec<Modifier>>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub shininess: Option<f64>,
    pub specular: Option<f64>,
    pub specular_map: Option<Texture>,
}
impl PartialEq for ShadedMaterialOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
