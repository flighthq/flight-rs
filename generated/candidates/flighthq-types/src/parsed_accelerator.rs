// @generated from upstream/packages/types/src/ParsedAccelerator.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ShortcutModifier;

// Source: upstream/packages/types/src/ParsedAccelerator.ts:5 (sha256:e5a370a23deaa2973797ecbab7dda4326fc52ebf08a72fc55747a4ce9957c748)
#[derive(Clone)]
pub struct ParsedAccelerator {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: String,
    pub modifiers: Vec<ShortcutModifier>,
}
impl PartialEq for ParsedAccelerator {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
