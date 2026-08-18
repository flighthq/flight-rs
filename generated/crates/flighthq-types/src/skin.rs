// @generated from upstream/packages/types/src/Skin.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node3D, Skeleton3D};

// Source: upstream/packages/types/src/Skin.ts:13 (sha256:b7e8ad399e4c8c4380cafd3c1a2f3e74211782882d88f21901a89e54cd5628e8)
#[derive(Clone, Default)]
pub struct Skin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub skeleton: Skeleton3D,
    pub skeleton_root: Option<Node3D>,
}
impl PartialEq for Skin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
