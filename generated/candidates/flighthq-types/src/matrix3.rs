// @generated from upstream/packages/types/src/Matrix3.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Matrix3.ts:3 (sha256:1dd682a2f18f302041b5c8236cc9661efacec65b5af270e201390fca5910b2d3)
#[derive(Clone)]
pub struct Matrix3 {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub m: Vec<f32>,
}
impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Matrix3.ts:7 (sha256:340bc2918ec512c5f79c768f7072006878520ca5f5a843d3e06ac4402a4b4103)
pub type Matrix3Like = Matrix3;
