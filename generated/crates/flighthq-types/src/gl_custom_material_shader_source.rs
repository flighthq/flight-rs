// @generated from upstream/packages/types/src/GlCustomMaterialShaderSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlCustomMaterialShaderSource.ts:2 (sha256:3b5b0c8ea32666202f6736c34615afdb80979165d23c93c59e22144094966556)
#[derive(Clone, Default)]
pub struct GlCustomMaterialShaderSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fragment: String,
    pub vertex: String,
}
impl PartialEq for GlCustomMaterialShaderSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
