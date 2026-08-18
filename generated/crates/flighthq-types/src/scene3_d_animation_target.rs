// @generated from upstream/packages/types/src/Scene3DAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node3D, Scene3DAnimationPath};

// Source: upstream/packages/types/src/Scene3DAnimationTarget.ts:9 (sha256:6806e5db114691f53b9d2c27bdfdcd73a46cb10804cd0f91129bde139d40260f)
#[derive(Clone, Default)]
pub struct Scene3DAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub node: Node3D,
    pub path: Scene3DAnimationPath,
}
impl PartialEq for Scene3DAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
