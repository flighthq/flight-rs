// @generated from upstream/packages/types/src/SpriteRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BatchFormat;

// Source: upstream/packages/types/src/SpriteRenderer.ts:7 (sha256:6c5c82e63871de2289003cf2c64d5554876fb51f355b187034bbfbd2f5e5d948)
#[derive(Clone)]
pub struct SpriteRenderer {
    pub format: Option<BatchFormat>,
    pub create_data: crate::OpaqueHostValue,
    pub destroy_data: Option<crate::OpaqueHostValue>,
    pub submit: crate::OpaqueHostValue,
}
