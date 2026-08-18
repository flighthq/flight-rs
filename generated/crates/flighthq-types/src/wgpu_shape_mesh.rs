// @generated from upstream/packages/types/src/WgpuShapeMesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuShapeMesh.ts:1 (sha256:a7bf1ac799a7a857268d1b05541826b1ec19e7b1e3f640d485b2a77dfdf793aa)
#[derive(Clone, Default)]
pub struct WgpuShapeMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub color: f64,
    pub alpha: f64,
}
impl PartialEq for WgpuShapeMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
