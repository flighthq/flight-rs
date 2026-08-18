// @generated from upstream/packages/types/src/MeshGeometryOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MeshSubset, PrimitiveTopology, VertexAttributeLayout};

// Source: upstream/packages/types/src/MeshGeometryOptions.ts:8 (sha256:a33a9efe64b61d07fa86e41111d86d5ec5ba0c4a2c4d9d17ab09563800dc99cb)
#[derive(Clone, Default)]
pub struct MeshGeometryOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Option<Vec<u32>>,
    pub layout: VertexAttributeLayout,
    pub subsets: Option<Vec<MeshSubset>>,
    pub topology: Option<PrimitiveTopology>,
    pub vertices: Vec<f32>,
}
impl PartialEq for MeshGeometryOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
