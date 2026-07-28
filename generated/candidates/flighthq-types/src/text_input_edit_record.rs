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
    pub caret_index_after: f64,
    pub caret_index_before: f64,
    pub merge_kind: Option<String>,
    pub selection_index_after: f64,
    pub selection_index_before: f64,
    pub text_after: String,
    pub text_before: String,
}
