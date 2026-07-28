// @generated from upstream/packages/types/src/CustomShaderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/CustomShaderMaterial.ts:11 (sha256:a8c84d5a06304f2943dc6a1e7a3b630148b0705730330018dafd7e117b112f4c)
#[derive(Clone, Default)]
pub struct CustomShaderMaterial {
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
}
impl PartialEq for CustomShaderMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CustomShaderMaterial.ts:18 (sha256:ada241d3c0d85c611924cc672149d8d4df6290a9aa9161188ee7a35c7d207ab2)
pub const CUSTOM_SHADER_MATERIAL_KIND: &'static str = "CustomShaderMaterial";
