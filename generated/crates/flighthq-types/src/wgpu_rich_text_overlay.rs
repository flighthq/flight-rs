// @generated from upstream/packages/types/src/WgpuRichTextOverlay.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RichText, TextLayoutResult};

// Source: upstream/packages/types/src/WgpuRichTextOverlay.ts:4 (sha256:a7e0376045e3d7fae1fddf0b2c09068970f0e1f5b1b9f617d71a387eda9d23a6)
pub type WgpuRichTextOverlay = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(crate::OpaqueHostValue, RichText, TextLayoutResult, f64, f64, String) -> ()
                + Send
                + 'static,
        >,
    >,
>;
