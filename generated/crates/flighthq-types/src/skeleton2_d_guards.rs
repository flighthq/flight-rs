// @generated from upstream/packages/types/src/Skeleton2DGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton2DGuards.ts:10 (sha256:64517b99f08a6a39bbf2a7bbf8186a463a92404ea6db0677c8868900f74ec740)
#[derive(Clone, Default)]
pub struct Skeleton2DCoercedInterpolation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub applied: String,
    pub stated: String,
    pub subject: String,
}
impl PartialEq for Skeleton2DCoercedInterpolation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Skeleton2DGuards.ts:27 (sha256:44e71053670ad27a1d05f7234b11a0117f2babd08dd43d092f03d50b8e0407ed)
#[derive(Clone, Default)]
pub struct Skeleton2DDeformLengthMismatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub addressed: f64,
    pub offsets: f64,
    pub subject: String,
}
impl PartialEq for Skeleton2DDeformLengthMismatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Skeleton2DGuards.ts:37 (sha256:7811b170f1d5fc6d4bb6acb7f8c7346e0558ef610f980992968f47163c907af3)
pub type Skeleton2DCoercedInterpolationGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Skeleton2DCoercedInterpolation) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Skeleton2DGuards.ts:40 (sha256:3da38abd004add746b54d2c2111419b7caa2ff403e7a12383a5d3d5a48807571)
pub type Skeleton2DDeformLengthGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Skeleton2DDeformLengthMismatch) -> () + Send + 'static>>,
>;
