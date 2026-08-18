// @generated from upstream/packages/types/src/GlRichTextOverlay.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RichText, TextLayoutResult};

// Source: upstream/packages/types/src/GlRichTextOverlay.ts:4 (sha256:6fb80ef1dc60f31637f9ed3cdb3c6008507a34a37452f20f140ff02ad7b4d3e6)
pub type GlRichTextOverlay = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(crate::OpaqueHostValue, RichText, TextLayoutResult, f64, f64, String) -> ()
                + Send
                + 'static,
        >,
    >,
>;
