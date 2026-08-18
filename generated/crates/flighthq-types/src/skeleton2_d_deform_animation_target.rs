// @generated from upstream/packages/types/src/Skeleton2DDeformAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Attachment2D, Skeleton2DAnimationTargetKind};

// Source: upstream/packages/types/src/Skeleton2DDeformAnimationTarget.ts:17 (sha256:20cdf3a58d4835ddf1c1e25628cd87009bdd906a60c4605ffcbd38ed52567033)
#[derive(Clone, Default)]
pub struct Skeleton2DDeformAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: Option<Attachment2D>,
    pub kind: Skeleton2DAnimationTargetKind,
    pub slot_index: f64,
}
impl PartialEq for Skeleton2DDeformAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
