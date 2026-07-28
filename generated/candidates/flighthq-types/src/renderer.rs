// @generated from upstream/packages/types/src/Renderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BatchFormat;

// Source: upstream/packages/types/src/Renderer.ts:7 (sha256:1e0a00a374e0c6c378f0486a35e66b87e2c0ae90105ea5a59d66c211c6c381fe)
#[derive(Clone)]
pub struct Renderer {
    pub format: Option<BatchFormat>,
    pub create_data: crate::OpaqueHostValue,
    pub destroy_data: Option<crate::OpaqueHostValue>,
    pub submit: crate::OpaqueHostValue,
}
