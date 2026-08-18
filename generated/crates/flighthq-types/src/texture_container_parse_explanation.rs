// @generated from upstream/packages/types/src/TextureContainerParseExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TextureContainerKind, TextureContainerParseFailureReason};

// Source: upstream/packages/types/src/TextureContainerParseExplanation.ts:4 (sha256:3418bf65dcf3d2d73b52bea1593f991ccc1c6b65e0a7d7740e69a231badda4d0)
#[derive(Clone, Default)]
pub struct TextureContainerParseExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub container: Option<TextureContainerKind>,
    pub reason: TextureContainerParseFailureReason,
}
impl PartialEq for TextureContainerParseExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
