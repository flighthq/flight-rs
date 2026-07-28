// @generated from upstream/packages/types/src/ColorLut.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ColorLut.ts:7 (sha256:be6ba6a1866c93da805e84383b814f93299e5cabc64fcf55fb68cbef7c3d3e19)
#[derive(Clone, Default)]
pub struct ColorLut {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub size: f64,
    pub samples: Vec<f64>,
}
impl PartialEq for ColorLut {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
