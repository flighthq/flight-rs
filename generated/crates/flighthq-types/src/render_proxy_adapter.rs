// @generated from upstream/packages/types/src/RenderProxyAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderProxy2D, RenderState, Renderable};

// Source: upstream/packages/types/src/RenderProxyAdapter.ts:5 (sha256:4026ea09e96b83cb35d71bc652ac9b7cf776ea01c3e354ff4656db3b4efbd9fd)
#[derive(Clone)]
pub struct RenderProxyAdapter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub adapt: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> Option<bool> + Send + 'static>,
        >,
    >,
}
impl PartialEq for RenderProxyAdapter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
