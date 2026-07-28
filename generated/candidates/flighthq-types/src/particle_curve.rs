// @generated from upstream/packages/types/src/ParticleCurve.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ParticleCurve.ts:1 (sha256:32554e3b8eebf5aaf9423083c77b376debdbb746939cdb6cf5f9df8b525ea0f2)
pub type ParticleCurve = Vec<f64>;

// Source: upstream/packages/types/src/ParticleCurve.ts:3 (sha256:f83b652e7da945380c930493f2fe2ceaad851d4d004b1823fa74f8b9919f746e)
#[derive(Clone, Default)]
pub struct CurveKeyframe {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub value: f64,
}
impl PartialEq for CurveKeyframe {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleCurve.ts:8 (sha256:ff48c933d515c2ffcd75088c60956bfda90f1054ba54f4a053d7bf4924dbafc5)
#[derive(Clone, Default)]
pub struct ColorKeyframe {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl PartialEq for ColorKeyframe {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
