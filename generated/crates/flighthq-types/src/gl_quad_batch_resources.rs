// @generated from upstream/packages/types/src/GlQuadBatchResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::GlQuadBatchShader;

// Source: upstream/packages/types/src/GlQuadBatchResources.ts:3 (sha256:5e6b02f48bfb8745053ce6cd7b867c0879321c9677a370948979cba065dd1174)
#[derive(Clone, Default)]
pub struct GlQuadBatchResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub corner_buffer: crate::OpaqueHostValue,
    pub shader: GlQuadBatchShader,
    pub writer_color_scale_bias_buffer: Option<crate::OpaqueHostValue>,
    pub writer_instance_buffer: Option<crate::OpaqueHostValue>,
    pub writer_material_buffer: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlQuadBatchResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
