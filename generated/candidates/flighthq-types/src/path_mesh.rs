// @generated from upstream/packages/types/src/PathMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PathMesh.ts:7 (sha256:66cba4b02f27ccf2d392f2ce60c410aaa294ac9c3344dcc6fe3c41e474430059)
#[derive(Clone)]
pub struct PathMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vertices: Vec<f64>,
    pub indices: Vec<f64>,
}
impl PartialEq for PathMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
