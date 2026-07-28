// @generated from upstream/packages/types/src/DisplayObjectRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BatchFormat;

// Source: upstream/packages/types/src/DisplayObjectRenderer.ts:7 (sha256:db64412470e3a77b56acccae26cf88379fd21c4c3c11f0a6724d90371334e060)
#[derive(Clone)]
pub struct DisplayObjectRenderer {
    pub format: Option<BatchFormat>,
    pub create_data: crate::OpaqueHostValue,
    pub destroy_data: Option<crate::OpaqueHostValue>,
    pub submit: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/DisplayObjectRenderer.ts:14 (sha256:c7383b822769baacf173c3a1b8042747b3ccbe077769ff1c32ed63a8d9811135)
#[derive(Clone)]
pub struct DisplayObjectClipHooks {
    pub finalize: crate::OpaqueHostValue,
    pub pop_clip: crate::OpaqueHostValue,
    pub push_clip: crate::OpaqueHostValue,
}
