// @generated from upstream/packages/types/src/SelectableRichTextManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RichText;

// Source: upstream/packages/types/src/SelectableRichTextManager.ts:3 (sha256:6d058addff5ee1865dc3b94841d6a732b6c5a31f6b9474f0cb5900253ceebf44)
#[derive(Clone)]
pub struct SelectableRichTextManager {
    pub focused: Option<RichText>,
}
