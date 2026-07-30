// @generated from upstream/packages/types/src/RectangleCollider.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RectangleCollider.ts:3 (sha256:d04abecd3e545664430a4e1898ff017dce37cf27a8bcb56a28856a9919551801)
#[derive(Clone, Default)]
pub struct RectangleCollider {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub restitution: Option<f64>,
    pub friction: Option<f64>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub mode: String,
}
impl PartialEq for RectangleCollider {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RectangleCollider.ts:12 (sha256:a14da3ec1e89ad1d05313ba5165bd32cb140e5b0519f395bb7792f904170fc35)
pub const RECTANGLE_COLLIDER_KIND: &'static str = "RectangleCollider";
