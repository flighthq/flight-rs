// @generated from upstream/packages/types/src/GlShapeMeshResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlShapeMeshBinding, GlShapeMeshColorScaleBiasShader};

// Source: upstream/packages/types/src/GlShapeMeshResources.ts:4 (sha256:e0903dd2131c2e8014c2d70675331dd5efefe8a05e2658d80dda7d762b5e527e)
#[derive(Clone, Default)]
pub struct GlShapeMeshResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: GlShapeMeshBinding,
    pub color_matrix_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub color_scale_bias_shader: Option<GlShapeMeshColorScaleBiasShader>,
}
impl PartialEq for GlShapeMeshResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
