// @generated from upstream/packages/types/src/EmissiveMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/EmissiveMaterial.ts:7 (sha256:69801d3535461cd2a982b945b3099efbf3129d5d5152ed829a5e5ab849141c58)
#[derive(Clone, Default)]
pub struct EmissiveMaterial {
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
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: f64,
}
impl PartialEq for EmissiveMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/EmissiveMaterial.ts:13 (sha256:0ea80f244d12092673fd5f187113c720c534bde41773158f4178e57fa1f4f09f)
pub const EMISSIVE_MATERIAL_KIND: &'static str = "EmissiveMaterial";
