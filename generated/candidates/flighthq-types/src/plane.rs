// @generated from upstream/packages/types/src/Plane.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Plane.ts:7 (sha256:73509be81808ed51445dcbf3fa04b6114e34d5f25f6bbcda267be4299ad107eb)
#[derive(Clone)]
pub struct Plane {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}
impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Plane.ts:14 (sha256:8ec5c9804e5f0eb42f1ab2e8c87457e1708d8ba0a315dfa3712be351ba02da09)
pub type PlaneLike = Plane;
