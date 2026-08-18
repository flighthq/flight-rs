// @generated from upstream/packages/types/src/MeshGeometryFromAttributesOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MeshGeometryFromAttributesOptions.ts:5 (sha256:594c1b6b37974803ded7399e56221006adb4f5cfd748c999ef2740be53a08cce)
#[derive(Clone, Default)]
pub struct MeshGeometryFromAttributesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Option<crate::FlightUnion2<Vec<f64>, crate::FlightUnion2<Vec<u16>, Vec<u32>>>>,
    pub normals: Option<Vec<f64>>,
    pub positions: Vec<f64>,
    pub uvs: Option<Vec<f64>>,
}
impl PartialEq for MeshGeometryFromAttributesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
