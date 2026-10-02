// @generated from upstream/packages/types/src/GlColorAdjustmentResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlColorScaleBiasInstancedShader, GlUniformColorScaleBiasShader};

// Source: upstream/packages/types/src/GlColorAdjustmentResources.ts:3 (sha256:3e9420e68ced9557d9a890e76739dde76b477c913f741b22f9324496375474e3)
#[derive(Clone, Default)]
pub struct GlColorAdjustmentResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub matrix_instanced_shader: GlColorScaleBiasInstancedShader,
    pub scale_bias_instanced_shader: GlColorScaleBiasInstancedShader,
    pub tint_instanced_shader: GlColorScaleBiasInstancedShader,
    pub uniform_scale_bias_shader: GlUniformColorScaleBiasShader,
}
impl PartialEq for GlColorAdjustmentResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
