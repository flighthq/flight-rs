// @generated from upstream/packages/types/src/SvgDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImageResource, SvgClipHandlerEntry, SvgElementHandlerEntry};

// Source: upstream/packages/types/src/SvgDocumentImport.ts:4 (sha256:2ddf86f39d735cc7f872e5bb23ca30204e74e0b47b0ef1bf30fe48477a4c4b90)
#[derive(Clone, Default)]
pub struct SvgDocumentImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip_handlers: Option<Vec<SvgClipHandlerEntry>>,
    pub element_handlers: Option<Vec<SvgElementHandlerEntry>>,
    pub resolve_image_resource: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(String) -> Option<ImageResource> + Send + 'static>>,
        >,
    >,
}
impl PartialEq for SvgDocumentImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
