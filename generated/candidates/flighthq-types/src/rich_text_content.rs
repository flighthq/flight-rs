// @generated from upstream/packages/types/src/RichTextContent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextFormatRange;

// Source: upstream/packages/types/src/RichTextContent.ts:3 (sha256:048d186739d8bfe34b14f636cd57fb89116b401bab1347c3742749f04b2838be)
#[derive(Clone)]
pub struct RichTextContent {
    pub format_ranges: Vec<TextFormatRange>,
    pub text: String,
}
