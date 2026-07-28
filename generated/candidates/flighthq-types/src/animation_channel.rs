// @generated from upstream/packages/types/src/AnimationChannel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AnimationTrack;

// Source: upstream/packages/types/src/AnimationChannel.ts:6 (sha256:35c26b1445ebf6535d070d7b27d3d52b160883c0dc766aa095eb0e7e67bdaa49)
#[derive(Clone)]
pub struct AnimationChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub track: AnimationTrack,
    pub target_ref: crate::OpaqueHostValue,
}
impl PartialEq for AnimationChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
