// @generated from upstream/packages/types/src/MeshTriangleVertexIndices.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MeshTriangleVertexIndices.ts:3 (sha256:db093d6cf16edf8ddaf1b3e33d9956ce8258129d098202c935bb4c4219c7ec0c)
#[derive(Clone, Default)]
pub struct MeshTriangleVertexIndices {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub i0: f64,
    pub i1: f64,
    pub i2: f64,
}
impl PartialEq for MeshTriangleVertexIndices {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
