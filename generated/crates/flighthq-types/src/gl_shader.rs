// @generated from upstream/packages/types/src/GlShader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlContext, GlRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/GlShader.ts:5 (sha256:7ee3c216fb85c36b5d06270fd12c37b36e76f6e792ea82b8f18c0956b1c67457)
#[derive(Clone)]
pub struct GlShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(GlContext, GlRenderState, RenderProxy2D) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for GlShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
