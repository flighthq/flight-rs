// @generated from upstream/packages/types/src/RenderEffectPadding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, RenderEffect};

// Source: upstream/packages/types/src/RenderEffectPadding.ts:4 (sha256:b38af857c9f1ced8a0efa777f84273e301f75abdceb65a7180dd2eef56c4802d)
#[derive(Clone, Default)]
pub struct RenderEffectPadding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub top: f64,
}
impl PartialEq for RenderEffectPadding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderEffectPadding.ts:11 (sha256:8252a418c482d61c37219cd9f89e056da865dbd50e18b6eeb9d14686e972111a)
pub type RenderEffectPaddingResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static>>,
>;

// Source: upstream/packages/types/src/RenderEffectPadding.ts:13 (sha256:4edc1ab7b54066189035f534f7fd4e86fd48714d343aaa60d7dce905da4ca024)
pub type RenderEffectPaddingStatus = String;

// Source: upstream/packages/types/src/RenderEffectPadding.ts:15 (sha256:e067f09fe67c95e3e1f176377554df6bec6d49e2e9f9c7f07c75098359c5a099)
#[derive(Clone, Default)]
pub struct RenderEffectPaddingExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub missing_kinds: Vec<Kind>,
    pub padding: RenderEffectPadding,
    pub status: RenderEffectPaddingStatus,
}
impl PartialEq for RenderEffectPaddingExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
