// @generated from upstream/packages/types/src/NodeRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BatchFormat, NodeAny, RenderProxy, RenderState, RendererData};

// Source: upstream/packages/types/src/NodeRenderer.ts:7 (sha256:82b3ce28a910652826bca008dd3fae01c9cb30be8ccf26ad2bc0a1ae8e0c5344)
#[derive(Clone)]
pub struct NodeRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<BatchFormat>,
    pub create_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, NodeAny) -> Option<RendererData> + Send + 'static>,
        >,
    >,
    pub destroy_data: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, RendererData) -> () + Send + 'static>>,
        >,
    >,
    pub is_dirty: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderState, NodeAny, Option<RendererData>) -> bool + Send + 'static>,
            >,
        >,
    >,
    pub submit: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy) -> () + Send + 'static>>,
    >,
}
impl PartialEq for NodeRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
