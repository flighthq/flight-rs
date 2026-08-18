// @generated from upstream/packages/types/src/GlShapeMeshBinding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlShapeMeshBinding.ts:4 (sha256:7b8ccd20857576a7d0ede53323b5a86d6d23b8b307faeba0bc2f406ec71547b2)
#[derive(Clone, Default)]
pub struct GlShapeMeshBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub vertex_buffer: crate::OpaqueHostValue,
    pub index_buffer: crate::OpaqueHostValue,
    pub position_location: f64,
    pub matrix_location: Option<crate::OpaqueHostValue>,
    pub color_location: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlShapeMeshBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
