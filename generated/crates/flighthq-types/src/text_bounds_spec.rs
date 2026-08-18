// @generated from upstream/packages/types/src/TextBoundsSpec.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextAutoSize;

// Source: upstream/packages/types/src/TextBoundsSpec.ts:7 (sha256:22ed1f7a16012f2378cd9af1f778a4fb162eded85e201f8d1ffe4d26c4d6dcde)
#[derive(Clone, Default)]
pub struct TextBoundsSpec {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub width: f64,
    pub word_wrap: Option<bool>,
}
impl PartialEq for TextBoundsSpec {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
