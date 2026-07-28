// @generated from upstream/packages/types/src/TextItem.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextDirection;

// Source: upstream/packages/types/src/TextItem.ts:2 (sha256:03eb326d7a5f55efed303d758eb178987ffdc0aa78c46cae7f57a143c6580a16)
#[derive(Clone)]
pub struct TextItem {
    pub direction: TextDirection,
    pub end: f64,
    pub script: String,
    pub start: f64,
}
