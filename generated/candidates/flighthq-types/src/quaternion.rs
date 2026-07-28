// @generated from upstream/packages/types/src/Quaternion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Quaternion.ts:6 (sha256:4c26914e14fa1460f7650189dc991a220e1efb5bcc96ed852feb1693d4f5ec36)
#[derive(Clone, Default)]
pub struct Quaternion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Quaternion.ts:13 (sha256:ff4e523b7a94515b59ec2d73128b3bd4894d875fa8472901df9f0b1fada518e7)
pub type QuaternionLike = Quaternion;
