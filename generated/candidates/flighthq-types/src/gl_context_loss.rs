// @generated from upstream/packages/types/src/GlContextLoss.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::GlRenderState;

// Source: upstream/packages/types/src/GlContextLoss.ts:2 (sha256:b2fb6a5b0a8d1a5b9676900f192c81d468570c3bc80cf710456d8644d925220e)
#[derive(Clone)]
pub struct GlContextLossSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_gl_context_lost:
        Vec<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>>,
    pub on_gl_context_restored:
        Vec<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>>,
}
impl PartialEq for GlContextLossSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
