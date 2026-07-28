// @generated from upstream/packages/types/src/GlShapeMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlShapeMesh.ts:6 (sha256:a98d63487f02cfd3f29e097d5ce1d4a4e7b76d27a12a74cd459a5383365de922)
#[derive(Clone)]
pub struct GlShapeMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub color: f64,
    pub alpha: f64,
}
impl PartialEq for GlShapeMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
