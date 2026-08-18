// @generated from upstream/packages/types/src/LottieDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AdvancedBlendMode, AnimationClip, DisplayObject, Image, LottieImageAsset};

// Source: upstream/packages/types/src/LottieDocumentImport.ts:11 (sha256:b97b138e2ffb8a5132a3dd9d9d86a70873374fe606b546316b8593105747b1c3)
#[derive(Clone, Default)]
pub struct LottieDocumentImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolve_image_resource: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(LottieImageAsset) -> Option<Image> + Send + 'static>>,
        >,
    >,
}
impl PartialEq for LottieDocumentImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocumentImport.ts:19 (sha256:d29cc295b5c1d4b844ba0299e7e06a6e416938488922051f1738dbcf4014ad46)
#[derive(Clone, Default)]
pub struct LottieAdvancedBlend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: AdvancedBlendMode,
    pub node: DisplayObject,
}
impl PartialEq for LottieAdvancedBlend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocumentImport.ts:29 (sha256:901d127e5694e632746bb2c671743d5abd0a607a98ba8736dcb08bd25da15542)
#[derive(Clone, Default)]
pub struct LottieDocumentImportResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advanced_blends: Vec<LottieAdvancedBlend>,
    pub clip: AnimationClip,
    pub duration: f64,
    pub frame_rate: f64,
    pub root: DisplayObject,
}
impl PartialEq for LottieDocumentImportResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
