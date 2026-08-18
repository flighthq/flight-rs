// @generated from upstream/packages/types/src/Skeleton2DAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Skeleton2DAnimationPath, Skeleton2DAnimationTargetKind};

// Source: upstream/packages/types/src/Skeleton2DAnimationTarget.ts:12 (sha256:4f84711d7e3e4b0a0070988a073c1b8e83c766b537f1d0c2fefbf086bf5b1322)
#[derive(Clone, Default)]
pub struct Skeleton2DAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bone_index: f64,
    pub kind: Skeleton2DAnimationTargetKind,
    pub path: Skeleton2DAnimationPath,
}
impl PartialEq for Skeleton2DAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
