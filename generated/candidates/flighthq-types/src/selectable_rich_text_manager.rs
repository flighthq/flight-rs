// @generated from upstream/packages/types/src/SelectableRichTextManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RichText;

// Source: upstream/packages/types/src/SelectableRichTextManager.ts:3 (sha256:6d058addff5ee1865dc3b94841d6a732b6c5a31f6b9474f0cb5900253ceebf44)
#[derive(Clone, Default)]
pub struct SelectableRichTextManager {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub focused: Option<RichText>,
}
impl PartialEq for SelectableRichTextManager {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
