// @generated from upstream/packages/types/src/FrameScript.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Node2D;

// Source: upstream/packages/types/src/FrameScript.ts:5 (sha256:1de0777829cda78138900e751c005f60769df863bb38121cebafd86413a0fefc)
pub type FrameScript =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node2D, f64) -> () + Send + 'static>>>;
