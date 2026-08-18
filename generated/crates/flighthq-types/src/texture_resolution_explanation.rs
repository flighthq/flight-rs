// @generated from upstream/packages/types/src/TextureResolutionExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureSourceKind;

// Source: upstream/packages/types/src/TextureResolutionExplanation.ts:3 (sha256:2f48238e94b1d31add2a19898ad3bc537d4d004f26ab01f3d3312639413d5753)
pub type TextureResolutionStatus = String;

// Source: upstream/packages/types/src/TextureResolutionExplanation.ts:9 (sha256:9158485dcccc8b5e1d859928ae1a2f11f2af995636b31f1e9f9530e29b1be60b)
#[derive(Clone, Default)]
pub struct TextureResolutionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<TextureSourceKind>,
    pub status: TextureResolutionStatus,
}
impl PartialEq for TextureResolutionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
