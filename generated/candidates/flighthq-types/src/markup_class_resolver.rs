// @generated from upstream/packages/types/src/MarkupClassResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextFormat;

// Source: upstream/packages/types/src/MarkupClassResolver.ts:9 (sha256:c052e7b035ddc0f1752c791c0cf69bcccee961e0005304d3c3b41c35f8ebe5ff)
pub type MarkupClassResolver =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> Option<TextFormat> + Send + 'static>>>;
