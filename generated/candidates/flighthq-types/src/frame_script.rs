// @generated from upstream/packages/types/src/FrameScript.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::DisplayObject;

// Source: upstream/packages/types/src/FrameScript.ts:5 (sha256:b6b35d6a74a18ef2879465a828b046c2242a8da63ac7777c6f06e79ad3b899e8)
pub type FrameScript =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(DisplayObject, f64) -> () + Send + 'static>>>;
