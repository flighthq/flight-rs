// @generated from upstream/packages/types/src/Node2DAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node2D, Node2DAnimationPath};

// Source: upstream/packages/types/src/Node2DAnimationTarget.ts:6 (sha256:24e4ce4469e52a87977989557cdc347849f15faf58faa71220caf23e52431a4e)
#[derive(Clone, Default)]
pub struct Node2DAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub node: Node2D,
    pub path: Node2DAnimationPath,
}
impl PartialEq for Node2DAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
