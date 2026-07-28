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
    pub key: String,
    pub modifiers: Vec<ShortcutModifier>,
}
