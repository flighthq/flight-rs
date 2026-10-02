// @generated from upstream/packages/types/src/EffectPadding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Effect, Kind};

// Source: upstream/packages/types/src/EffectPadding.ts:4 (sha256:d0fe3a7a803da00acb50c4f6e4575fcb5635369a299e208c7303e55f39e4952d)
#[derive(Clone, Default)]
pub struct EffectPadding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub top: f64,
}
impl PartialEq for EffectPadding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/EffectPadding.ts:11 (sha256:8c1d8de6b770965b9f0ed34b68600c08b0b20fe7848fb1d0da846dadb93c1c1e)
pub type EffectPaddingResolver =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Effect) -> EffectPadding + Send + 'static>>>;

// Source: upstream/packages/types/src/EffectPadding.ts:13 (sha256:73f610f34cd499115a91d26b4ecea94ce4a157ef120c41ebea4eb912d372c9f9)
pub type EffectPaddingStatus = String;

// Source: upstream/packages/types/src/EffectPadding.ts:15 (sha256:3170bab106b3a41a7fe589a58adbbca56390461c9882064da7187ddf5d4a7219)
#[derive(Clone, Default)]
pub struct EffectPaddingExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub missing_kinds: Vec<Kind>,
    pub padding: EffectPadding,
    pub status: EffectPaddingStatus,
}
impl PartialEq for EffectPaddingExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
