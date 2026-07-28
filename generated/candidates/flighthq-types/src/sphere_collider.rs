// @generated from upstream/packages/types/src/SphereCollider.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SphereCollider.ts:3 (sha256:e1395bd52f8ef9e59ced5eba00b1b6fa6801860e758edbb7764c6f482d55d205)
#[derive(Clone, Default)]
pub struct SphereCollider {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub restitution: Option<f64>,
    pub friction: Option<f64>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub mode: String,
}
impl PartialEq for SphereCollider {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SphereCollider.ts:12 (sha256:e5f7ad8d06f247dfbbe9e2f8a5af4d60332e567962f5694c4d5405592757a1f4)
pub const SPHERE_COLLIDER_KIND: &'static str = "SphereCollider";
