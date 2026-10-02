// @generated from upstream/packages/types/src/GlContextLoss.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, Signal};

// Source: upstream/packages/types/src/GlContextLoss.ts:4 (sha256:9451346ef1929d148aaca5590727f86ae727dbc235328d573e2dd88d5808bf86)
#[derive(Clone)]
pub struct GlContextLossSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_gl_context_lost: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>,
    >,
    pub on_gl_context_restored: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for GlContextLossSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
