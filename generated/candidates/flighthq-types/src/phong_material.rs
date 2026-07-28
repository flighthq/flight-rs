// @generated from upstream/packages/types/src/PhongMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/PhongMaterial.ts:7 (sha256:64e437f2e5a0160d04bbc20e190fa582580cd6407ac92088e8b008e8c8d4aa9b)
#[derive(Clone, Default)]
pub struct PhongMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub shader_key: String,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub shininess: f64,
    pub specular: f64,
    pub specular_map: Option<Texture>,
}
impl PartialEq for PhongMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PhongMaterial.ts:17 (sha256:7de1035721a622fb84c8bb99dc6a08e17c8aaef2385219f54b548e4bf5c67636)
pub const PHONG_MATERIAL_KIND: &'static str = "PhongMaterial";
