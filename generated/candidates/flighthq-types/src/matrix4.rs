// @generated from upstream/packages/types/src/Matrix4.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Matrix4.ts:3 (sha256:e0189d7efd4bd4cd11bccad543eda6bd532b98e5c3f1c0a9cb72d5b9f6d12a7e)
#[derive(Clone, Default)]
pub struct Matrix4 {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub m: Vec<f32>,
}
impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Matrix4.ts:7 (sha256:10408cec6d4b72983a65a6744836381d39e8cc02d7fca3a4e18305a4eda10d71)
pub type Matrix4Like = Matrix4;
