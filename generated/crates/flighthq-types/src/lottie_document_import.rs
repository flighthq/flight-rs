// @generated from upstream/packages/types/src/LottieDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AnimationClip, DisplayObject, EntityRuntime, ImageResource, LottieAdvancedBlend,
    LottieImageAsset, LottieLayerHandlerEntry, LottieMaskHandlerEntry, LottieShapeItemHandlerEntry,
};

// Source: upstream/packages/types/src/LottieDocumentImport.ts:13 (sha256:1e2b32dd822a222a52b21766a512c860b779a4567ebe751d21fe76e315084d14)
#[derive(Clone, Default)]
pub struct LottieDocumentImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub layer_handlers: Option<Vec<LottieLayerHandlerEntry>>,
    pub mask_handlers: Option<Vec<LottieMaskHandlerEntry>>,
    pub resolve_image_resource: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(LottieImageAsset) -> Option<ImageResource> + Send + 'static>,
            >,
        >,
    >,
    pub shape_item_handlers: Option<Vec<LottieShapeItemHandlerEntry>>,
}
impl PartialEq for LottieDocumentImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocumentImport.ts:20 (sha256:111c59a5d322dfa35bf314861944934b12c2e2a96dcccddd34d3abccac338871)
#[derive(Clone, Default)]
pub struct LottieDocumentImportResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for LottieDocumentImportResult {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
