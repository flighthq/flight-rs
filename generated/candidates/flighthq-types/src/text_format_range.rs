// @generated from upstream/packages/types/src/TextFormatRange.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextFormat;

// Source: upstream/packages/types/src/TextFormatRange.ts:3 (sha256:2fbda001021463d274256011f3219ce1967f554a26d37e1db2e928d8ba7bfbe1)
#[derive(Clone)]
pub struct TextFormatRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub end: f64,
    pub format: TextFormat,
    pub start: f64,
}
impl PartialEq for TextFormatRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
