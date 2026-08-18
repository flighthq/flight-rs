// @generated from upstream/packages/types/src/SvgDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Image;

// Source: upstream/packages/types/src/SvgDocumentImport.ts:7 (sha256:02c4f707180d138fa944a4fbaa30ca5914362ac8bfbd2b974e7c6fd9be6b4a6f)
#[derive(Clone, Default)]
pub struct SvgDocumentImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolve_image_resource: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> Option<Image> + Send + 'static>>>,
    >,
}
impl PartialEq for SvgDocumentImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
