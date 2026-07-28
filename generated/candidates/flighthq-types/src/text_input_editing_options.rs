// @generated from upstream/packages/types/src/TextInputEditingOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextLayoutResult;

// Source: upstream/packages/types/src/TextInputEditingOptions.ts:3 (sha256:5590b9268fa80b60dca5aef1830cbce447b9bc3240aa278d1e8c521d2f3c2868)
#[derive(Clone)]
pub struct HandleTextInputKeyboardOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clipboard_text: Option<String>,
    pub layout: Option<TextLayoutResult>,
    pub on_copy:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
}
impl PartialEq for HandleTextInputKeyboardOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextInputEditingOptions.ts:11 (sha256:3682fbf9a96dcd27f1b6c9bc18511c79dd7807f6758e8170b382151c31a2fc2a)
#[derive(Clone)]
pub struct ReplaceTextInputOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply_input_rules: Option<bool>,
    pub merge_kind: Option<String>,
    pub skip_history: Option<bool>,
}
impl PartialEq for ReplaceTextInputOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
