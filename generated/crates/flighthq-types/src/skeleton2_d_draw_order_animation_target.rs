// @generated from upstream/packages/types/src/Skeleton2DDrawOrderAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node, NodeOrderList, Skeleton2DAnimationTargetKind};

// Source: upstream/packages/types/src/Skeleton2DDrawOrderAnimationTarget.ts:24 (sha256:be5b749551e2edbe4703295b187fb914afcec26e0d7ed692b0922bed742151fc)
#[derive(Clone, Default)]
pub struct Skeleton2DDrawOrderAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Skeleton2DAnimationTargetKind,
    pub nodes: Vec<Option<Node>>,
    pub order_list: NodeOrderList,
}
impl PartialEq for Skeleton2DDrawOrderAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
