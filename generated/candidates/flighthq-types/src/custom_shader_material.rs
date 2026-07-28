// @generated from upstream/packages/types/src/CustomShaderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, MaterialAlphaMode};

// Source: upstream/packages/types/src/CustomShaderMaterial.ts:11 (sha256:a8c84d5a06304f2943dc6a1e7a3b630148b0705730330018dafd7e117b112f4c)
#[derive(Clone)]
pub struct CustomShaderMaterial {
    pub kind: String,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub shader_key: String,
    pub textures: Option<crate::OpaqueHostValue>,
    pub uniforms: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/CustomShaderMaterial.ts:18 (sha256:ada241d3c0d85c611924cc672149d8d4df6290a9aa9161188ee7a35c7d207ab2)
pub const CUSTOM_SHADER_MATERIAL_KIND: &'static str = "CustomShaderMaterial";
