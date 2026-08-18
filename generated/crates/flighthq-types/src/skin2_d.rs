// @generated from upstream/packages/types/src/Skin2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skin2D.ts:12 (sha256:c8b58a1904ff60a56c3372ac438d177906fd9c72de5fcf3fcc6c2b03332b2a74)
#[derive(Clone, Default)]
pub struct Skin2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub influence_counts: Vec<u16>,
    pub influences: Vec<f32>,
}
impl PartialEq for Skin2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
