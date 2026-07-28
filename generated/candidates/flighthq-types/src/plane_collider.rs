// @generated from upstream/packages/types/src/PlaneCollider.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PlaneCollider.ts:3 (sha256:f907b1021536fba69371bfd0af406ef009dd4095952f0183e8c9d04a58d6277c)
#[derive(Clone, Default)]
pub struct PlaneCollider {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub restitution: Option<f64>,
    pub friction: Option<f64>,
    pub kind: String,
    pub nx: f64,
    pub ny: f64,
    pub nz: Option<f64>,
    pub distance: f64,
}
impl PartialEq for PlaneCollider {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PlaneCollider.ts:11 (sha256:b0c26c460be14e7ca0c054ed182d9aebbdd46c7e2abf5aa6f4e6fac92a21075c)
pub const PLANE_COLLIDER_KIND: &'static str = "PlaneCollider";
