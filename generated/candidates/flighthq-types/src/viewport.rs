// @generated from upstream/packages/types/src/Viewport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node, ViewportAlign, ViewportScaleMode};

// Source: upstream/packages/types/src/Viewport.ts:8 (sha256:f22bef597526a4a97430496236312e76b67d38a2ed79e7314f5461b0d31da44c)
#[derive(Clone)]
pub struct Viewport {
    pub align: ViewportAlign,
    pub root: Option<Node>,
    pub scale_mode: ViewportScaleMode,
}
