// @generated from upstream/packages/types/src/GlVelocityWriter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, VelocityField};

// Source: upstream/packages/types/src/GlVelocityWriter.ts:11 (sha256:4bb0b95c7721e34f551efe228cd44c568f2c48c8992d41604f80e0f3dc582058)
#[derive(Clone)]
pub struct GlVelocityContext {
    pub state: GlRenderState,
    pub field: VelocityField,
    pub width: f64,
    pub height: f64,
    pub pixel_ratio: f64,
}

// Source: upstream/packages/types/src/GlVelocityWriter.ts:19 (sha256:d840c60b1e61d000a8a46af6e7cc2ea051f3f56b9a3ff19afdc7e89424190040)
pub type GlVelocityWriter =
    std::sync::Arc<dyn Fn(GlVelocityContext, crate::OpaqueHostValue) -> () + Send + Sync + 'static>;
