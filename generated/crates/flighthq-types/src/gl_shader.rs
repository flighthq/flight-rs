// @generated from upstream/packages/types/src/GlShader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/GlShader.ts:4 (sha256:bc27eb05329e99a8a1978d826475e2001bc2b76528fa31104b1e115a3c6f1761)
#[derive(Clone)]
pub struct GlShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(crate::OpaqueHostValue, GlRenderState, RenderProxy2D) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for GlShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
