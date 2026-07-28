// @generated from upstream/packages/types/src/TextInputEditRecord.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextInputEditRecord.ts:1 (sha256:ea05859e4ca23f53459b8a910a23c684cd6fa115d61f3e3c658e611d6d8fe522)
#[derive(Clone)]
pub struct TextInputEditRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub caret_index_after: f64,
    pub caret_index_before: f64,
    pub merge_kind: Option<String>,
    pub selection_index_after: f64,
    pub selection_index_before: f64,
    pub text_after: String,
    pub text_before: String,
}
impl PartialEq for TextInputEditRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
