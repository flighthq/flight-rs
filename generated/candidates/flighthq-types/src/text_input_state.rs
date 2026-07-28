// @generated from upstream/packages/types/src/TextInputState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextInputState.ts:10 (sha256:a9ce5aa975796bb35e867912a36b63da2afa409d140e37ef526468868d688827)
#[derive(Clone)]
pub struct TextInputHistoryEntry {
    pub caret_index_after: f64,
    pub caret_index_before: f64,
    pub merge_kind: Option<String>,
    pub selection_index_after: f64,
    pub selection_index_before: f64,
    pub text_after: String,
    pub text_before: String,
}

// Source: upstream/packages/types/src/TextInputState.ts:20 (sha256:b8c71131b48fb802bf08fc22ab717a50b460ecb96f29c0d9615fb6319184d31c)
#[derive(Clone)]
pub struct TextInputState {
    pub always_show_selection: bool,
    pub caret_color: f64,
    pub caret_index: f64,
    pub caret_width: f64,
    pub desired_caret_x: f64,
    pub display_as_password: bool,
    pub focused: bool,
    pub history: Vec<TextInputHistoryEntry>,
    pub history_index: f64,
    pub history_limit: f64,
    pub password_character: String,
    pub restrict: String,
    pub selection_alpha: f64,
    pub selection_color: f64,
    pub selection_index: f64,
}

// Source: upstream/packages/types/src/TextInputState.ts:44 (sha256:1b5f5456e620e7bbc76f4a5bb4aaa3a55f80a9ebc786347d9c288be4f77737da)
#[derive(Clone)]
pub struct TextInputOptions {
    pub always_show_selection: Option<bool>,
    pub caret_color: Option<f64>,
    pub caret_width: Option<f64>,
    pub display_as_password: Option<bool>,
    pub history_limit: Option<f64>,
    pub password_character: Option<String>,
    pub restrict: Option<String>,
    pub selection_alpha: Option<f64>,
    pub selection_color: Option<f64>,
}
