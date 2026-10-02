// @generated from upstream/packages/types/src/MeshGeometryUvSetExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PbrUvSet;

// Source: upstream/packages/types/src/MeshGeometryUvSetExplanation.ts:14 (sha256:f63735e288b4ec58f1fa56abba4cabb338a565ab6dd9e25507fcd2792034ef8b)
#[derive(Clone, Default)]
pub struct MeshGeometryUvSetExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub available_uv_sets: Vec<PbrUvSet>,
    pub requested_uv_sets: Vec<PbrUvSet>,
    pub samples_at_origin: bool,
    pub unserved_uv_sets: Vec<PbrUvSet>,
}
impl PartialEq for MeshGeometryUvSetExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
