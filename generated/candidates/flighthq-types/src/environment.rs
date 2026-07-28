// @generated from upstream/packages/types/src/Environment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::CubeTexture;

// Source: upstream/packages/types/src/Environment.ts:6 (sha256:37d6730ce4b1aa4065299ff5ae1ab3d22a8d91deca205f4b503605f00ff3a1ed)
#[derive(Clone)]
pub struct Environment {
    pub kind: String,
    pub environment: Option<CubeTexture>,
    pub intensity: f64,
}

// Source: upstream/packages/types/src/Environment.ts:12 (sha256:65722e8f1a29f53e8a8e974096ff0f2e8d794d9f898fcbc9e8cea99357346190)
pub const ENVIRONMENT_KIND: &'static str = "Environment";
